//! Central structure to keep track of the mutexes in the code.
//!
//! The `MutexManager` stores the mutexes discovered so far in the code.
//! It also performs the translation for each mutex function.

use crate::translator::naming::mutex_function_transition_label;
use crate::translator::place_to_local;
use crate::translator::special_function::foreign_function_call;
use crate::translator::sync::extract_self_reference_from_arguments_to_function_call;
use crate::translator::sync::mutex::Mutex;
use crate::translator::sync::Memory;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

#[derive(Default)]
pub struct MutexManager {
    mutexes: Vec<Mutex>,
    lock_counter: usize,
}

/// A wrapper type around the indexes to the elements in `Vec<Mutex>`.
#[derive(Clone)]
pub struct MutexRef(usize);

impl MutexManager {
    /// Returns a new empty `MutexManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a mutex function call using the same logic as in `foreign_function_call`.
    /// Returns the transition that represents the function call.
    ///
    /// The naming depends:
    /// - For `std::sync::Mutex::<T>::new` the numbering follows the numbering of the labels of the mutexes.
    /// - For `std::sync::Mutex::<T>::lock` a separate counter is incremented every time that the function is called.
    pub fn translate_function_call(
        &self,
        function_name: &str,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = if function_name == "std::sync::Mutex::<T>::new" {
            self.mutexes.len()
        } else {
            self.lock_counter
        };

        let transition_label = &mutex_function_transition_label(function_name, index);
        foreign_function_call(start_place, end_place, cleanup_place, transition_label, net)
    }

    /// Translates the side effects for the methods of `std::sync::Mutex`, i.e.,
    /// the specific logic of creating a new mutex and locking it.
    /// Receives a reference to the memory of the caller function to update
    /// and retrieve the mapping between local variables and mutex references.
    pub fn translate_function_side_effects(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand],
        return_value: rustc_middle::mir::Place,
        transition_function_call: &TransitionRef,
        net: &mut PetriNet,
        memory: &mut Memory,
    ) {
        if function_name == "std::sync::Mutex::<T>::new" {
            let mutex_ref = self.add_mutex(net);
            let return_value_local = place_to_local(&return_value);
            memory.link_local_to_mutex(return_value_local, mutex_ref);
        } else if function_name == "std::sync::Mutex::<T>::lock" {
            let self_ref = extract_self_reference_from_arguments_to_function_call(args);
            let local_with_mutex = place_to_local(&self_ref);
            let mutex_ref = memory.get_mutex_ref_from_local(local_with_mutex);
            self.add_lock_guard(mutex_ref, transition_function_call, net);
        }
    }

    /// Adds a new mutex and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new mutex.
    pub fn add_mutex(&mut self, net: &mut PetriNet) -> MutexRef {
        let index = self.mutexes.len();
        self.mutexes.push(Mutex::new(index, net));
        MutexRef(index)
    }

    /// Adds a lock guard to the mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    ///
    /// # Panics
    ///
    /// If the mutex reference is invalid, then the function panics.
    pub fn add_lock_guard(
        &mut self,
        mutex_ref: &MutexRef,
        transition_lock: &TransitionRef,
        net: &mut PetriNet,
    ) {
        let mutex = self
            .mutexes
            .get(mutex_ref.0)
            .expect("BUG: The mutex reference should be a valid index for the vector of mutexes");
        mutex.add_lock_guard(transition_lock, net);
        self.lock_counter += 1;
    }
}
