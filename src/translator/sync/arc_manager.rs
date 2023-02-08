//! Central structure to keep track of the `std::sync::Arc` in the code.
//!
//! It is mainly used in conjunction with the `MutexManager` and the `CondvarManager`
//! to keep track of the mutexes and condvars when they are wrapped around a `std::sync::Arc`.

use crate::naming::arc::{clone_transition_labels, deref_transition_labels, new_transition_labels};
use crate::petri_net_interface::PetriNet;
use crate::translator::function_call::FunctionPlaces;
use crate::translator::special_function::call_foreign_function;

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
        function_call_places: &FunctionPlaces,
        net: &mut PetriNet,
    ) {
        let index = self.arc_counter;
        self.arc_counter += 1;
        call_foreign_function(function_call_places, &new_transition_labels(index), net);
    }

    /// Translates a call to `std::ops::Deref::deref` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_deref(
        &mut self,
        function_call_places: &FunctionPlaces,
        net: &mut PetriNet,
    ) {
        let index = self.deref_counter;
        self.deref_counter += 1;
        call_foreign_function(function_call_places, &deref_transition_labels(index), net);
    }

    /// Translates a call to `std::clone::Clone::clone` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_clone(
        &mut self,
        function_call_places: &FunctionPlaces,
        net: &mut PetriNet,
    ) {
        let index = self.clone_counter;
        self.clone_counter += 1;
        call_foreign_function(function_call_places, &clone_transition_labels(index), net);
    }
}
