//! Turns the AST into a module source.

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use super::collect;
use super::errors;
use super::sequence::StdSequenceStore;
use super::writer::SourceWriter;
use crate::server_shell::lls::model;
use crate::shell_lib::modules;
use crate::shell_lib::structure::meta;

const LLS_VERSION_1: &str = "1.0.0";

pub fn lls_to_module_source<SW: SourceWriter>(
    lls: &model::NativeShellLowLevelScriptSchema,
    out: SW,
) -> Result<(), errors::BuilderError> {
    check_supported(lls)?;

    // Step 1: register all the thread and job definitions.
    let collector = collect::Collector::new();

    // Step 2: Create the commands.
    // Step 3: Create the jobs + runtime.
    // Step 4: Create the threads.

    // Step 5: Write the main.rs file.
    super::rustgen::gen_main::write_main_rs(&seq, &out)?;

    // Step 6: Copy the shell_lib into the output directory.
    //   This can either be an unzip step, or it can be provided
    //   in the executable's install directory.
    //   For now, it's hard-coded.
    super::extract_lib::extract_libs(&out)?;

    Ok(())
}

pub fn check_supported(
    lls: &model::NativeShellLowLevelScriptSchema,
) -> Result<(), errors::BuilderError> {
    if lls.schema_version != LLS_VERSION_1 {
        Err(errors::BuilderError::InvalidLLS(errors::ErrorDetails {
            message: format!(
                "Unsupported LLS version: {}. Supported versions include: {}",
                lls.schema_version, LLS_VERSION_1
            ),
            source: lls.meta.source.clone(),
            related: vec![],
        }))
    } else {
        Ok(())
    }
}

pub fn get_available_modules() -> HashMap<String, Arc<meta::ModuleMeta>> {
    let mut ret = HashMap::new();
    for m in modules::available_modules() {
        ret.insert(m.name.clone(), Arc::new(m));
    }
    ret
}

pub fn get_available_macros() -> HashMap<String, Arc<Box<dyn super::super::meta::MacroMeta>>> {
    let mut ret = HashMap::new();
    for (n, m) in super::super::macros::available_macros().drain() {
        ret.insert(n, Arc::new(m));
    }
    ret
}

fn pass1(lls: &model::NativeShellLowLevelScriptSchema, collector: &collect::Collector) {
    // Pull in thread names.
    for (name, ent) in lls.threads.iter() {
        collector
            .issues
            .add_result(collector.add_thread(name, &ent.source));
    }

    let modules = get_available_modules();
    let macros = get_available_macros();

    // Each command is a named job.
    for (name, ent) in lls.commands.iter() {
        let job_struct = match ent.definition {
            model::CommandDef::ModuleJob(module_job) => match modules.get(&module_job.kind) {
                Some(m) => collect::JobStructure::Module(m.clone()),
                None => {
                    collector
                        .issues
                        .add_err(errors::BuilderError::ModuleNotRegistered(
                            errors::ErrorDetails {
                                message: module_job.kind.clone(),
                                source: ent.source.clone(),
                                related: Vec::new(),
                            },
                        ));
                    collect::JobStructure::Unknown
                }
            },
            model::CommandDef::MacroJob(macro_job) => match macros.get(&macro_job.kind) {
                Some(m) => collect::JobStructure::Macro(m.clone()),
                None => {
                    collector
                        .issues
                        .add_err(errors::BuilderError::MacroNotRegistered(
                            errors::ErrorDetails {
                                message: macro_job.kind.clone(),
                                source: ent.source.clone(),
                                related: Vec::new(),
                            },
                        ));
                    collect::JobStructure::Unknown
                }
            },
        };

        collector.issues.add_result(collector.add_job(
            name,
            &ent.source,
            collect::JobSource {
                is_cmd: true,
                structure: job_struct,
            },
        ));
    }

    // Then each job.  These shouldn't overlap the commands.
    for name in lls.jobs.keys() {
        if collector.has_job(name) {
            collector
                .issues
                .add_err(errors::BuilderError::JobCommandOverlap(
                    errors::ErrorDetails {
                        message: format!("job ({})"),
                        source: (),
                        related: (),
                    },
                ));
        }
    }
}
