//SPDX:MIT

//! Inter-job event system.

use std::{collections::HashMap, fmt::Display};

/// Events are sent as names from the modules.
/// They pass through the event registrar to turn into ids for faster internal checking.
pub type EventRef = usize;

pub type SignalCode = i32;

#[derive(Debug, Clone, PartialEq)]
pub struct EventDesc {
    pub e_ref: EventRef,
    pub name: String,
}

impl EventDesc {
    pub fn new(e_ref: EventRef, name: String) -> Self {
        Self { e_ref, name }
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
    pub fn add_event(&mut self, name: &str) -> EventRef {
        let name = name.to_string();
        match self.by_name.get(&name) {
            Some(v) => *v,
            None => {
                let e_ref = self.by_ref.len();
                self.by_name.insert(name.clone(), e_ref);
                self.by_ref.push(EventDesc::new(e_ref, name));
                e_ref
            }
        }
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
        assert_eq!(0, er.add_event("f0"));
        assert_eq!(1, er.add_event("f1"));
        assert_eq!(0, er.add_event("f0"));
        let es = er.close();
        assert_eq!(&EventDesc::new(0, "f0".to_string()), es.get(0).unwrap());
        assert_eq!(&EventDesc::new(1, "f1".to_string()), es.get(1).unwrap());
    }
}
