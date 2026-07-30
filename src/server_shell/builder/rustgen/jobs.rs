//SPDX:MIT

use std::sync::Arc;
use std::thread;

use crate::server_shell::lls;
use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;
use super::helpers;
use super::names;

/// Write the rust files related to jobs.
/// Returns an empty result for quick returns.
pub fn gen_jobs(
    issues: errors::ScriptIssues,
    collector: collect::Collector,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    // Each job can be written independently.
    let mut jobs = Vec::new();
    for (name, job_ref, cmd) in collector.ordered_jobs().iter() {
        let name = name.clone();
        let job_ref = *job_ref;
        let co2 = collector.clone();
        let c2 = cmd.clone();
        let i2 = issues.clone();
        let o2 = out.clone();
        let n2 = now.clone();
        jobs.push(thread::spawn(move || {
            let _ = gen_job_files(name, job_ref, c2, co2, i2, o2, &n2);
        }));
    }

    // Create the job index while the job generators run.
    let _ = gen_job_index(collector, &issues, out, now);

    // Wait for the job generators to complete.
    for j in jobs {
        issues.add_result(
            j.join()
                .map_err(|e| errors::BuilderError::General(format!("{:?}", e))),
        );
    }
    Ok(())
}

fn gen_job_index(
    collector: collect::Collector,
    issues: &errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    let mut out = issues.consume(out.writer_for("jobs.rs"))?;
    helpers::write_string(
        &mut out,
        issues,
        helpers::rust_file_header(&collector.meta, now),
    )?;

    // Add in the 'pub mod' line for each job.
    let cmds = collector.ordered_jobs();
    for (_, jref, _) in &cmds {
        helpers::write_str(&mut out, issues, "pub mod ")?;
        helpers::write_string(&mut out, issues, names::job_module_name(*jref))?;
        helpers::write_str(&mut out, issues, ";\n")?;
    }
    Ok(())
}

/// Generate the one job's job files.
fn gen_job_files(
    name: String,
    job_ref: structure::JobRef,
    job: Arc<collect::JobSource>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    assert!(!job.is_cmd);

    match &job.structure {
        collect::JobStructure::Inline(inline) => super::job_inline::gen_inline_job_file(
            create_settings(name, job_ref, &inline.source, &now),
            inline.clone(),
            collector,
            issues,
            out,
        ),
        collect::JobStructure::Module(module) => {
            let job_mod = match &module.1.job {
                Some(j) => j,
                None => {
                    // Does not support jobs.
                    issues.add_err(errors::BuilderError::ModuleNotUsableForJob(
                        errors::ErrorDetails {
                            message: (&module.1.name).clone(),
                            source: module.0.source.clone().into(),
                            related: Vec::new(),
                        },
                    ));
                    return Err(());
                }
            };
            super::job_mod::gen_module_job_file(
                create_settings(name, job_ref, &module.0.source, &now),
                module.0.clone(),
                module.1.clone(),
                job_mod,
                collector,
                issues,
                out,
            )
        }
        collect::JobStructure::Macro(mac) => {
            if !mac.1.supports_jobs() {
                issues.add_err(errors::BuilderError::MacroNotUsableForJob(
                    errors::ErrorDetails {
                        message: (*mac.0.macro_).clone(),
                        source: (&mac.0.source).into(),
                        related: Vec::new(),
                    },
                ));
                // Error, but allow this to keep  going.
                Ok(())
            } else {
                super::job_macro::gen_macro_job_file(
                    create_settings(name, job_ref, &mac.0.source, &now),
                    mac.0.clone(),
                    mac.1.clone(),
                    collector,
                    issues,
                    out,
                )
            }
        }
        collect::JobStructure::Stream(stream_job) => super::job_stream::gen_stream_job_file(
            create_settings(name, job_ref, &stream_job.source, now),
            stream_job.clone(),
            collector,
            issues,
            out,
        ),
        collect::JobStructure::Unknown => {
            // Already handled by the unknown detection.
            Ok(())
        }
    }
}

fn create_settings(
    name: String,
    job_ref: structure::JobRef,
    source: &lls::model::Source,
    now: &String,
) -> super::jobs::JobSettings {
    super::jobs::JobSettings {
        name,
        source: source.clone(),
        job_ref,
        parent_module: vec!["jobs".to_string()],
        module_name: names::job_module_name(job_ref),
        mod_struct_name: names::job_mod_struct(job_ref),
        run_struct_name: names::command_run_struct(job_ref),
        now: now.clone(),
    }
}

/// Defines various parts of the job, for constructing the file.
pub struct JobSettings {
    pub name: String,
    pub source: lls::model::Source,
    pub job_ref: structure::JobRef,
    pub parent_module: Vec<String>,
    pub module_name: String,
    pub run_struct_name: String,
    pub mod_struct_name: String,
    pub now: String,
}
