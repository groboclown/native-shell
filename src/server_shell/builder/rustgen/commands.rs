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

/// Write the rust files related to commands.
/// Returns an empty result for quick returns.
pub fn gen_commands(
    issues: errors::ScriptIssues,
    collector: collect::Collector,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    // Each command can be written independently.
    let mut cmds = Vec::new();
    for (name, job_ref, cmd) in collector.ordered_commands().iter() {
        let name = name.clone();
        let job_ref = *job_ref;
        let co2 = collector.clone();
        let c2 = cmd.clone();
        let i2 = issues.clone();
        let o2 = out.clone();
        let n2 = now.clone();
        cmds.push(thread::spawn(move || {
            let _ = gen_command_file(name, job_ref, c2, co2, i2, o2, &n2);
        }));
    }

    // Create the command index while the command generators run.
    let _ = gen_command_index(collector, &issues, out, now);

    // Wait for the command generators to complete.
    for j in cmds {
        issues.add_result(
            j.join()
                .map_err(|e| errors::BuilderError::General(format!("{:?}", e))),
        );
    }
    Ok(())
}

fn gen_command_index(
    collector: collect::Collector,
    issues: &errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    let mut out = issues.consume(out.writer_for("commands.rs").map_err(|e| e.into()))?;
    helpers::write_string(
        &mut out,
        issues,
        helpers::rust_file_header(&collector.meta, now),
    )?;

    // Add in the 'pub mod' line for each command.
    let cmds = collector.ordered_commands();
    for (_, jref, _) in &cmds {
        helpers::write_str(&mut out, issues, "pub mod ")?;
        helpers::write_string(&mut out, issues, names::command_module(*jref))?;
        helpers::write_str(&mut out, issues, ";\n")?;
    }

    // If there is just one command:
    //   Add the pub use for the state.
    // If there are multiple commands:
    //   Add in the SubCommand enum
    //   For the SubCommand enum, the enum entry name == structure name.
    if cmds.len() == 1 {
        // -> 'pub use MODULE::CmdX;\n'
        let (_, jref, _) = cmds.first().expect("bug");
        helpers::write_str(&mut out, issues, "\npub use ")?;
        helpers::write_string(
            &mut out,
            issues,
            helpers::qualify_name(
                &vec![names::command_module(*jref)],
                &names::command_run_struct(*jref),
            ),
        )?;
        helpers::write_str(&mut out, issues, ";\n")?;
    } else {
        // -> 'pub enum SubCommand {\n'
        // -> '    CmdX(MODULE::CmdX),\n
        // -> '}\n'
        helpers::write_str(&mut out, issues, "\npub enum SubCommand {\n")?;
        for (_, jref, _) in cmds {
            let struct_name = names::command_run_struct(jref);
            helpers::write_str(&mut out, issues, "    ")?;
            helpers::write_string_ref(&mut out, issues, &struct_name)?;
            helpers::write_str(&mut out, issues, "(")?;
            helpers::write_string(
                &mut out,
                issues,
                helpers::qualify_name(&vec![names::command_module(jref)], &struct_name),
            )?;
            helpers::write_str(&mut out, issues, "),\n")?;
        }
        helpers::write_str(&mut out, issues, "}\n")?;
    }

    Ok(())
}

/// Generate the command file.
fn gen_command_file(
    name: String,
    job_ref: structure::JobRef,
    cmd: Arc<collect::JobSource>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
    now: &String,
) -> Result<(), ()> {
    assert!(cmd.is_cmd);

    match &cmd.structure {
        collect::JobStructure::Inline(inline) => super::job_inline::gen_inline_job_file(
            create_settings(name, job_ref, &inline.source, &now),
            inline.clone(),
            collector,
            issues,
            out,
        ),
        collect::JobStructure::Module(module) => {
            // #1: ensure the module complies with the cmd requirements.
            let settings = create_settings(name, job_ref, &module.0.source, &now);
            let cmd_mod = match &module.1.command {
                Some(c) => c,
                None => {
                    // Does not support commands.
                    issues.add_err(errors::BuilderError::ModuleNotUsableForCommand(
                        errors::ErrorDetails {
                            message: (&module.1.name).clone(),
                            source: module.0.source.clone().into(),
                            related: Vec::new(),
                        },
                    ));
                    return Err(());
                }
            };
            // #2: create the job.
            let job_mod: structure::meta::JobModuleStruct = cmd_mod.into();
            super::job_mod::gen_module_job_file(
                settings,
                module.0.clone(),
                module.1.clone(),
                &job_mod,
                collector,
                issues,
                out,
            )
        }
        collect::JobStructure::Macro(mac) => {
            if !mac.1.supports_commands() {
                issues.add_err(errors::BuilderError::MacroNotUsableForCommand(
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
        collect::JobStructure::Stream(stream_job) => {
            // Should never happen.  Not a valid command type.
            issues.add_err(errors::BuilderError::InvalidLLS(errors::ErrorDetails {
                message: format!("command ({}) marked as stream - invalid command type", name),
                source: (&stream_job.source).into(),
                related: Vec::new(),
            }));
            Ok(())
        }
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
        parent_module: vec!["commands".to_string()],
        module_name: names::command_module(job_ref),
        run_struct_name: names::command_run_struct(job_ref),
        mod_struct_name: names::command_mod_struct(job_ref),
        now: now.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server_shell::{builder::lls_v1, lls::llsio};

    #[test]
    fn test_gen_command_index_1() {
        let mem_out = Arc::new(writer::MemSourceWriter::new());
        let issues = errors::ScriptIssues::new();
        let script =
            llsio::read_string_ref(&(SIMPLE_CMD_1.to_string())).expect("should compile fine");
        let col = collect::Collector::new(&script.meta);
        lls_v1::pass1(&script, &col, &issues);
        assert_eq!(issues.get(), Vec::new());

        gen_command_index(col.clone(), &issues, mem_out.clone(), &"|now|".to_string())
            .expect("should run fine");
        let cmd_file: String = String::from_utf8(
            mem_out
                .get_for("commands.rs")
                .expect("should have created file"),
        )
        .expect("bad utf-8 encoding");
        assert_eq!(
            cmd_file,
            r#"//SPDX:LIC
//! GENERATED FILE.  DO NOT EDIT.
//! Created on |now|

pub mod c0;

pub use c0::Cmd0Runner;
"#,
        );
    }

    const SIMPLE_CMD_1: &str = r#"{
        "schema-version": "1.0.0",
        "meta": {"name": "c1", "version": "2", "source": {"file": "f"}, "license": {"spdx-id": "LIC"}},
        "commands": {
            "one": {
                "source": {"file":"c"}, "thread":"0",
                "definition": {
                    "source": {"file":"cd"},
                    "kind": "module",
                    "module": "cat",
                    "rerunnable":true
                }
            }
        },
        "threads": {
            "0": {
                "source": {"file": "t"}, "steps": [
                  {"name": "0", "source": {"file": "0s"}, "run": {"kind": "noop"}}
                ]
            }
        },
        "jobs": {}
    }"#;

    #[test]
    fn test_gen_command_index_2() {
        let mem_out = Arc::new(writer::MemSourceWriter::new());
        let issues = errors::ScriptIssues::new();
        let script =
            llsio::read_string_ref(&(SIMPLE_CMD_2.to_string())).expect("should compile fine");
        let col = collect::Collector::new(&script.meta);
        lls_v1::pass1(&script, &col, &issues);
        assert_eq!(issues.get(), Vec::new());

        gen_command_index(col.clone(), &issues, mem_out.clone(), &"|now|".to_string())
            .expect("should run fine");
        let cmd_file: String = String::from_utf8(
            mem_out
                .get_for("commands.rs")
                .expect("should have created file"),
        )
        .expect("bad utf-8 encoding");
        assert_eq!(
            cmd_file,
            r#"//SPDX:LIC
//! GENERATED FILE.  DO NOT EDIT.
//! Created on |now|

pub mod c0;
pub mod c1;

pub enum SubCommand {
    Cmd0Runner(c0::Cmd0Runner),
    Cmd1Runner(c1::Cmd1Runner),
}
"#,
        );
    }

    const SIMPLE_CMD_2: &str = r#"{
        "schema-version": "1.0.0",
        "meta": {"name": "c1", "version": "2", "source": {"file": "f"}, "license": {"spdx-id": "LIC"}},
        "commands": {
            "one": {
                "source": {"file":"c1"}, "thread":"0",
                "definition": {
                    "source": {"file":"cd"},
                    "kind": "module",
                    "module": "cat",
                    "rerunnable":true
                }
            },
            "two": {
                "source": {"file":"2c"}, "thread":"0",
                "definition": {
                    "source": {"file":"ct"},
                    "kind": "module",
                    "module": "echo",
                    "rerunnable":true
                }
            }
        },
        "threads": {
            "0": {
                "source": {"file": "t"}, "steps": [
                  {"name": "0", "source": {"file": "0s"}, "run": {"kind": "noop"}}
                ]
            }
        },
        "jobs": {}
    }"#;
}
