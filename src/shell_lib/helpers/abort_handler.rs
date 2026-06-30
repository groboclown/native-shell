//! Methods for handling the abort calls into modules.

use std::{ops::DerefMut, sync::RwLock};

use crate::shell_lib::structure::ScriptExit;

/// Manages the run state of a module.
/// Allows the module to handle abort cleaner.
pub struct RunState<T> {
    active: RwLock<Option<T>>,
}

pub type ActionHandler<T, R> = fn(&mut T) -> Result<R, ScriptExit>;

impl<T> RunState<T> {
    pub fn new() -> Self {
        RunState {
            active: RwLock::new(None),
        }
    }

    /// Mark the module as active.
    pub fn start(&self, state: T) {
        let mut active_ref = self
            .active
            .write()
            .expect("Failed to lock the abort state mutex");
        active_ref.deref_mut().replace(state);
    }

    /// Check if the module is active.
    pub fn is_active(&self) -> bool {
        let active_ref = self
            .active
            .read()
            .expect("Failed to lock the abort state mutex");
        active_ref.as_ref().is_some()
    }

    /// Mark the module as stopped.
    /// If the module is not active, it will not run the handler.
    /// This will lock the state while the handler runs.
    pub fn on_stop(&self, handler: ActionHandler<T, ()>) -> Result<(), ScriptExit> {
        let mut active_ref = self.active.write().map_err(|e| format!("{}", e))?;
        {
            let state = active_ref.deref_mut();
            if (*state).is_some() {
                let mut s = (*state).take().unwrap();
                return handler(&mut s);
            }
        }
        Ok(())
    }
}
