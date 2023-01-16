//! Representation of a thread to be translated later.
//!
//! The thread involves a translation subprocess that connects
//! to the main Petri net in two places:
//! - The start place i.e, when `std::thread::spawn` is called.
//! - The end place i.e, when `std::thread::JoinHandle::<T>::join` is called.
//!
//! The thread starts after the call to `std::thread::spawn`.
//! The transition that represents this function call produces a second token
//! and places it in the new thread's start place.
//! This token models the execution flow of the new thread.
//!
//! The thread end place is connected to the transition that models
//! the call to `std::thread::JoinHandle::<T>::join`.
//!
//! The Petri net for the new thread "spans" between the two transitions.
//! We cannot connect this net to the net of the spawning thread until the
//! call to `std::thread::JoinHandle::<T>::join` is translated.
//! Therefore we store the basic information we need
//! to translate the thread function and defer the translation.
//! The function executed by the thread is translated to a Petri net just as any other.

use crate::error_handling::handle_err_add_arc;
use crate::naming::thread::{thread_end_place_label, thread_start_place_label};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct ThreadSpan {
    /// The transition from which the thread branches off at the start.
    spawn_transition: TransitionRef,
    /// The definition ID that uniquely identifies the function run by the thread.
    thread_function_def_id: rustc_hir::def_id::DefId,
    /// The transition to which the thread joins in at the end.
    join_transition: Option<TransitionRef>,
    /// An index to identify the thread span.
    index: usize,
}

impl ThreadSpan {
    /// Creates a new thread span without a join transition.
    /// The join transition must be set later.
    pub const fn new(
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        index: usize,
    ) -> Self {
        Self {
            spawn_transition,
            thread_function_def_id,
            join_transition: None,
            index,
        }
    }

    /// Sets the transition that models joining this thread.
    pub fn set_join_transition(&mut self, join_transition: TransitionRef) {
        self.join_transition = Some(join_transition);
    }

    /// Prepares the thread span for translation.
    /// Adds a start and end place for the thread to the Petri net.
    /// Connects the spawn transition to the start place and the end place to the join transition (if available).
    /// Returns a 3-tuple containing the definition ID, the start place and the end place.
    pub fn prepare_for_translation(
        &self,
        net: &mut PetriNet,
    ) -> (rustc_hir::def_id::DefId, PlaceRef, PlaceRef) {
        let thread_start_place = net.add_place(&thread_start_place_label(self.index));
        let thread_end_place = net.add_place(&thread_end_place_label(self.index));

        net.add_arc_transition_place(&self.spawn_transition, &thread_start_place)
            .unwrap_or_else(|_| handle_err_add_arc("spawn transition", "thread start place"));
        if let Some(join_transition) = &self.join_transition {
            net.add_arc_place_transition(&thread_end_place, join_transition)
                .unwrap_or_else(|_| handle_err_add_arc("thread end place", "join transition"));
        }
        (
            self.thread_function_def_id,
            thread_start_place,
            thread_end_place,
        )
    }
}
