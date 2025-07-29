//! Manages listeners in a event groups.

use std::{collections::HashMap, sync::RwLock};

use crate::shell_lib::compile::job;


pub struct EventGroup {
    /// The name of the event group; for debugging.
    pub desc: job::EventGroup,

    /// The list of jobs that are listening to this event group.
    /// Each job's listener triggers a job sequence.
    listeners: RwLock<HashMap<job::JobRef, job::EventHandler>>,
}

impl EventGroup {
    pub fn new(desc: job::EventGroup) -> Self {
        Self {
            desc,
            listeners: RwLock::new(HashMap::new()),
        }
    }

    /// Add a job sequence to the event group.
    pub fn add_listener(&self, job_ref: job::JobRef, handler: job::EventHandler) {
        self.listeners
            .write().expect("failed locking event group")
            .insert(job_ref, handler);
    }

    /// Remove a job sequence from the event group.
    pub fn remove_listener(&self, job_ref: &job::JobRef) {
        self.listeners
            .write().expect("failed locking event group")
            .remove(job_ref);
    }

    /// Run the function over each listener in the event group.
    pub fn listener_map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(job::JobRef, &job::EventHandler) -> Result<(), String>,
    {
        // The read lock means that multiple of these can run at the same time,
        // so the 'f' can have a longer execution time if needed.
        // Write operations are limited to simple add/remove operations, which are fast.
        let listeners = self.listeners
            .read().expect("failed locking event group");

        let mut errs = Vec::new();
        for listener in listeners.iter() {
            if let Err(s) = f(*listener.0, listener.1) {
                errs.push(s);
            }
        }
        errs
    }
}

/// The handler for managing events passed through the scheduler.
/// The event groups are defined by the script and, thus, are static.
pub struct EventBus {
    /// The list of event groups.
    group_names: HashMap<String, usize>,
    event_groups: Vec<EventGroup>,
}

impl EventBus {
    pub fn new(groups: Vec<job::EventGroup>) -> Self {
        let mut group_names = HashMap::new();
        let mut event_groups = Vec::with_capacity(groups.len());
        for desc in groups {
            let name = &desc.name;
            if group_names.contains_key(name) {
                panic!("Duplicate group names registered ({name})");
            }
            group_names.insert(name.clone(), event_groups.len());
            event_groups.push(EventGroup::new(desc));
        }
        Self { group_names, event_groups }
    }

    pub fn add_listener(&self, event_ref: &job::EventRef, job_ref: job::JobRef, handler: job::EventHandler) {
        let idx = self.group_names.get(event_ref).expect("unknown event group name");
        self.event_groups
            .get(*idx).expect("event reference out of bounds")
            .add_listener(job_ref, handler);
    }

    pub fn remove_listener(&self, event_ref: &job::EventRef, job_ref: &job::JobRef) {
        let idx = self.group_names.get(event_ref).expect("unknown event group name");
        self.event_groups
            .get(*idx).expect("event reference out of bounds")
            .remove_listener(job_ref);
    }

    pub fn listener_map<F>(&self, event_ref: &job::EventRef, f: F) -> Vec<String>
    where
        F: Fn(job::JobRef, &job::EventHandler) -> Result<(), String>,
    {
        let idx = self.group_names.get(event_ref).expect("unknown event group name");
        self.event_groups
            .get(*idx).expect("event reference out of bounds")
            .listener_map(f)
    }
}
