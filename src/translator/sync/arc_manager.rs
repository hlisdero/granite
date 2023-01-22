//! Central structure to keep track of the `std::sync::Arc` in the code.
//!
//! It is mainly used in conjunction with the `MutexManager` to keep track of the mutexes
//! when they are wrapped around a `std::sync::Arc`.

use super::{is_mutex, is_reference_to_arc_with_mutex};
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::{
    naming::arc::function_transition_label, utils::extract_first_argument_for_function_call,
};
use netcrab::petri_net::{PetriNet, PlaceRef};

#[derive(Default)]
pub struct ArcManager {
    arc_counter: usize,
    deref_counter: usize,
    clone_counter: usize,
}

impl ArcManager {
    /// Returns a new empty `ArcManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a call to `std::sync::Arc::<T>::new` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_new(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        net: &mut PetriNet,
    ) {
        let index = self.arc_counter;
        let transition_label = &function_transition_label("std::sync::Arc::<T>::new", index);
        self.arc_counter += 1;
        call_foreign_function(start_place, end_place, None, transition_label, net);
    }

    /// Translates a call to `std::ops::Deref::deref` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_deref(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) {
        let index = self.deref_counter;
        let transition_label = &function_transition_label("std::ops::Deref::deref", index);
        self.deref_counter += 1;
        call_foreign_function(start_place, end_place, cleanup_place, transition_label, net);
    }

    /// Translates a call to `std::clone::Clone::clone` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_clone(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) {
        let index = self.clone_counter;
        let transition_label = &function_transition_label("std::clone::Clone::clone", index);
        self.clone_counter += 1;
        call_foreign_function(start_place, end_place, cleanup_place, transition_label, net);
    }

    /// Translates the side effects for `std::sync::Arc::<T>::new` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_new<'tcx>(
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        memory: &mut Memory<'tcx>,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        let first_argument = extract_first_argument_for_function_call(args);
        if is_mutex(&first_argument, caller_function_def_id, tcx) {
            memory.link_place_to_same_mutex(return_value, first_argument);
        }
    }

    /// Translates the side effects for `std::ops::Deref::deref` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_deref<'tcx>(
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        memory: &mut Memory<'tcx>,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        let first_argument = extract_first_argument_for_function_call(args);
        if is_reference_to_arc_with_mutex(&first_argument, caller_function_def_id, tcx) {
            memory.link_place_to_same_mutex(return_value, first_argument);
        }
    }

    /// Translates the side effects for `std::clone::Clone::clone` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_clone<'tcx>(
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        memory: &mut Memory<'tcx>,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        let first_argument = extract_first_argument_for_function_call(args);
        if is_reference_to_arc_with_mutex(&first_argument, caller_function_def_id, tcx) {
            memory.link_place_to_same_mutex(return_value, first_argument);
        }
    }
}
