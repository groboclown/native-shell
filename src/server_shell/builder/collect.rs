//SPDX:MIT

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::errors;
use crate::{
    server_shell::{lls, meta},
    shell_lib::structure,
};

pub struct JobSource {
    pub structure: JobStructure,
    pub is_cmd: bool,
}

pub enum JobStructure {
    Inline(Arc<lls::model::InlineJob>),
    Module((Arc<lls::model::ModuleJob>, Arc<structure::meta::ModuleMeta>)),
    Macro(
        (
            Arc<lls::model::MacroJob>,
            Arc<Box<dyn meta::MacroMeta + Send + Sync>>,
            Option<Arc<meta::MacroModule>>, // set once the macro has been built.
        ),
    ),
    Stream(Arc<lls::model::StreamJob>),
    Unknown,
}

/// Collect the different kinds of references in a thread safe way.
/// This handles the first pass of reading in the LLS - it gathers the data to find
/// missing references.
/// Once this pass finishes, then the handler can inspect the job runtime values for
/// valid module references in the linked-to jobs, and for valid stream references
/// in the stream jobs.
pub struct Collector {
    pub meta: Arc<lls::model::Metadata>,
    threads: LockedBuilderRef<lls::model::Thread>,
    jobs: LockedBuilderRef<JobSource>,
    events: LockedBuilderRef<structure::EventKind>,
    future: Arc<Mutex<Vec<FutureResolve>>>,
}

#[derive(Clone)]
struct FutureResolve {
    job_name: String,
    source: lls::model::Source,
    state_field: String,
    set_type: Option<structure::meta::ValueType>,
}

impl Clone for Collector {
    fn clone(&self) -> Self {
        Self {
            meta: self.meta.clone(),
            threads: self.threads.clone(),
            jobs: self.jobs.clone(),
            events: self.events.clone(),
            future: self.future.clone(),
        }
    }
}

impl Collector {
    pub fn new(meta: &lls::model::Metadata) -> Self {
        Self {
            meta: Arc::new(meta.clone()),
            threads: LockedBuilderRef::new(),
            jobs: LockedBuilderRef::new(),
            events: LockedBuilderRef::new(),
            future: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add the primary thread.
    /// Call during the first pass when collecting all the thread names.
    pub fn add_thread(
        &self,
        name: &String,
        source: &lls::model::Thread,
    ) -> Result<structure::ThreadRef, errors::BuilderError> {
        self.threads
            .add_primary(name.clone(), source.source.clone(), source.clone())
            .map_err(|e| errors::BuilderError::LLSBug(e))
    }

    /// Add a reference to a thread.
    pub fn ref_thread(
        &self,
        name: &String,
        source: &lls::model::Source,
    ) -> Result<structure::ThreadRef, errors::BuilderError> {
        self.threads
            .add_ref(name.clone(), source.clone())
            .map(|r| r.0)
            .map_err(|e| errors::BuilderError::NoSuchThread(e))
    }

    pub fn has_thread(&self, name: &String) -> bool {
        self.threads.contains(name)
    }

    pub fn get_thread(&self, name: &String) -> Option<Arc<lls::model::Thread>> {
        self.threads.get(name)
    }

    /// Get the thread, and returns an error if it was not registered.
    pub fn get_thread_checked(
        &self,
        name: &String,
        source: &lls::model::Source,
    ) -> Result<Arc<lls::model::Thread>, errors::BuilderError> {
        self.threads
            .get(name)
            .ok_or(errors::BuilderError::NoSuchThread(errors::ErrorDetails {
                message: name.clone(),
                source: source.into(),
                related: Vec::new(),
            }))
    }

    /// Get the list of the registered thread names, ordered by thread ref.
    pub fn ordered_threads(&self) -> Vec<(String, structure::ThreadRef, Arc<lls::model::Thread>)> {
        self.threads.ordered_values()
    }

    /// Add the primary job.
    /// Called by the first pass, collecting all the job names.
    pub fn add_job(
        &self,
        name: &String,
        source: &lls::model::Source,
        job: JobSource,
    ) -> Result<structure::JobRef, errors::BuilderError> {
        // An error here means that there's a job/cmd collision, or a bug.
        self.jobs
            .add_primary(name.clone(), source.clone(), job)
            .map_err(|e| errors::BuilderError::JobCommandOverlap(e))
    }

    pub fn add_job_ref(
        &self,
        name: &String,
        source: &lls::model::Source,
    ) -> Result<structure::JobRef, errors::BuilderError> {
        self.jobs
            .add_ref(name.clone(), source.clone())
            .map(|e| e.0)
            .map_err(|e| errors::BuilderError::NoSuchJob(e))
    }

    pub fn has_job(&self, name: &String) -> bool {
        self.jobs.contains(name)
    }

    pub fn get_job(&self, name: &String) -> Option<Arc<JobSource>> {
        self.jobs.get(name)
    }

    pub fn get_job_ref(&self, name: &String) -> Option<structure::JobRef> {
        self.jobs.get_ref_id(name)
    }

    pub fn get_job_src(&self, name: &String) -> Option<(lls::model::Source, Arc<JobSource>)> {
        match self.jobs.get(name) {
            Some(j) => self.jobs.get_primary(name).map(|p| (p.clone(), j)),
            None => None,
        }
    }

    /// Get the job, and returns an error if it was not registered.
    pub fn get_job_checked<'a, 'b, 'c>(
        &'a self,
        name: &'b String,
        source: &'c lls::model::Source,
    ) -> Result<Arc<JobSource>, errors::BuilderError> {
        self.jobs
            .get(name)
            .ok_or(errors::BuilderError::NoSuchJob(errors::ErrorDetails {
                message: name.clone(),
                source: source.into(),
                related: Vec::new(),
            }))
    }

    pub fn set_macro_job_definition(
        &self,
        name: &String,
        source: &lls::model::Source,
        definition: Arc<meta::MacroModule>,
    ) -> Result<(), errors::BuilderError> {
        let job = self.get_job_checked(name, source)?;
        match &job.structure {
            JobStructure::Macro(m_j) => self
                .jobs
                .update(name, source, |v| JobSource {
                    structure: JobStructure::Macro((
                        m_j.0.clone(),
                        m_j.1.clone(),
                        Some(definition),
                    )),
                    is_cmd: v.is_cmd,
                })
                .map_err(|e| errors::BuilderError::NoSuchJob(e)),
            _ => Err(errors::BuilderError::MacroNotRegistered(
                errors::ErrorDetails {
                    message: name.clone(),
                    source: source.into(),
                    related: Vec::new(),
                },
            )),
        }
    }

    /// Get the list of the registered job names (not commands), ordered by job ref.
    pub fn ordered_jobs(&self) -> Vec<(String, structure::JobRef, Arc<JobSource>)> {
        let mut ret = Vec::new();
        let mut jobs = self.jobs.ordered_values();
        for (n, j, s) in jobs.drain(0..jobs.len()) {
            if !s.is_cmd {
                ret.push((n, j, s));
            }
        }
        ret
    }

    /// Get the list of the registered command names (not jobs), ordered by job ref.
    pub fn ordered_commands(&self) -> Vec<(String, structure::JobRef, Arc<JobSource>)> {
        let mut ret = Vec::new();
        let mut jobs = self.jobs.ordered_values();
        for (n, j, s) in jobs.drain(0..jobs.len()) {
            if s.is_cmd {
                ret.push((n, j, s));
            }
        }
        ret
    }

    /// Get the list of the registered jobs + commands, ordered by job ref.
    pub fn ordered_jobs_commands(&self) -> Vec<(String, structure::JobRef, Arc<JobSource>)> {
        self.jobs.ordered_values()
    }

    /// Get the events reference with the given name.
    pub fn mark_event_ref(
        &self,
        name: &String,
        source: &lls::model::Source,
        kind: &structure::EventKind,
    ) -> Result<structure::EventRef, errors::BuilderError> {
        // Make some attempts.
        // If added as primary, then everything's fine.
        if let Ok(r) = self
            .events
            .add_primary(name.clone(), source.clone(), kind.clone())
        {
            return Ok(r);
        }
        // It's already been added, so add a reference...
        match self.events.add_ref(name.clone(), source.clone()) {
            // Should not happen due to already trying to add the primary.
            Err(e) => Err(errors::BuilderError::LLSBug(e)),
            // The original event kind inserter matches this expected kind.
            Ok((r, k)) if *k == *kind => Ok(r),
            Ok((_, k)) => Err(errors::BuilderError::EventKindMismatch(
                errors::ErrorDetails {
                    message: format!(
                        "referenced event '{}' with kind {:?}, but it was already used as {:?}",
                        name, kind, k
                    ),
                    source: source.into(),
                    related: self
                        .events
                        .get_all_refs(name)
                        .iter()
                        .map(|s| errors::RelatedSource {
                            relation: errors::Relationship::Definition,
                            source: s.into(),
                        })
                        .collect(),
                },
            )),
        }
    }

    /// Get the list of the registered events names, ordered by events ref.
    pub fn ordered_events(&self) -> Vec<(String, structure::EventRef, structure::EventKind)> {
        self.events
            .ordered_values()
            .iter()
            .map(|v| (v.0.clone(), v.1, (*v.2).clone()))
            .collect()
    }

    /// Get the named job's state's field.
    /// This can return None in the case of macro definitions, when the macro hasn't been declared yet.
    /// This does not perform type evaluation; instead, it uses set_to_type as a future
    /// type evaluation for calls to `resolve_pending_job_fields()`.
    pub fn get_job_state_field<'a, 'b, 'c, 'd, 'e>(
        &'a self,
        source: &'b lls::model::Source,
        job_name: &'c String,
        field_name: &'d String,
        set_to_type: &'e Option<structure::meta::ValueType>,
    ) -> Result<(structure::JobRef, Option<structure::meta::NamedValue>), errors::BuilderError>
    {
        let job_src = self.get_job_checked(job_name, source)?;
        let job_ref = self
            .jobs
            .get_ref_id(job_name)
            .ok_or(errors::BuilderError::General(
                "bug: job found but no id found".into(),
            ))?;
        let kind = match job_src.is_cmd {
            true => "command",
            false => "job",
        };
        match &job_src.structure {
            JobStructure::Module(module) => {
                match get_field_named(
                    field_name,
                    &(get_mod_state_struct(job_name, source, &job_src, &module.1)?.fields),
                ) {
                    Some(nv) => Ok((job_ref, Some(nv.clone()))),
                    None => Err(errors::BuilderError::NoSuchField(errors::ErrorDetails {
                        message: format!("{} in {} {}", field_name, kind, job_name),
                        source: source.into(),
                        related: Vec::new(),
                    })),
                }
            }
            JobStructure::Macro(m_job) => match &m_job.2 {
                Some(j) => match get_field_named(
                    field_name,
                    &(get_mod_state_struct(job_name, source, &job_src, &j.meta)?.fields),
                ) {
                    Some(nv) => Ok((job_ref, Some(nv.clone()))),
                    None => Err(errors::BuilderError::NoSuchField(errors::ErrorDetails {
                        message: format!("{} in {} {}", field_name, kind, job_name),
                        source: source.into(),
                        related: Vec::new(),
                    })),
                },
                None => {
                    // Indeterminate; must decide later.
                    match self.future.lock() {
                        Ok(mut f) => f.push(FutureResolve {
                            job_name: job_name.clone(),
                            source: source.clone(),
                            state_field: field_name.clone(),
                            set_type: set_to_type.clone(),
                        }),
                        Err(mut e) => (*e.get_mut()).push(FutureResolve {
                            job_name: job_name.clone(),
                            source: source.clone(),
                            state_field: field_name.clone(),
                            set_type: set_to_type.clone(),
                        }),
                    }
                    Ok((job_ref, None))
                }
            },
            JobStructure::Stream(_) | JobStructure::Unknown | JobStructure::Inline(_) => {
                // Currently, these job definitions do not allow for defining state.
                Err(errors::BuilderError::NoSuchField(errors::ErrorDetails {
                    message: format!("{} in {} {}", field_name, kind, job_name),
                    source: source.into(),
                    related: Vec::new(),
                }))
            }
        }
    }

    /// Resolve all job fields that had a requested lookup, but whose presence or type was not available.
    /// Call after the construction of all the macro jobs to perform a final resolution of generated
    /// state fields.
    pub fn resolve_pending_job_fields(&self) -> Result<(), errors::BuilderError> {
        todo!()
    }
}

fn get_field_named<'a, 'b>(
    name: &'a String,
    fields: &'b Vec<structure::meta::NamedValue>,
) -> Option<&'b structure::meta::NamedValue> {
    for nv in fields {
        if nv.name == *name {
            return Some(nv);
        }
    }
    None
}

fn get_mod_state_struct<'a, 'b, 'c, 'd>(
    name: &'a String,
    source: &'b lls::model::Source,
    job_src: &'c JobSource,
    module: &'d structure::meta::ModuleMeta,
) -> Result<structure::meta::ModuleStructure, errors::BuilderError> {
    match &job_src.is_cmd {
        true => match &module.command {
            Some(c) => match &c.state_struct {
                Some(s) => Ok(s.clone()),
                None => Err(errors::BuilderError::NoStateForModule(
                    errors::ErrorDetails {
                        message: name.clone(),
                        source: source.into(),
                        related: Vec::new(),
                    },
                )),
            },
            None => Err(errors::BuilderError::ModuleNotUsableForCommand(
                errors::ErrorDetails {
                    message: name.clone(),
                    source: source.into(),
                    related: Vec::new(),
                },
            )),
        },
        false => match &module.job {
            Some(j) => match &j.state_struct {
                Some(s) => Ok(s.clone()),
                None => Err(errors::BuilderError::NoStateForModule(
                    errors::ErrorDetails {
                        message: name.clone(),
                        source: source.into(),
                        related: Vec::new(),
                    },
                )),
            },
            None => Err(errors::BuilderError::ModuleNotUsableForJob(
                errors::ErrorDetails {
                    message: name.clone(),
                    source: source.into(),
                    related: Vec::new(),
                },
            )),
        },
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

struct LockedBuilderRef<T> {
    lb: Arc<Mutex<BuilderRef<Arc<T>>>>,
}

impl<T> Clone for LockedBuilderRef<T> {
    fn clone(&self) -> Self {
        Self {
            lb: self.lb.clone(),
        }
    }
}

impl<T> LockedBuilderRef<T> {
    pub fn new() -> Self {
        Self {
            lb: Arc::new(Mutex::new(BuilderRef::new())),
        }
    }

    pub fn add_primary(
        &self,
        name: String,
        source: lls::model::Source,
        val: T,
    ) -> Result<usize, errors::ErrorDetails> {
        match self.lb.lock() {
            Ok(mut m) => m.add_primary(name, source, Arc::new(val)),
            Err(mut e) => (*e.get_mut()).add_primary(name, source, Arc::new(val)),
        }
    }

    pub fn add_ref(
        &self,
        name: String,
        source: lls::model::Source,
    ) -> Result<(usize, Arc<T>), errors::ErrorDetails> {
        match self.lb.lock() {
            Ok(mut m) => m.add_ref(name, source).map(|v| (v.0, v.1.clone())),
            Err(mut e) => (*e.get_mut())
                .add_ref(name, source)
                .map(|v| (v.0, v.1.clone())),
        }
    }

    pub fn contains(&self, name: &String) -> bool {
        match self.lb.lock() {
            Ok(m) => m.contains(name),
            Err(e) => (*e.get_ref()).contains(name),
        }
    }

    pub fn get(&self, name: &String) -> Option<Arc<T>> {
        match self.lb.lock() {
            Ok(m) => m.get_val(name).map(|v| v.clone()),
            Err(e) => (*e.get_ref()).get_val(name).map(|v| v.clone()),
        }
    }

    pub fn get_ref_id(&self, name: &String) -> Option<usize> {
        match self.lb.lock() {
            Ok(m) => m.get_ref_id(name),
            Err(e) => (*e.get_ref()).get_ref_id(name),
        }
    }

    pub fn get_primary(&self, name: &String) -> Option<lls::model::Source> {
        match self.lb.lock() {
            Ok(m) => m.get_primary(name).map(|v| v.clone()),
            Err(e) => (*e.get_ref()).get_primary(name).map(|v| v.clone()),
        }
    }

    pub fn get_all_refs(&self, name: &String) -> Vec<lls::model::Source> {
        match self.lb.lock() {
            Ok(m) => {
                let mut r = m.get_refs(name);
                if let Some(s) = m.get_primary(name) {
                    r.insert(0, s.clone());
                }
                r
            }
            Err(e) => {
                let mut r = (*e.get_ref()).get_refs(name).clone();
                if let Some(s) = (*e.get_ref()).get_primary(name) {
                    r.insert(0, s.clone());
                }
                r
            }
        }
    }

    pub fn update(
        &self,
        name: &String,
        source: &lls::model::Source,
        f: impl FnOnce(&T) -> T,
    ) -> Result<(), errors::ErrorDetails> {
        match self.lb.lock() {
            Ok(mut m) => m.update(name, source, |v| Arc::new(f(v.as_ref()))),
            Err(mut e) => (*e.get_mut()).update(name, source, |v| Arc::new(f(v.as_ref()))),
        }
    }

    pub fn ordered_primary(&self) -> Vec<(String, usize, lls::model::Source)> {
        match self.lb.lock() {
            Ok(m) => m.ordered_primary(),
            Err(e) => (*e.get_ref()).ordered_primary(),
        }
    }

    pub fn ordered_refs(&self) -> std::ops::Range<usize> {
        match self.lb.lock() {
            Ok(m) => m.ordered_refs(),
            Err(e) => (*e.get_ref()).ordered_refs(),
        }
    }

    pub fn ordered_values(&self) -> Vec<(String, usize, Arc<T>)> {
        match self.lb.lock() {
            Ok(m) => m
                .ordered_values()
                .iter()
                .map(|v| (v.0.clone(), v.1, v.2.clone()))
                .collect(),
            Err(e) => (*e.get_ref())
                .ordered_values()
                .iter()
                .map(|v| (v.0.clone(), v.1, v.2.clone()))
                .collect(),
        }
    }
}

struct RefEntry<T> {
    name: String,
    primary: lls::model::Source,
    refs: Vec<lls::model::Source>,
    val: T,
}

/// Keeps a *Ref (index) to a primary source (index 0) + all its references.
struct BuilderRef<T> {
    by_ref: Vec<RefEntry<T>>,
    by_name: HashMap<String, usize>,
}

impl<T> BuilderRef<T> {
    fn new() -> Self {
        BuilderRef {
            by_ref: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    fn add_primary(
        &mut self,
        name: String,
        source: lls::model::Source,
        val: T,
    ) -> Result<usize, errors::ErrorDetails> {
        match self.by_name.get(&name) {
            None => {
                let r = self.by_ref.len();
                self.by_name.insert(name.clone(), r);
                let entry = RefEntry {
                    name: name.clone(),
                    primary: source,
                    refs: Vec::new(),
                    val: val,
                };
                self.by_ref.push(entry);
                Ok(r)
            }
            Some(r) => {
                let v = self.by_ref.get(*r).expect("should exist");
                Err(errors::ErrorDetails {
                    message: format!("attempted to register '{}'", name),
                    source: source.into(),
                    related: vec![errors::RelatedSource {
                        relation: errors::Relationship::Definition,
                        source: (&v.primary).into(),
                    }],
                })
            }
        }
    }

    fn update(
        &mut self,
        name: &String,
        source: &lls::model::Source,
        f: impl FnOnce(&T) -> T,
    ) -> Result<(), errors::ErrorDetails> {
        let rid = match self.by_name.get(name) {
            Some(r) => *r,
            None => {
                return Err(errors::ErrorDetails {
                    message: name.clone(),
                    source: source.into(),
                    related: Vec::new(),
                });
            }
        };
        let old = self.by_ref.get_mut(rid);
        match old {
            None => Err(errors::ErrorDetails {
                message: rid.to_string(),
                source: source.into(),
                related: Vec::new(),
            }),
            Some(old) => {
                old.val = f(&old.val);
                Ok(())
            }
        }
    }

    fn add_ref(
        &mut self,
        name: String,
        source: lls::model::Source,
    ) -> Result<(usize, &T), errors::ErrorDetails> {
        match self.by_name.get(&name) {
            Some(r) => {
                let entry = self.by_ref.get_mut(*r).expect("exists");
                entry.refs.push(source);
                Ok((*r, &entry.val))
            }
            None => Err(errors::ErrorDetails {
                message: format!("reference to non-existent '{}'", name),
                source: source.into(),
                related: Vec::new(),
            }),
        }
    }

    fn contains(&self, name: &String) -> bool {
        self.by_name.contains_key(name)
    }

    /// Get the source for the primary declaration of the thing.
    fn get_primary(&self, name: &String) -> Option<&lls::model::Source> {
        if let Some(r) = self.by_name.get(name) {
            self.by_ref.get(*r).map(|v| &v.primary)
        } else {
            None
        }
    }

    /// Get everything that references this item.
    fn get_refs(&self, name: &String) -> Vec<lls::model::Source> {
        if let Some(r) = self.by_name.get(name) {
            self.by_ref.get(*r).expect("exists").refs.clone()
        } else {
            Vec::new()
        }
    }

    fn get_ref_id(&self, name: &String) -> Option<usize> {
        self.by_name.get(name).map(|f| *f)
    }

    fn get_val(&self, name: &String) -> Option<&T> {
        match self.by_name.get(name) {
            None => None,
            Some(r) => self.by_ref.get(*r).map(|e| &e.val),
        }
    }

    pub fn ordered_primary(&self) -> Vec<(String, usize, lls::model::Source)> {
        self.by_ref
            .iter()
            .enumerate()
            .map(|i| (i.1.name.clone(), i.0, i.1.primary.clone()))
            .collect()
    }

    pub fn ordered_values(&self) -> Vec<(String, usize, &T)> {
        self.by_ref
            .iter()
            .enumerate()
            .map(|i| (i.1.name.clone(), i.0, &i.1.val))
            .collect()
    }

    pub fn ordered_refs(&self) -> std::ops::Range<usize> {
        0..self.by_ref.len()
    }
}
