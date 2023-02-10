//! Representation of the local memory of the function.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a mutex.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a lock guard.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with thread references. This tracks which variables contain a join handle.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with condvar references. This tracks which variables contain a condition variable.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

use crate::translator::sync::{CondvarRef, MutexRef, ThreadRef};
use log::debug;
use std::collections::HashMap;

#[derive(Default)]
pub struct Memory<'tcx> {
    places_linked_to_mutexes: HashMap<rustc_middle::mir::Place<'tcx>, MutexRef>,
    places_linked_to_lock_guards: HashMap<rustc_middle::mir::Place<'tcx>, MutexRef>,
    places_linked_to_join_handles: HashMap<rustc_middle::mir::Place<'tcx>, ThreadRef>,
    places_linked_to_condvars: HashMap<rustc_middle::mir::Place<'tcx>, CondvarRef>,
}

/// An auxiliary type for passing mutex memory entries from one function to the other.
pub type MutexEntries = Vec<MutexRef>;
/// An auxiliary type for passing condvar memory entries from one function to the other.
pub type CondvarEntries = Vec<CondvarRef>;

impl<'tcx> Memory<'tcx> {
    /// Creates a new memory with empty mappings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a place as containing a mutex.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a mutex, then the function panics.
    pub fn link_place_to_mutex(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        mutex_ref: MutexRef,
    ) {
        let Some(old_mutex_ref) = self.places_linked_to_mutexes.insert(place, mutex_ref) else {
            return;
        };
        if mutex_ref == old_mutex_ref {
            debug!("PLACE {place:?} LINKED AGAIN TO SAME MUTEX");
        } else {
            debug!("PLACE {place:?} LINKED TO A DIFFERENT MUTEX");
        }
    }

    /// Marks a place as containing a lock guard for a mutex.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a lock guard, then the function panics.
    pub fn link_place_to_lock_guard(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        mutex_ref: MutexRef,
    ) {
        let Some(old_mutex_ref) = self.places_linked_to_lock_guards.insert(place, mutex_ref) else {
            return;
        };
        if mutex_ref == old_mutex_ref {
            debug!("PLACE {place:?} LINKED AGAIN TO SAME LOCK GUARD");
        } else {
            debug!("PLACE {place:?} LINKED TO A DIFFERENT LOCK GUARD");
        }
    }

    /// Marks a place as containing a join handle for a thread.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a join handle, then the function panics.
    pub fn link_place_to_join_handle(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        thread_ref: ThreadRef,
    ) {
        let Some(old_thread_ref) = self.places_linked_to_join_handles.insert(place, thread_ref) else {
            return;
        };
        if thread_ref == old_thread_ref {
            debug!("PLACE {place:?} LINKED AGAIN TO SAME JOIN HANDLE");
        } else {
            debug!("PLACE {place:?} LINKED TO A DIFFERENT JOIN HANDLE");
        }
    }

    /// Marks a place as containing a condition variable.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a condition variable, then the function panics.
    pub fn link_place_to_condvar(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        condvar_ref: CondvarRef,
    ) {
        let Some(old_condvar_ref) = self.places_linked_to_condvars.insert(place, condvar_ref) else {
            return;
        };
        if condvar_ref == old_condvar_ref {
            debug!("PLACE {place:?} LINKED AGAIN TO SAME CONDITION VARIABLE");
        } else {
            debug!("PLACE {place:?} LINKED TO A DIFFERENT CONDITION VARIABLE");
        }
    }

    /// Returns the mutex reference linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a mutex, then the function panics.
    pub fn get_linked_mutex(&self, place: &rustc_middle::mir::Place<'tcx>) -> &MutexRef {
        let Some(mutex) = self.places_linked_to_mutexes.get(place) else {
            panic!("BUG: The place {place:?} should be linked to a mutex");
        };
        mutex
    }

    /// Returns the mutex for the lock guard linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a lock guard, then the function panics.
    pub fn get_linked_lock_guard(&self, place: &rustc_middle::mir::Place<'tcx>) -> &MutexRef {
        let Some(lock_guard) = self.places_linked_to_lock_guards.get(place) else {
            panic!("BUG: The place {place:?} should be linked to a lock guard")
        };
        lock_guard
    }

    /// Returns the thread reference for the join handle linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a join handle, then the function panics.
    pub fn get_linked_join_handle(&self, place: &rustc_middle::mir::Place<'tcx>) -> &ThreadRef {
        let Some(join_handle) = self.places_linked_to_join_handles.get(place) else {
            panic!("BUG: The place {place:?} should be linked to a join handle")
        };
        join_handle
    }

    /// Returns the condvar reference for the condition variable linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a condition variable, then the function panics.
    pub fn get_linked_condvar(&self, place: &rustc_middle::mir::Place<'tcx>) -> &CondvarRef {
        let Some(condvar) = self.places_linked_to_condvars.get(place) else {
            panic!("BUG: The place {place:?} should be linked to a condition variable")
        };
        condvar
    }

    /// Checks whether the place is linked to a lock guard.
    pub fn is_linked_to_lock_guard(&self, place: rustc_middle::mir::Place<'tcx>) -> bool {
        self.places_linked_to_lock_guards.contains_key(&place)
    }

    /// Links a place to the mutex linked to another place.
    /// After this operation both places point to the same mutex, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a mutex, then the function panics.
    /// If the place linked to a mutex is not linked to a mutex, then the function panics.
    pub fn link_place_to_same_mutex(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_mutex: rustc_middle::mir::Place<'tcx>,
    ) {
        let mutex_ref = self.get_linked_mutex(&place_linked_to_mutex);
        self.link_place_to_mutex(place_to_be_linked, *mutex_ref);
        debug!("SAME MUTEX: {place_to_be_linked:?} = {place_linked_to_mutex:?}");
    }

    /// Links a place to the lock guard linked to another place.
    /// After this operation both places point to the same lock guard, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a lock guard, then the function panics.
    /// If the place linked to a lock guard is not linked to a lock guard, then the function panics.
    pub fn link_place_to_same_lock_guard(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_lock_guard: rustc_middle::mir::Place<'tcx>,
    ) {
        let mutex_ref = self.get_linked_lock_guard(&place_linked_to_lock_guard);
        self.link_place_to_lock_guard(place_to_be_linked, *mutex_ref);
        debug!("SAME LOCK GUARD: {place_to_be_linked:?} = {place_linked_to_lock_guard:?}");
    }

    /// Links a place to the join handle linked to another place.
    /// After this operation both places point to the same join handle, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a join handle, then the function panics.
    /// If the place linked to a join handle is not linked to a join handle, then the function panics.
    pub fn link_place_to_same_join_handle(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_join_handle: rustc_middle::mir::Place<'tcx>,
    ) {
        let thread_ref = self.get_linked_join_handle(&place_linked_to_join_handle);
        self.link_place_to_join_handle(place_to_be_linked, *thread_ref);
        debug!("SAME JOIN HANDLE: {place_to_be_linked:?} = {place_linked_to_join_handle:?}");
    }

    /// Links a place to the condition variable linked to another place.
    /// After this operation both places point to the same condition variable, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a condition variable, then the function panics.
    /// If the place linked to a condvar is not linked to a condition variable, then the function panics.
    pub fn link_place_to_same_condvar(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_condvar: rustc_middle::mir::Place<'tcx>,
    ) {
        let condvar_ref = self.get_linked_condvar(&place_linked_to_condvar);
        self.link_place_to_condvar(place_to_be_linked, *condvar_ref);
        debug!("SAME CONDVAR: {place_to_be_linked:?} = {place_linked_to_condvar:?}");
    }

    /// Finds all the mutexes linked to the given place.
    /// It takes into consideration that the place may have several fields (a subtype of projections).
    /// <https://rustc-dev-guide.rust-lang.org/mir/index.html?highlight=Projections#mir-data-types>
    /// Returns a vector of places which share the same local.
    pub fn find_mutexes_linked_to_place(
        &self,
        place: rustc_middle::mir::Place<'tcx>,
    ) -> MutexEntries {
        let mut result: MutexEntries = Vec::new();
        for mutex_place in self.places_linked_to_mutexes.keys() {
            if mutex_place.local == place.local {
                let mutex_ref = self.get_linked_mutex(mutex_place);
                result.push(*mutex_ref);
                debug!("FOUND MUTEX IN PLACE {mutex_place:?}");
            }
        }
        result
    }

    /// Finds all the condvars linked to the given place.
    /// It takes into consideration that the place may have several fields (a subtype of projections).
    /// <https://rustc-dev-guide.rust-lang.org/mir/index.html?highlight=Projections#mir-data-types>
    /// Returns a vector of places which share the same local.
    pub fn find_condvars_linked_to_place(
        &self,
        place: rustc_middle::mir::Place<'tcx>,
    ) -> CondvarEntries {
        let mut result: CondvarEntries = Vec::new();
        for condvar_place in self.places_linked_to_condvars.keys() {
            if condvar_place.local == place.local {
                let condvar_ref = self.get_linked_condvar(condvar_place);
                result.push(*condvar_ref);
                debug!("FOUND CONDVAR IN PLACE {condvar_place:?}");
            }
        }
        result
    }
}
