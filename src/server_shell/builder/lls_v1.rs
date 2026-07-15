//! Turns the AST into a module source.

use std::collections::HashMap;
use std::sync::Arc;

use super::collect;
use super::errors;
use super::rustgen;
use super::writer;
use crate::server_shell::builder::collect::JobSource;
use crate::server_shell::builder::errors::ScriptIssues;
use crate::server_shell::lls::model;
use crate::shell_lib::modules;
use crate::shell_lib::structure::meta;

pub fn lls_v1_to_module_source(
    lls: &model::NativeShellLowLevelScriptSchema,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> ScriptIssues {
    let issues = ScriptIssues::new();

    // Step 1: register all the thread and job definitions.
    let collector = collect::Collector::new();
    pass1(lls, &collector, &issues);

    // Step 2: Generate each file group asynchronously.

    // Step 2a: the commands.
    let cmd = {
        let o2 = out.clone();
        let i2 = issues.clone();
        let c2 = collector.clone();
        std::thread::spawn(move || {
            let _ = rustgen::commands::gen_commands(i2, c2, o2);
        })
    };

    // Step 2b: the jobs
    let job = {
        let o2 = out.clone();
        let i2 = issues.clone();
        let c2 = collector.clone();
        std::thread::spawn(move || {
            let _ = rustgen::jobs::gen_jobs(i2, c2, o2);
        })
    };

    // Step 2c: the runtime.
    // Step 2d: the threads.
    // Step 2e: the main.rs file.
    //super::rustgen::gen_main::write_main_rs(&seq, &out, issues);
    // Step 2f: cargo.toml
    //super::extract_lib::extract_libs(&out, issues);

    // Step 2g: Copy the shell_lib into the output directory.
    //   This can either be an unzip step, or it can be provided
    //   in the executable's install directory.
    //   For now, it's hard-coded.

    // Wait for all the jobs to finish.
    issues.add_result(
        cmd.join()
            .map_err(|e| errors::BuilderError::General(format!("{:?}", e))),
    );
    issues.add_result(
        job.join()
            .map_err(|e| errors::BuilderError::General(format!("{:?}", e))),
    );

    issues
}

pub fn get_available_modules() -> HashMap<String, Arc<meta::ModuleMeta>> {
    let mut ret = HashMap::new();
    for m in modules::available_modules() {
        ret.insert(m.name.clone(), Arc::new(m));
    }
    ret
}

pub fn get_available_macros()
-> HashMap<String, Arc<Box<dyn super::super::meta::MacroMeta + Send + Sync>>> {
    let mut ret = HashMap::new();
    for (n, m) in super::super::macros::available_macros().drain() {
        ret.insert(n, Arc::new(m));
    }
    ret
}

/// Pull in the names into hashes along with generating their reference IDs.
/// This must not run in parallel with the later steps.
fn pass1(
    lls: &model::NativeShellLowLevelScriptSchema,
    collector: &collect::Collector,
    issues: &errors::ScriptIssues,
) {
    // Pull in thread names.
    for (name, ent) in lls.threads.iter() {
        issues.add_result(collector.add_thread(name, &ent));
    }

    let modules = get_available_modules();
    let macros = get_available_macros();

    // Each command is a named job.
    for (name, ent) in lls.commands.iter() {
        let job_struct = match &ent.definition {
            model::CommandDef::ModuleJob(module_job) => issues
                .add_result(new_module_job_structure(module_job, &modules))
                .unwrap_or(collect::JobStructure::Unknown),
            model::CommandDef::MacroJob(macro_job) => issues
                .add_result(new_macro_job_structure(macro_job, &macros))
                .unwrap_or(collect::JobStructure::Unknown),
        };

        issues.add_result(collector.add_job(
            name,
            &ent.source,
            collect::JobSource {
                is_cmd: true,
                structure: job_struct,
            },
        ));
    }

    // Then each job.  These shouldn't overlap the commands.
    for (name, job) in lls.jobs.iter() {
        let name: String = (**name).clone();
        let source = match job {
            model::Job::ModuleJob(j) => j.source.clone(),
            model::Job::StreamJob(j) => j.source.clone(),
            model::Job::InlineJob(j) => j.source.clone(),
            model::Job::MacroJob(j) => j.source.clone(),
        };
        let primary = collector.get_job_src(&name);
        if let Some(primary) = primary {
            issues.add_err(errors::BuilderError::JobCommandOverlap(
                errors::ErrorDetails {
                    message: name,
                    source: source,
                    related: vec![errors::RelatedSource {
                        relation: errors::Relationship::Definition,
                        source: primary.0.clone(),
                    }],
                },
            ));
        } else {
            issues.add_result(collector.add_job(
                &name,
                &source,
                JobSource {
                    is_cmd: false,
                    structure: new_job_structure(job, &modules, &macros, &issues),
                },
            ));
        }
    }
}

fn new_job_structure(
    job: &model::Job,
    modules: &HashMap<String, Arc<meta::ModuleMeta>>,
    macros: &HashMap<String, Arc<Box<dyn super::super::meta::MacroMeta + Send + Sync>>>,
    issues: &errors::ScriptIssues,
) -> collect::JobStructure {
    match job {
        model::Job::ModuleJob(module_job) => issues
            .add_result(new_module_job_structure(module_job, modules))
            .unwrap_or(collect::JobStructure::Unknown),
        model::Job::MacroJob(macro_job) => issues
            .add_result(new_macro_job_structure(macro_job, macros))
            .unwrap_or(collect::JobStructure::Unknown),
        model::Job::StreamJob(stream_job) => {
            collect::JobStructure::Stream(Arc::new(stream_job.clone()))
        }
        model::Job::InlineJob(inline_job) => {
            collect::JobStructure::Inline(Arc::new(inline_job.clone()))
        }
    }
}

fn new_module_job_structure(
    job: &model::ModuleJob,
    modules: &HashMap<String, Arc<meta::ModuleMeta>>,
) -> Result<collect::JobStructure, errors::BuilderError> {
    match modules.get(&job.kind) {
        Some(m) => Ok(collect::JobStructure::Module((
            Arc::new(job.clone()),
            m.clone(),
        ))),
        None => Err(errors::BuilderError::ModuleNotRegistered(
            errors::ErrorDetails {
                message: job.kind.clone(),
                source: job.source.clone(),
                related: Vec::new(),
            },
        )),
    }
}

fn new_macro_job_structure(
    job: &model::MacroJob,
    macros: &HashMap<String, Arc<Box<dyn super::super::meta::MacroMeta + Send + Sync>>>,
) -> Result<collect::JobStructure, errors::BuilderError> {
    match macros.get(&job.kind) {
        Some(m) => Ok(collect::JobStructure::Macro((
            Arc::new(job.clone()),
            m.clone(),
        ))),
        None => Err(errors::BuilderError::MacroNotRegistered(
            errors::ErrorDetails {
                message: job.kind.clone(),
                source: job.source.clone(),
                related: Vec::new(),
            },
        )),
    }
}
