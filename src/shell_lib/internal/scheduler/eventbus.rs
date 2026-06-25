//! Manages listeners in a event groups.

use std::{collections::HashMap, sync::RwLock};

use crate::shell_lib::structure::{event, job, thread};

/// The primary event bus.
pub struct EventBus {
    store: event::EventStore,
    job_wait: RwLock<ExitListenersState<job::JobRef>>,
    thread_wait: RwLock<ExitListenersState<thread::ThreadRef>>,
    event_listeners: RwLock<EventListenersState>,
}

impl EventBus {
    pub fn new(store: event::EventStore) -> Self {
        Self {
            store,
            job_wait: ExitListenersState::new(),
            thread_wait: ExactSizeIterator::new(),
            event_listeners: EventListenersState::new(),
        }
    }

    pub fn add_listener_for_job(
        &mut self,
        for_job: buildup::job::JobRef,
        listener: buildup::job::JobThreadRef,
    ) {
        self.job_wait
            .get_mut()
            .expect("job listener has poisoned lock")
            .add_listener(for_job, listener);
    }

    pub fn add_listener_for_thread(
        &mut self,
        for_thread: buildup::job::JobThreadRef,
        listener: buildup::job::JobThreadRef,
    ) {
        self.thread_wait
            .get_mut()
            .expect("job thread listener has poisoned lock")
            .add_listener(for_thread, listener);
    }

    pub fn add_event_listener(
        &mut self,
        job: buildup::job::JobRef,
        event: buildup::event::EventRef,
    ) {
        self.event_listeners
            .get_mut()
            .expect("event listener has poisoned lock")
            .add_listener(event, job);
    }

    pub fn add_event_listeners(
        &mut self,
        job: buildup::job::JobRef,
        events: Vec<buildup::event::EventRef>,
    ) {
        events.iter().for_each(|e| self.add_event_listener(job, *e));
    }

    /// Tells the bus that the job has exited, and returns (and clears) the list
    /// of threads listening for that job to end.
    pub fn on_job_exit(&mut self, job: buildup::job::JobRef) -> Vec<buildup::job::JobThreadRef> {
        self.job_wait
            .get_mut()
            .expect("job listener has poisoned lock")
            .on_exit(job)
            .unwrap_or(Vec::new())
    }

    /// Tells the bus that the thread has exited, and returns (and clears) the list
    /// of threads listening for that thread to end.
    pub fn on_thread_exit(
        &mut self,
        thread: buildup::job::JobThreadRef,
    ) -> Vec<buildup::job::JobThreadRef> {
        self.thread_wait
            .get_mut()
            .expect("job thread listener has poisoned lock")
            .on_exit(thread)
            .unwrap_or(Vec::new())
    }

    /// Get the list of jobs listening for the event.
    pub fn on_event(&mut self, event: buildup::event::EventRef) -> &Vec<buildup::job::JobRef> {
        self.event_listeners
            .get_mut()
            .expect("event listener has poisoned lock")
            .listeners(event)
            .unwrap_or(&Vec::new())
    }
}

impl buildup::create::EventRegistrar for EventBus {
    fn add_event(&mut self, name: &str) -> buildup::event::EventDesc {
        self.registrar.add_event(name)
    }
}

/// All the job or job threads waiting on a job to complete.
struct ExitListenersState<T> {
    /// Map of the job that, when completed, will trigger these threads to awaken.
    waiting_on: HashMap<T, Vec<buildup::job::JobThreadRef>>,
}

impl<T> ExitListenersState<T> {
    pub fn new() -> RwLock<Self> {
        RwLock::new(Self {
            waiting_on: HashMap::new(),
        })
    }

    pub fn add_listener(&mut self, on: T, listener: buildup::job::JobThreadRef) {
        // For debugging,
        #[cfg(test)]
        self.ensure_not_listening(&on, listener);

        match self.waiting_on.get_mut(&on) {
            Some(v) => v.push(listener),
            None => {
                let mut v = Vec::new();
                v.push(listener);
                self.waiting_on.insert(on, v);
            }
        };
    }

    pub fn on_exit(&mut self, exited: T) -> Option<Vec<buildup::job::JobThreadRef>> {
        self.waiting_on.remove(&exited)
    }

    // Ensure that the listened-on does not already have the listener.
    fn ensure_not_listening(&self, on: &T, listener: buildup::job::JobThreadRef) {
        if let Some(existing) = self.waiting_on.get(on) {
            for e in existing {
                if e == listener {
                    panic!("BUG double-registered listener {} for {}", listener, on);
                }
            }
        }
    }
}

/// Jobs that listen to specific events.
/// This matches very closely to the ExitListenersState, but, because they have
/// different usage characteristics, they are separate structures to keep from type confusion bugs.
struct EventListenersState {
    /// Map of the job that, when completed, will trigger these threads to awaken.
    listeners: HashMap<buildup::event::EventRef, Vec<buildup::job::JobRef>>,
}

impl EventListenersState {
    pub fn new() -> RwLock<Self> {
        RwLock::new(Self {
            listeners: HashMap::new(),
        })
    }

    pub fn add_listener(&mut self, on: buildup::event::EventRef, listener: buildup::job::JobRef) {
        // For debugging,
        #[cfg(test)]
        self.ensure_not_listening(on, listener);

        match self.listeners.get_mut(&on) {
            Some(v) => v.push(listener),
            None => {
                let mut v = Vec::new();
                v.push(listener);
                self.listeners.insert(on, v);
            }
        };
    }

    pub fn listeners(&self, on: buildup::event::EventRef) -> Option<&Vec<buildup::job::JobRef>> {
        self.listeners.get(&on)
    }

    // Ensure that the listened-on does not already have the listener.
    fn ensure_not_listening(
        &self,
        on: buildup::event::EventRef,
        listener: buildup::job::JobThreadRef,
    ) {
        if let Some(existing) = self.listeners.get(&on) {
            for e in existing {
                if e == listener {
                    panic!(
                        "BUG double-registered listener {} for event {}",
                        listener, on
                    );
                }
            }
        }
    }
}
