//SPDX:MIT

//! Handles sending event signals to jobs.
//!
//! A job will have implementation of an event handler which the glue system
//! registers with this bus.

use std::sync::{Arc, RwLock, mpsc};

use crate::shell_lib::helpers::async_signal::SignalNotice;
use crate::shell_lib::{
    helpers::mapvec::HashMapVec,
    structure::{
        Event, EventCallback, EventKind, EventPayload, EventRef, ExecCtx, InitCtx, ScriptExit,
        event::{EventRegistrar, EventStore},
        job::JobRunnerContext,
        thread::EventCollection,
    },
};

/// The event bus for job handlers.
/// Because of the nature of jobs, handlers are only ever registered
/// at the start.
///
/// The event bus doesn't send out events immediately, but instead it does it in batches.
pub struct JobEventBus {
    inner: Arc<JobEventBusInner>,
    channels: HashMapVec<EventRef, Arc<mpsc::Sender<Event>>>,
    callbacks: HashMapVec<EventRef, Arc<Box<dyn EventCallback>>>,
}

impl JobEventBus {
    pub fn new(
        channels: HashMapVec<EventRef, Arc<mpsc::Sender<Event>>>,
        callbacks: HashMapVec<EventRef, Arc<Box<dyn EventCallback>>>,
    ) -> Self {
        Self {
            channels,
            callbacks,
            inner: Arc::new(JobEventBusInner {
                pending: Arc::new(RwLock::new(Vec::new())),
            }),
        }
    }

    /// Construct a thread sendable version of the bus for use as an exec context.
    /// This does not allow for an asynchronous 'wait' operation on events.
    pub fn as_serial_sender(&self) -> JobEventBusSerialContext {
        JobEventBusSerialContext {
            inner: self.inner.clone(),
        }
    }

    pub fn as_async_sender(&self, notice: SignalNotice) -> JobEventBusAsyncContext {
        JobEventBusAsyncContext {
            signal: notice,
            inner: self.inner.clone(),
        }
    }

    /// Add an event to the ready-to-process list.
    pub fn add_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit> {
        self.inner.add_event((event_ref, payload))
    }
}

impl EventCollection for JobEventBus {
    fn collect_and_handle(&mut self) -> Result<Vec<Event>, ScriptExit> {
        let events = self.inner.extract()?;
        let ctx = Box::new(JobEventBusSerialContext {
            inner: self.inner.clone(),
        });
        let ctx = ctx as Box<dyn JobRunnerContext>;
        for evt in &events {
            for ch in self.channels.get_ref(evt.0) {
                ch.send(evt.clone())?;
            }
            for ch in self.callbacks.get_ref(evt.0) {
                ch.on(&ctx, evt.0, &evt.1)?;
            }
        }
        Ok(events)
    }
}

struct JobEventBusInner {
    pending: Arc<RwLock<Vec<Event>>>,
}

impl JobEventBusInner {
    fn is_empty(&self) -> bool {
        match self.pending.read() {
            Ok(p) => p.is_empty(),
            Err(_) => true, // assume the worst.
        }
    }

    /// Add the event to the pending list.
    fn add_event(&self, event: Event) -> Result<(), ScriptExit> {
        self.pending.write()?.push(event);
        Ok(())
    }

    /// Get the events from the pending list and remove them.
    fn extract(&self) -> Result<Vec<Event>, ScriptExit> {
        let mut vec = self.pending.write()?;
        let len = vec.len();
        Ok(vec.drain(0..len).collect())
    }
}

pub struct JobEventBusSerialContext {
    inner: Arc<JobEventBusInner>,
}

impl JobEventBusSerialContext {
    pub fn as_ctx(&mut self) -> &mut dyn ExecCtx {
        self
    }
}

impl JobRunnerContext for JobEventBusSerialContext {
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit> {
        self.inner.add_event((event_ref, payload))
    }
}

impl ExecCtx for JobEventBusSerialContext {}

pub struct JobEventBusAsyncContext {
    signal: SignalNotice,
    inner: Arc<JobEventBusInner>,
}

impl JobEventBusAsyncContext {
    pub fn as_ctx(&mut self) -> &mut dyn ExecCtx {
        self
    }
}

impl JobRunnerContext for JobEventBusAsyncContext {
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit> {
        let res = self.inner.add_event((event_ref, payload));
        // Send the notice *after* adding the event.
        self.signal.notify();
        res
    }
}

impl ExecCtx for JobEventBusAsyncContext {}

/// Constructs the listener wiring for events.
/// Because of the nature of jobs, handlers are only ever registered
/// at the start.
pub struct JobEventBuilder {
    registrar: EventRegistrar,
    channels: HashMapVec<EventRef, Arc<mpsc::Sender<Event>>>,
    callbacks: HashMapVec<EventRef, Arc<Box<dyn EventCallback>>>,
}

impl JobEventBuilder {
    pub fn new() -> Self {
        Self {
            registrar: EventRegistrar::new(),
            channels: HashMapVec::new(),
            callbacks: HashMapVec::new(),
        }
    }

    /// Allow using this instance as an InitCtx reference.
    pub fn as_ctx(&mut self) -> &mut dyn InitCtx {
        self
    }

    /// Close out the event system buildup.
    pub fn close(self) -> (EventStore, JobEventBus) {
        (
            self.registrar.close(),
            JobEventBus::new(self.channels, self.callbacks),
        )
    }
}

impl InitCtx for JobEventBuilder {
    /// Get or add the event ID for the event name.
    fn get_event(&mut self, name: &str, kind: &EventKind) -> EventRef {
        self.registrar.add_event(name, kind)
    }

    /// Send events with the given ID to the channel.
    fn channel_on(&mut self, e_ref: EventRef, _kind: &EventKind, ch: Arc<mpsc::Sender<Event>>) {
        #[cfg(test)]
        assert!(self.registrar.is_kind(e_ref, _kind));
        self.channels.insert(e_ref, ch);
    }

    /// Send events with the given name to the channel.
    fn channel_on_name(&mut self, event: &str, kind: &EventKind, ch: Arc<mpsc::Sender<Event>>) {
        let evt = self.get_event(event, kind);
        self.channel_on(evt, kind, ch)
    }

    /// Send events with the given ID to the callback.
    fn callback_on(&mut self, e_ref: EventRef, _kind: &EventKind, cb: Arc<Box<dyn EventCallback>>) {
        #[cfg(test)]
        assert!(self.registrar.is_kind(e_ref, _kind));
        self.callbacks.insert(e_ref, cb);
    }

    /// Send events with the given name to the callback.
    fn callback_on_name(&mut self, event: &str, kind: &EventKind, cb: Arc<Box<dyn EventCallback>>) {
        let evt = self.get_event(event, kind);
        self.callback_on(evt, kind, cb)
    }
}
