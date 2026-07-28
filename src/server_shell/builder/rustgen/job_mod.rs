//SPDX:MIT

use std::collections::HashMap;
use std::sync::Arc;

use crate::server_shell::lls;
use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;
use super::helpers;

pub fn gen_module_job_file(
    settings: super::jobs::JobSettings,
    job: Arc<lls::model::ModuleJob>,
    module: Arc<structure::meta::ModuleMeta>,
    job_mod: &structure::meta::JobModuleStruct,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let mut out = issues.consume(
        out.writer_for(
            format!(
                "src/{}/{}.rs",
                settings.parent_module.join("/"),
                settings.module_name
            )
            .as_str(),
        )
        .map_err(|e| e.into()),
    )?;
    helpers::write_string(
        &mut out,
        &issues,
        helpers::rust_file_header(&collector.meta, &settings.now),
    )?;

    // 1. Define the Job.  It contains the module's state, if it has one.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            "\npub struct {} {{\n    state: ::std::sync::Arc<{}>,\n}}\n",
            settings.mod_struct_name,
            helpers::qualify_name(&module.mod_name, &job_mod.instance_struct),
        ),
    )?;

    // ----------------------------------------------------------------
    // 2. Create the job container.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
impl {} {{
    pub fn new(ctx: &mut dyn crate::shell_lib::structure::InitCtx) -> Result<Self, crate::shell_lib::structure::ScriptExit> {{
"#,
            settings.mod_struct_name
        ),
    )?;
    let empty_params = lls::model::InitialParameters::from(HashMap::new());
    let mut param_arg = "";
    if let Some(p) = &job_mod.compile_param_struct {
        param_arg = ", params";
        helpers::write_str(&mut out, &issues, "        let params = ")?;
        gen_param_struct(
            &mut out,
            &issues,
            &collector,
            &job.source,
            &module.mod_name,
            p,
            match &job.initial_parameters {
                Some(p) => p,
                None => &empty_params,
            },
            super::values::conv_initial_parameter,
        )?;
        helpers::write_str(&mut out, &issues, ";\n")?;
    }

    helpers::write_string(
        &mut out,
        &issues,
        format!(
            "        let state = ::std::sync::Arc::new({}::new(crate::shell_lib::structure::Source::new({}, {}, {}), &mut ctx{}));\n",
            helpers::qualify_name(&module.mod_name, &job_mod.instance_struct),
            helpers::as_rust_str(&((&job).source.file.clone().into())),
            job.source.line.unwrap_or(0).to_string(),
            job.source.column.unwrap_or(0).to_string(),
            param_arg,
        ),
    )?;

    for (idx, handler) in job_mod.handlers.iter().enumerate() {
        // Register each handler.
        helpers::write_string(
            &mut out,
            &issues,
            format!(
                r#"
        ctx.callback_on_name(
            {},
            &crate::shell_lib::structure::EventKind::{},
            ::std::sync::Arc::new(Box::new({}(state.clone()))),
        );
"#,
                helpers::as_rust_str(&handler.event_name),
                match handler.kind {
                    structure::EventKind::Message => "Message",
                    structure::EventKind::Signal => "Signal",
                },
                super::names::event_handler_struct(settings.job_ref, idx),
            ),
        )?;
    }

    helpers::write_str(
        &mut out,
        &issues,
        r#"
        Ok(Self { state })
    }
"#,
    )?;

    // ----------------------------------------------------------------
    // Create the state getter function.
    if let Some(s) = &job_mod.state_struct {
        helpers::write_string(
            &mut out,
            &issues,
            format!(
                r#"
    pub fn state(&self) -> ::std::sync::Arc<{}> {{
        self.state.state().clone()
    }}
"#,
                helpers::qualify_name(&module.mod_name, &s.name),
            ),
        )?;
    }

    // ----------------------------------------------------------------
    // Create the runner factory function.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
    pub fn runner(&self, runtime: super::runtime::Runtime) -> crate::shell_lib::structure::job::JobDescription {{
        crate::shell_lib::structure::job::JobDescription {{
            source: crate::shell_lib::structure::source::Resource {{
                name: {},
                kind: "module-job".into(),
                source: crate::shell_lib::structure::Source::new({}, {}, {}),
            }},
            rerunnable: {},
            runner: Box::new({} {{
                state: self.state.clone(),
                runtime,
            }}),
        }}
    }}
"#,
            helpers::as_rust_string(&settings.name),
            helpers::as_rust_str(&((&job).source.file.clone().into())),
            job.source.line.unwrap_or(0).to_string(),
            job.source.column.unwrap_or(0).to_string(),
            helpers::as_rust_bool(job.rerunnable),
            settings.run_struct_name,
        ),
    )?;

    // ----------------------------------------------------------------
    // Create the job runner structure.
    helpers::write_str(&mut out, &issues, "}\n\n")?;

    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
struct {} {{
    state: ::std::sync::Arc<{}>,
    runtime: super::runtime::Runtime,
}}

impl crate::shell_lib::structure::job::JobRunner for {} {{
    fn run(
        &self,
        ctx: Box<dyn crate::shell_lib::structure::job::JobRunnerContext>,
    ) -> Result<crate::shell_lib::structure::ScriptExit, crate::shell_lib::structure::ScriptExit> {{
        let mut ctx = crate::shell_lib::structure::JobRunnerCtx::new(ctx);
"#,
            settings.run_struct_name,
            helpers::qualify_name(&module.mod_name, &job_mod.instance_struct),
            settings.run_struct_name,
        ),
    )?;

    let empty_params = lls::model::NamedParameters::from(HashMap::new());
    let mut param_arg = "";
    if let Some(r) = &job_mod.runtime_param_struct {
        param_arg = ", params";
        helpers::write_str(&mut out, &issues, "        let params = ")?;
        gen_param_struct(
            &mut out,
            &issues,
            &collector,
            &settings.source,
            &module.mod_name,
            r,
            match &job.runtime_parameters {
                Some(r) => r,
                None => &empty_params,
            },
            super::values::conv_action_parameter,
        )?;
        helpers::write_str(&mut out, &issues, ";\n")?;
    }

    let mut stream_arg = "";
    if let Some(s) = &job_mod.stream_struct {
        stream_arg = ", streams";
        helpers::write_str(&mut out, &issues, "        let streams = ")?;
        // TODO add in stream parameter generation.
        helpers::write_str(&mut out, &issues, ";\n")?;
        todo!();
    }

    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
        self.state.exec(&mut ctx{}{})
    }}
}}
"#,
            param_arg, stream_arg
        ),
    )?;

    // ----------------------------------------------------------------
    // Create the event listener wrappers.
    for (idx, handler) in job_mod.handlers.iter().enumerate() {
        helpers::write_string(
            &mut out,
            &issues,
            format!(
                r#"

struct {}(::std::sync::Arc<{}>);

impl crate::shell_lib::structure::EventCallback for ErrorCb {{
    fn on<'a, 'b, 'c>(
        &'a self,
        context: &'b crate::shell_lib::structure::ExecCtx,
        event_ref: crate::shell_lib::structure::EventRef,
        payload: &'c crate::shell_lib::structure::EventPayload,
    ) -> Result<(), crate::shell_lib::structure::ScriptExit> {{
        self.0.{}(context, event_ref, match payload {{
            {}(v) => v,
            _ => {{
                return Err(crate::shell_lib::structure::ScriptExit::new(180, None));
            }}
        }})
    }}
}}
"#,
                super::names::event_handler_struct(settings.job_ref, idx),
                helpers::qualify_name(&module.mod_name, &job_mod.instance_struct),
                handler.func_name,
                match &handler.kind {
                    structure::EventKind::Message =>
                        "crate::shell_lib::structure::EventPayload::Message",
                    structure::EventKind::Signal =>
                        "crate::shell_lib::structure::EventPayload::Signal",
                },
            ),
        )?;
    }

    Ok(())
}

/// Generate the parameter structure construction.
fn gen_param_struct<K: Into<String> + Clone, V: Clone>(
    out: &mut Box<dyn std::io::Write>,
    issues: &errors::ScriptIssues,
    col: &collect::Collector,
    source: &lls::model::Source,
    mod_name: &Vec<String>,
    params: &structure::meta::ModuleStructure,
    values: &HashMap<K, V>,
    value_conv: fn(
        &errors::ScriptIssues,
        &collect::Collector,
        &V,
    ) -> Result<Option<(structure::meta::ValueType, String)>, ()>,
) -> Result<(), ()> {
    helpers::write_string(out, &issues, helpers::qualify_name(mod_name, &params.name))?;
    let use_field_name: bool;
    let block_end: &str;
    match &params.new {
        Some(new_name) => {
            // Create the 'new' using the public parameters as ordered arguments.
            helpers::write_string(out, &issues, format!("::{}(", new_name))?;
            block_end = ")";
            use_field_name = false;
        }
        None => {
            // Create the bare structure.
            helpers::write_str(out, &issues, " { ")?;
            block_end = " }";
            use_field_name = true;
        }
    }
    let mut remaining_user_fields = collect_keys(values);
    for field in &params.fields {
        match remaining_user_fields.remove(&field.name) {
            None => {
                if field.optional {
                    helpers::write_string(
                        out,
                        &issues,
                        format!(
                            "{}None, ",
                            match use_field_name {
                                true => format!("{}: ", field.name),
                                false => "".to_string(),
                            }
                        ),
                    )?;
                } else {
                    issues.add_err(errors::BuilderError::RequiredFieldMissing(
                        errors::ErrorDetails {
                            message: field.name.clone(),
                            source: source.clone().into(),
                            related: Vec::new(),
                        },
                    ));
                }
            }
            Some(val) => {
                let c_val = value_conv(issues, col, &val)?;
                match c_val {
                    None if !field.optional => {
                        issues.add_err(errors::BuilderError::FieldTypeMismatch(
                            errors::ErrorDetails {
                                message: field.name.clone(),
                                // TODO the source should come from the parameter value source,
                                //      but the type declaration makes that not possible.
                                // source: val.source.clone(),
                                source: source.into(),
                                related: Vec::new(),
                            },
                        ));
                        continue;
                    }
                    None => {
                        helpers::write_string(
                            out,
                            &issues,
                            format!(
                                "{}None, ",
                                match use_field_name {
                                    true => format!("{}: ", field.name),
                                    false => "".to_string(),
                                },
                            ),
                        )?;
                    }
                    Some((s_t, s_val)) => {
                        if s_t != field.value_type {
                            issues.add_err(errors::BuilderError::FieldTypeMismatch(
                                errors::ErrorDetails {
                                    message: field.name.clone(),
                                    // TODO the source should come from the parameter value source,
                                    //      but the type declaration makes that not possible.
                                    // source: val.source.clone(),
                                    source: source.into(),
                                    related: Vec::new(),
                                },
                            ));
                            continue;
                        }
                        helpers::write_string(
                            out,
                            &issues,
                            format!(
                                "{}{}{}{}, ",
                                match use_field_name {
                                    true => format!("{}: ", field.name),
                                    false => "".to_string(),
                                },
                                match field.optional {
                                    true => "Some(",
                                    false => "",
                                },
                                s_val,
                                match field.optional {
                                    true => ")",
                                    false => "",
                                },
                            ),
                        )?;
                    }
                }
                helpers::write_str(out, &issues, ", ")?;
                todo!();
            }
        }
    }
    helpers::write_str(out, &issues, block_end)?;
    Ok(())
}

fn collect_keys<K: Into<String> + Clone, V: Clone>(map: &HashMap<K, V>) -> HashMap<String, V> {
    let mut ret = HashMap::new();
    for (k, v) in map.iter() {
        ret.insert(k.clone().into(), v.clone());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server_shell::builder::{rustgen::jobs::JobSettings, writer::MemSourceWriter};
    use std::{io::Read, str::FromStr, sync::Arc};

    #[test]
    fn simplest_job() {
        let out = Arc::new(MemSourceWriter::new());
        let source = lls::model::Source {
            column: Some(1),
            file: lls::model::File::from_str("f.sh").unwrap(),
            line: Some(2),
            text: None,
        };
        let job = Arc::new(lls::model::ModuleJob {
            initial_parameters: None,
            kind: "module".to_string(),
            module: lls::model::ModuleJobModule::from_str("m1").unwrap(),
            rerunnable: true,
            runtime_parameters: None,
            source: source.clone(),
        });
        let job_mod = structure::meta::JobModuleStruct {
            instance_struct: "JM1".to_string(),
            state_struct: None,
            compile_param_struct: None,
            runtime_param_struct: None,
            stream_struct: None,
            handlers: Vec::new(),
        };
        let module = Arc::new(structure::meta::ModuleMeta {
            name: "m1".to_string(),
            description: "m one".to_string(),
            version: "1.0".to_string(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            os_dependencies: Vec::new(),
            mod_name: vec!["cm".to_string(), "test".to_string()],
            job: Some(job_mod.clone()),
            command: None,
        });

        let col = collect::Collector::new(&lls::model::Metadata {
            authors: Vec::new(),
            description: None,
            license: Some(lls::model::ScriptLicense {
                copyright: Some("no body".to_string()),
                spdx_id: lls::model::SpdxId::from_str("MIT").unwrap(),
            }),
            name: lls::model::ScriptName::from_str("s1").unwrap(),
            source: source.clone(),
            version: lls::model::ScriptVersion::from_str("0.1.0").unwrap(),
        });
        let issues = errors::ScriptIssues::new();
        gen_module_job_file(
            JobSettings {
                name: "job 1".to_string(),
                source: source.clone(),
                job_ref: 12,
                parent_module: vec!["jobs".to_string()],
                module_name: "m1".to_string(),
                run_struct_name: "Job1Runner".to_string(),
                mod_struct_name: "Job1".to_string(),
                now: "|now|".to_string(),
            },
            job,
            module,
            &job_mod,
            col.clone(),
            issues.clone(),
            out.clone(),
        )
        .unwrap();

        assert_eq!(issues.get(), Vec::new());

        let files = out.files.lock();
        let mut files = files.as_ref().unwrap().iter();
        let first = files.next().unwrap();
        assert!(files.next().is_none());
        assert_eq!(first.0, &"src/jobs/m1.rs".to_string());
        let mut out = String::new();
        first.1.as_slice().read_to_string(&mut out).unwrap();
        assert_eq!(
            out,
            r#"//SPDX:MIT
//! GENERATED FILE.  DO NOT EDIT.
//! Created on |now|


pub struct Job1 {
    state: ::std::sync::Arc<cm::test::JM1>,
}

impl Job1 {
    pub fn new(ctx: &mut dyn crate::shell_lib::structure::InitCtx) -> Result<Self, crate::shell_lib::structure::ScriptExit> {
        let state = ::std::sync::Arc::new(cm::test::JM1::new(crate::shell_lib::structure::Source::new("f.sh", 2, 1), &mut ctx));
        Ok(Self { state })
    }

    pub fn runner(&self, runtime: super::runtime::Runtime) -> crate::shell_lib::structure::job::JobDescription {
        crate::shell_lib::structure::job::JobDescription {
            source: crate::shell_lib::structure::source::Resource {
                name: "job 1".to_string(),
                kind: "module-job".into(),
                source: crate::shell_lib::structure::Source::new("f.sh", 2, 1),
            },
            rerunnable: true,
            runner: Box::new(Job1Runner {
                state: self.state.clone(),
                runtime,
            }),
        }
    }
}


struct Job1Runner {
    state: ::std::sync::Arc<cm::test::JM1>,
    runtime: super::runtime::Runtime,
}

impl crate::shell_lib::structure::job::JobRunner for Job1Runner {
    fn run(
        &self,
        ctx: Box<dyn crate::shell_lib::structure::job::JobRunnerContext>,
    ) -> Result<crate::shell_lib::structure::ScriptExit, crate::shell_lib::structure::ScriptExit> {
        let mut ctx = crate::shell_lib::structure::JobRunnerCtx::new(ctx);
        self.state.exec(&mut ctx)
    }
}
"#
            .to_string()
        );
    }
}
