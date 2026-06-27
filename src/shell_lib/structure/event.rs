//SPDX:MIT

//! Inter-job event system.
//!
//! The event system as described in this structure only allows for inter-communication
//! from a thread perspective, where a thread can send or wait on events.  For jobs,
//! their interaction with the events lives external to this structure.

use std::{collections::HashMap, fmt::Display};

/// Events are sent as names from the modules.
/// They pass through the event registrar to turn into ids for faster internal checking.
pub type EventRef = usize;

pub type SignalCode = i32;

#[derive(Debug, Clone, PartialEq)]
pub struct EventDesc {
    pub e_ref: EventRef,
    pub name: String,
    pub kind: EventKind,
}

impl EventDesc {
    pub fn new(e_ref: EventRef, name: String, kind: EventKind) -> Self {
        Self { e_ref, name, kind }
    }
}

impl Display for EventDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Event:")?;
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventPayload {
    /// An event with a string payload.
    Message(String),

    /// An event with an integer payload.
    Signal(SignalCode),
}

impl EventPayload {
    pub fn kind(&self) -> EventKind {
        match self {
            EventPayload::Message(_) => EventKind::Message,
            EventPayload::Signal(_) => EventKind::Signal,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    Message,
    Signal,
}

impl Display for EventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => {
                f.write_str("EventPayload::Message(")?;
                m.fmt(f)?;
                f.write_str(")")
            }
            Self::Signal(s) => {
                f.write_str("EventPayload::Signal(")?;
                s.fmt(f)?;
                f.write_str(")")
            }
        }
    }
}

pub type Event = (EventRef, EventPayload);

/// Build-up stage for allocating event names.
/// The event registration, like all setup stage structures, does not allow for asynchronous construction.
pub struct EventRegistrar {
    by_name: HashMap<String, EventRef>,
    by_ref: Vec<EventDesc>,
}

impl EventRegistrar {
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            by_ref: Vec::new(),
        }
    }

    /// Add a named event to the registration.  It's safe to register the same
    /// name multiple times; it will return the existing ID.
    pub fn add_event(&mut self, name: &str, kind: &EventKind) -> EventRef {
        let name = name.to_string();
        match self.by_name.get(&name) {
            Some(v) => {
                #[cfg(test)]
                assert_eq!(self.by_ref.get(*v).expect("no ref").kind, *kind);
                *v
            }
            None => {
                let e_ref = self.by_ref.len();
                self.by_name.insert(name.clone(), e_ref);
                self.by_ref.push(EventDesc::new(e_ref, name, kind.clone()));
                e_ref
            }
        }
    }

    /// Describe the event, if registered.
    pub fn describe(&self, e_ref: EventRef) -> Option<&EventDesc> {
        self.by_ref.get(e_ref)
    }

    /// Is the event of the given kind?  If the event isn't registered, this returns false.
    pub fn is_kind(&self, e_ref: EventRef, kind: &EventKind) -> bool {
        self.by_ref.get(e_ref).map_or(false, |d| *kind == d.kind)
    }

    /// Close off registration.
    pub fn close(self) -> EventStore {
        EventStore { store: self.by_ref }
    }
}

/// Read-only store for event references, which helps with debugging.
pub struct EventStore {
    store: Vec<EventDesc>,
}

impl EventStore {
    /// Get the event description for the event ID.
    pub fn get(&self, e_ref: EventRef) -> Option<&EventDesc> {
        self.store.get(e_ref)
    }

    /// Lookup (slow) the event description by name.
    pub fn lookup(&self, name: &str) -> Option<&EventDesc> {
        for desc in &self.store {
            if desc.name == name {
                return Some(desc);
            }
        }
        None
    }

    /// Format the event ID for friendly output.
    pub fn fmt(&self, e_ref: EventRef, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.store.get(e_ref) {
            Some(v) => v.fmt(f),
            None => f.write_fmt(format_args!("Event:!{}", e_ref)),
        }
    }

    /// Format the event packet for friendly output.
    pub fn fmt_evt(
        &self,
        e_ref: EventRef,
        payload: &EventPayload,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.fmt(e_ref, f)?;
        f.write_str(",")?;
        payload.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_er_add_event() {
        let mut er = EventRegistrar::new();
        assert_eq!(0, er.add_event("f0", &EventKind::Signal));
        assert_eq!(1, er.add_event("f1", &EventKind::Message));
        assert_eq!(0, er.add_event("f0", &EventKind::Signal));
        let es = er.close();
        assert_eq!(
            &EventDesc::new(0, "f0".to_string(), EventKind::Signal),
            es.get(0).unwrap()
        );
        assert_eq!(
            &EventDesc::new(1, "f1".to_string(), EventKind::Message),
            es.get(1).unwrap()
        );
    }
}
