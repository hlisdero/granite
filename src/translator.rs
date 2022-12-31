//! Submodule for the main translation logic.
mod error_handling;
mod function;
mod local;
mod mir_visitor;
mod mutex;
mod mutex_manager;
mod naming;
mod special_function;
mod virtual_memory;

use crate::stack::Stack;
use crate::translator::error_handling::EMPTY_CALL_STACK;
use crate::translator::function::Function;
use crate::translator::mutex_manager::MutexManager;
use crate::translator::naming::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use netcrab::petri_net::{PetriNet, PlaceRef};
use rustc_middle::mir::visit::Visitor;

pub struct Translator<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    err_str: Option<&'static str>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<Function>,
    mutex_manager: MutexManager,
}

impl<'tcx> Translator<'tcx> {
    /// Creates a new `Translator`.
    /// Requires a global typing context `rustc_middle::ty::TyCtxt`, the main data structure of the compiler.
    /// The initial Petri net contains three places representing the program start state,
    /// the program end state and the abnormal end state after `panic!()`.
    pub fn new(tcx: rustc_middle::ty::TyCtxt<'tcx>) -> Self {
        let mut net = PetriNet::new();
        let program_panic = net.add_place(PROGRAM_PANIC);
        let program_end = net.add_place(PROGRAM_END);
        let program_start = net.add_place(PROGRAM_START);
        net.add_token(&program_start, 1).expect(
            "BUG: Adding initial token to empty PROGRAM_START place should not cause an overflow",
        );

        Self {
            tcx,
            err_str: None,
            net,
            program_start,
            program_end,
            program_panic,
            call_stack: Stack::new(),
            mutex_manager: MutexManager::new(),
        }
    }

    /// Get the result of the translation, i.e. the Petri net.
    /// The ownership is transferred to the caller.
    ///
    /// # Errors
    ///
    /// If the translation failed, then an error is returned.
    pub fn get_result(&mut self) -> Result<PetriNet, &'static str> {
        match self.err_str {
            Some(err_str) => Err(err_str),
            // We do not want to panic here. The user should call `has_err()` first.
            None => Ok(std::mem::take(&mut self.net)),
        }
    }

    /// Set the error string explicitly.
    /// This is only used internally during the translation process.
    fn set_err_str(&mut self, err_str: &'static str) {
        self.err_str = Some(err_str);
    }

    /// Translate the source code to a Petri net.
    ///
    /// # Errors
    ///
    /// If the translation fails due to a recoverable error, then an error message is set.
    ///
    /// # Panics
    ///
    /// If the translation fails due to an unsupported feature present in the code, then the function panics.
    pub fn run(&mut self) {
        let Some((main_function_id, _)) = self.tcx.entry_fn(()) else {
            self.set_err_str("No main function found in the given source code");
            return;
        };
        self.push_function_to_call_stack(
            main_function_id,
            self.program_start.clone(),
            self.program_end.clone(),
        );
        self.translate_top_call_stack();
    }

    /// Pushes a new function frame to the call stack.
    /// The call stack is the main method to pass information between methods.
    fn push_function_to_call_stack(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
    ) {
        let function = Function::new(
            function_def_id,
            start_place,
            end_place,
            &mut self.mutex_manager,
            &mut self.net,
            &mut self.tcx,
        );
        self.call_stack.push(function);
    }

    /// Main translation loop.
    /// Translate the function from the top of the call stack.
    /// Inside the MIR Visitor, when a call to another function happens this method will be called again
    /// to jump to the new function. Eventually a "leaf function" will be reached, the functions will exit and the
    /// elements from the stack will be popped in order.
    fn translate_top_call_stack(&mut self) {
        let function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        // Translate the function to a Petri net from the MIR representation.
        let body = self.tcx.optimized_mir(function.def_id);
        // Visit the MIR body of the function using the methods of `rustc_middle::mir::visit::Visitor`.
        // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
        self.visit_body(body);
        // Finished processing this function.
        self.call_stack.pop();
    }

    /// Extracts the function call ID from the `rustc_middle::mir::Operand`.
    ///
    /// First obtains the type (`rustc_middle::ty::Ty`) of the operand for every possible case.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Operand.html>
    ///
    /// Then checks that the type is a function definition (`rustc_middle::ty::TyKind::FnDef`)
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/enum.TyKind.html>
    ///
    /// This method is used to know which function will be called as part of the `Call` MIR Terminator.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/enum.TerminatorKind.html#variant.Call>
    fn extract_def_id_of_called_function_from_operand(
        operand: &rustc_middle::mir::Operand<'tcx>,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_hir::def_id::DefId {
        let function_type = match operand {
            rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
                // Find the type through the local declarations of the caller function.
                // The place should be declared there and we can query its type.
                let body = tcx.optimized_mir(caller_function_def_id);
                let place_ty = place.ty(&body.local_decls, tcx);
                place_ty.ty
            }
            rustc_middle::mir::Operand::Constant(constant) => constant.ty(),
        };
        match function_type.kind() {
            rustc_middle::ty::TyKind::FnPtr(_) => {
                unimplemented!(
                    "TyKind::FnPtr not implemented yet. Function pointers are present in the MIR"
                );
            }
            rustc_middle::ty::TyKind::FnDef(def_id, _) => *def_id,
            _ => {
                panic!("TyKind::FnDef, a function definition, but got: {function_type:?}")
            }
        }
    }
}
