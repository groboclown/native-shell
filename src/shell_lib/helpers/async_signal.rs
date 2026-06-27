//SPDX:MIT

//! A simple "is it ready" asychronous publish + consume pattern.

use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

/// Create the notice / wait pair for a signal.
/// This is designed to annouce within a system about a state change.  The state
/// change should happen in its own lock.
/// Care must be taken to have the notice happen only *after* the state has changed
/// and its lock released.  This can lead to situations of too many wakeups and
/// wasted cycles, but the alternative has the waiters miss announcements and
/// incorrectly wait when no later change happens.
pub fn signal() -> (SignalNotice, SignalWait) {
    let state = State::new();
    let notice = SignalNotice {
        inner: state.clone(),
    };
    let wait = SignalWait { inner: state };
    (notice, wait)
}

/// Sends notices to the signal.
/// This can be cloned for many things to send signals.
#[derive(Clone)]
pub struct SignalNotice {
    inner: Arc<State>,
}

impl SignalNotice {
    /// Send a notice to all active waiters.
    pub fn notify(&self) {
        self.inner.notify();
    }
}

/// Waits for signals.
pub struct SignalWait {
    inner: Arc<State>,
}

impl SignalWait {
    /// Wait for a notify, or a timeout.  Returns true if it was notified.
    /// In cases of errors, this assumes that a notification happened, so that
    /// systems that use this signal can then use standard checks to extract state.
    pub fn wait_for(&self, timeout: Duration) -> bool {
        self.inner.wait_for(timeout)
    }
}

struct State {
    g: Mutex<()>,
    c: Condvar,
}

impl State {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            c: Condvar::new(),
            g: Mutex::new(()),
        })
    }

    /// Send the notification.  This will wake up all listeners.
    pub fn notify(&self) {
        self.c.notify_all()
    }

    /// Wait for a notify, or a timeout.  Returns true if it was notified.
    /// In cases of errors, this assumes that a notification happened, so that
    /// systems that use this signal can then use standard checks to extract state.
    pub fn wait_for(&self, timeout: Duration) -> bool {
        match self.c.wait_timeout(
            match self.g.lock() {
                Ok(g) => g,
                Err(_) => return true,
            },
            timeout,
        ) {
            Ok((_, res)) => !res.timed_out(),
            Err(_) => true,
        }
    }
}
