//! The basic event groups defined by the script.
//! The compiler will associate job sequences with these event groups.

pub type EventRef = u32;


pub struct EventGroup {
    /// The internal identifier for the event group.
    pub id: EventRef,

    /// The name of the event group.
    pub name: String,

    /// The description of the event group.
    pub description: String,
}
