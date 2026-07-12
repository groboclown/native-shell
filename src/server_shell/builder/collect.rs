//SPDX:MIT

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    server_shell::lls,
    shell_lib::structure::{EventKind, EventRef, JobRef, ThreadRef, thread::ScheduleStep},
};

/// Collect the different kinds of references in a thread safe way.
/// This handles the first pass of reading in the LLS - it gathers the data to find
/// missing references.
/// Once this pass finishes, then the handler can inspect the job runtime values for
/// valid module references in the linked-to jobs, and for valid stream references
/// in the stream jobs.
pub struct Collector {
    threads: Arc<Mutex<BuilderRef<Vec<ScheduleStep>>>>,
    jobs: Arc<Mutex<BuilderRef<JobInstance>>>,
    events: Arc<Mutex<BuilderRef<Option<EventKind>>>>,
}

impl Clone for Collector {
    fn clone(&self) -> Self {
        Self {
            threads: self.threads.clone(),
            jobs: self.jobs.clone(),
            events: self.events.clone(),
        }
    }
}

impl Collector {
    pub fn new() -> Self {
        Self {
            threads: Arc::new(Mutex::new(BuilderRef::new(Vec::new()))),
            jobs: Arc::new(Mutex::new(BuilderRef::new(JobInstance::Unknown))),
            events: Arc::new(Mutex::new(BuilderRef::new(None))),
        }
    }

    /// Get the thread reference with the given name.
    pub fn get_thread_ref(&self, name: &String) -> Result<ThreadRef, String> {
        let mut m = self.threads.lock().map_err(|e| format!("{}", e))?;
        Ok(m.get_ref(name.clone()))
    }

    pub fn add_thread_step(&self, name: &String, step: ScheduleStep) -> Result<(), String> {
        let mut m = self.threads.lock().map_err(|e| format!("{}", e))?;
        m.update(name, |mut v: Vec<ScheduleStep>| {
            v.push(step);
            v
        })
    }

    /// Get the list of the registered thread names, ordered by thread ref.
    pub fn ordered_threads(&self) -> Result<Vec<(ThreadRef, Vec<ScheduleStep>)>, String> {
        let m = self.threads.lock().map_err(|e| format!("{}", e))?;
        Ok(m.ordered()
            .iter()
            .enumerate()
            .map(|e| (e.0, e.1.clone()))
            .collect())
    }

    /// Get the job reference with the given name.
    pub fn get_job_ref(&self, name: &String) -> Result<JobRef, String> {
        let mut m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        Ok(m.get_ref(name.clone()))
    }

    /// Get the list of the registered job names, ordered by job ref.
    pub fn ordered_jobs(&self) -> Result<Vec<(JobRef, JobInstance)>, String> {
        let m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        Ok(m.ordered()
            .iter()
            .enumerate()
            .map(|e| (e.0, e.1.clone()))
            .collect())
    }

    /// Set the named job as a macro.
    /// The caller must ensure that the referenced macro exists.
    pub fn set_macro_job(
        &self,
        name: &String,
        code: lls::model::MacroJob,
    ) -> Result<JobRef, String> {
        let mut m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        let ret = m.get_ref(name.clone());
        m.update(name, |_| JobInstance::Macro(code));
        Ok(ret)
    }

    /// Set the named job as a module.
    /// The caller must ensure that the referenced module exists.
    pub fn set_module_job(
        &self,
        name: &String,
        code: lls::model::ModuleJob,
    ) -> Result<JobRef, String> {
        let mut m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        let ret = m.get_ref(name.clone());
        m.update(name, |_| JobInstance::Module(code));
        Ok(ret)
    }

    /// Set the named job as inline.
    pub fn set_inline_job(
        &self,
        name: &String,
        code: lls::model::ModuleJob,
    ) -> Result<JobRef, String> {
        let mut m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        let ret = m.get_ref(name.clone());
        m.update(name, |_| JobInstance::Module(code));
        Ok(ret)
    }

    /// Set the named job as stream.
    pub fn set_stream_job(
        &self,
        name: &String,
        code: lls::model::StreamJob,
    ) -> Result<JobRef, String> {
        let mut m = self.jobs.lock().map_err(|e| format!("{}", e))?;
        let ret = m.get_ref(name.clone());
        m.update(name, |_| JobInstance::Stream(code));
        Ok(ret)
    }

    /// Get the events reference with the given name.
    pub fn get_event_ref(&self, name: &String) -> Result<EventRef, String> {
        let mut m = self.events.lock().map_err(|e| format!("{}", e))?;
        Ok(m.get_ref(name.clone()))
    }

    pub fn set_event_kind(&self, name: &String, kind: &EventKind) -> Result<(), String> {
        // Note: duplicates some logic from EventRegistrar::add_event.
        let mut m = self.events.lock().map_err(|e| format!("{}", e))?;
        if let Some(v) = m.get(name) {
            match v {
                None => m.update(name, |_| Some(kind.clone())),
                Some(v) if v == kind => Ok(()),
                Some(v) => Err(format!(
                    "event {} kind mismatch (has {:?}, but new version is {:?})",
                    name, kind, v
                )),
            }
        } else {
            m.get_ref(name.clone());
            m.update(name, |_| Some(kind.clone()))
        }
    }

    /// Get the list of the registered events names, ordered by events ref.
    pub fn ordered_events(&self) -> Result<Vec<(EventRef, Option<EventKind>)>, String> {
        let m = self.events.lock().map_err(|e| format!("{}", e))?;
        Ok(m.ordered()
            .iter()
            .enumerate()
            .map(|e| (e.0, e.1.clone()))
            .collect())
    }
}

#[derive(Clone)]
pub enum JobInstance {
    Macro(lls::model::MacroJob),
    Module(lls::model::ModuleJob),
    Inline(lls::model::InlineJob),
    Stream(lls::model::StreamJob),
    Unknown,
}

struct BuilderRef<T: Clone> {
    builder: T,
    by_ref: Vec<T>,
    by_name: HashMap<String, usize>,
}

impl<T: Clone> BuilderRef<T> {
    fn new(base: T) -> Self {
        BuilderRef {
            builder: base,
            by_ref: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    fn get_ref(&mut self, name: String) -> usize {
        if let Some(r) = self.by_name.get(&name) {
            *r
        } else {
            let r = self.by_ref.len();
            self.by_name.insert(name, r);
            self.by_ref.push(self.builder.clone());
            r
        }
    }

    fn contains(&self, name: &String) -> bool {
        self.by_name.contains_key(name)
    }

    fn get(&self, name: &String) -> Option<&T> {
        if let Some(r) = self.by_name.get(name) {
            self.by_ref.get(*r)
        } else {
            None
        }
    }

    fn update<F: FnOnce(T) -> T>(&mut self, name: &String, f: F) -> Result<(), String> {
        if let Some(r) = self.by_name.get(name) {
            // We want to extract the value out of the index, modify it, then put it back in.
            // Simulate this with a push blank at the end, swap_remove() to extract the
            // index and replace it with the final item, modify, then push the modified back
            // and swap_remove again.
            self.by_ref.push(self.builder.clone());
            let val = self.by_ref.swap_remove(*r);
            let val = f(val);
            self.by_ref.push(val);
            self.by_ref.swap_remove(*r);
            Ok(())
        } else {
            Err(format!("not registered: {}", name))
        }
    }

    fn ordered(&self) -> &Vec<T> {
        &self.by_ref
    }
}
