use std::{ops::DerefMut, sync::{Arc, RwLock}};

pub enum ExecState<R, E> {
    LockContention,
    Ran(Result<R, E>),
}

pub struct StateGuard<T> {
    guard: Arc<RwLock<T>>,
}

impl<T> StateGuard<T> {
    pub fn new(state: T) -> Self {
        StateGuard{ guard: Arc::new(RwLock::new(state)) }
    }

    pub fn new_double(state: T) -> (Self, Self) {
        let guard = Arc::new(RwLock::new(state));
        (StateGuard{guard: guard.clone()}, StateGuard{guard})
    }

    /// Run the handler with a lock on the mutable state
    pub fn run_mut<R, E>(&self, handler: impl FnOnce(&mut T) -> Result<R, E>) -> ExecState<R, E> {
        let mut active_ref = match self.guard.write() {
            Ok(lock) => lock,
            Err(_) => return ExecState::LockContention,
        };
        let mut state = active_ref.deref_mut();
        ExecState::Ran(handler(&mut state))
    }

    /// Run the handler with a lock on the immutable state
    pub fn run<R, E>(&self, handler: impl FnOnce(&T) -> Result<R, E>) -> ExecState<R, E> {
        let active_ref = match self.guard.read() {
            Ok(lock) => lock,
            Err(_) => return ExecState::LockContention,
        };
        ExecState::Ran(handler(&active_ref))
    }
}

impl<T> Clone for StateGuard<T> {
    fn clone(&self) -> Self {
        StateGuard { guard: self.guard.clone() }
    }
}


/// Special guard that allows for a one time deactivation.
pub struct DeactivateGuard {
    guard: Arc<RwLock<bool>>,
}

impl DeactivateGuard {
    pub fn new() -> Self {
        DeactivateGuard {
            guard: Arc::new(RwLock::new(true)),
        }
    }

    pub fn new_dual() -> (Self, Self) {
        let guard = Arc::new(RwLock::new(true));
        (DeactivateGuard {guard: guard.clone()}, DeactivateGuard {guard})
    }

    /// Deactivate the guard, allowing the state to be cleaned up.
    /// Returns true if the guard was active before deactivation, which allows for thread safe cleanup.
    pub fn deactivate(&self) -> bool {
        let mut active_ref = self.guard.write()
            .expect("Failed to lock the deactivate guard");
        let ret = *active_ref;
        *(active_ref.deref_mut()) = false;
        ret
    }

    /// Check if the guard is deactivated.
    pub fn is_active(&self) -> bool {
        let active_ref = self.guard.read()
            .expect("Failed to lock the deactivate guard");
        *active_ref
    }
}
