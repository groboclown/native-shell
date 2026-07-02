//SPDX:MIT

//! Encapsulates the shell program that the OS interacts with.
//! It hosts the input and output streams to the OS (implicitly),
//! environment variables, and the command arguments.
//!
//! Its setup allows for configuring argument parsing and environment variable defaults.
//! It also constructs the signal handlers that send OS signals to the event bus.
//!
//! TODO document the input values.

use super::super::meta;

pub fn macro_meta() -> Box<dyn meta::MacroMeta> {
    Box::new(ShellMacro {})
}

pub struct ShellMacro {}

impl meta::MacroMeta for ShellMacro {
    fn supports_jobs(&self) -> bool {
        false
    }

    fn supports_commands(&self) -> bool {
        true
    }

    fn build_job(
        &self,
        _script: &super::super::lls::model::Metadata,
        _mod_name: Vec<String>,
        _input: serde_json::Value,
        _writer: &dyn std::io::Write,
    ) -> Result<meta::MacroModule, String> {
        Err("does not support building job".into())
    }

    fn build_command(
        &self,
        _script: &super::super::lls::model::Metadata,
        mod_name: Vec<String>,
        input: serde_json::Value,
        writer: &dyn std::io::Write,
    ) -> Result<meta::MacroModule, String> {
        // This will use 'clap' to construct a mod that parses the arguments.
        //   It requires adding the feature 'derive'.
        // This requires for non-Windows OS the 'signal_hook' crate.
        todo!()
    }
}

/*
    ModuleMeta {
        name: "shell".to_string(),
        description: "The shell program that interacts with the OS".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec![
            "shell_lib".to_string(),
            "modules".to_string(),
            "shell".to_string(),
        ],
        dependencies: vec![
            as_latest_crate_dependency("clap"),
            as_latest_crate_dependency("termion"),
            as_latest_crate_dependency("textwrap"),
            as_latest_crate_dependency("env_logger"),
            as_latest_crate_dependency("log"),
        ],
        os_dependencies: vec![],
        job: None,
        command: Some(CommandModuleStruct {
            instance_struct: "ShellModule".to_string(),
            compile_param_struct: Some(ModuleStructure {
                name: "ShellModuleCompileParams".to_string(),
                new: Some("new".to_string()),
                fields: vec![
                    NamedValue {
                        name: "name".to_string(),
                        value_type: ValueType::String,
                        optional: true,
                    },
                    NamedValue {
                        name: "description".to_string(),
                        value_type: ValueType::String,
                        optional: true,
                    },
                    NamedValue {
                        name: "version".to_string(),
                        value_type: ValueType::String,
                        optional: true,
                    },
                    NamedValue {
                        name: "authors".to_string(),
                        value_type: ValueType::StringList,
                        optional: true,
                    },
                    NamedValue {
                        // The 'usage' line.  Must not include a 'Usage:' prefix or the command name.
                        // TODO may not need this; will need to inspect what kind of CLI
                        // argument libraries exist.
                        name: "usage_line".to_string(),
                        value_type: ValueType::String,
                        optional: true,
                    },
                    NamedValue {
                        name: "start_help".to_string(),
                        value_type: ValueType::StringList,
                        optional: true,
                    },
                    NamedValue {
                        name: "end_help".to_string(),
                        value_type: ValueType::StringList,
                        optional: true,
                    },
                    NamedValue {
                        name: "parameters".to_string(),
                        value_type: ValueType::StringMapList,
                        optional: true,
                    },
                    // main() provided only; script users do not provide these.
                    NamedValue {
                        name: "argv".to_string(),
                        value_type: ValueType::StringList,
                        optional: false,
                    },
                    NamedValue {
                        name: "environ".to_string(),
                        value_type: ValueType::StringMap,
                        optional: false,
                    },
                ],
            }),

            // The main modules themselves don't declare the streams.  Instead,
            // the constructed job provides it.
            // BUT! need to double check that logic.
            //stream_struct: Some(ModuleStreamStructure {
            //    name: "ShellModuleStreams".to_string(),
            //    fixed_streams: vec![
            //        FixedStreamDef {
            //            name: Some("stdin".to_string()),
            //            fd_index: Some(0),
            //            // Other nodes read from the stdin provided by the shell.
            //            stream_type: StreamType::Output(StreamInterface::Fd),
            //            required: false,
            //        },
            //        FixedStreamDef {
            //            name: Some("stdout".to_string()),
            //            fd_index: Some(1),
            //            // The shell "consumes" stdout from other nodes.
            //            stream_type: StreamType::Input(StreamInterface::Fd),
            //            required: false,
            //        },
            //        FixedStreamDef {
            //            name: Some("stderr".to_string()),
            //            fd_index: Some(2),
            //            stream_type: StreamType::Input(StreamInterface::Fd),
            //            required: false,
            //        },
            //    ],
            //    input_variable: Some(VariableStreamField {
            //        field_name: "input_fds".to_string(),
            //        stream_type: StreamInterface::Fd,
            //        min_count: 0,
            //        max_count: u16::MAX,
            //    }),
            //    output_variable: Some(VariableStreamField {
            //        field_name: "output_fds".to_string(),
            //        stream_type: StreamInterface::Fd,
            //        min_count: 0,
            //        max_count: u16::MAX,
            //    }),
            //}),
            state_struct: Some(ModuleStructure {
                name: "ShellModuleState".to_string(),
                new: Some("new".to_string()),
                fields: vec![
                    NamedValue {
                        name: "environ".to_string(),
                        value_type: ValueType::StringMap,
                        optional: false,
                    },
                    NamedValue {
                        name: "value_params".to_string(),
                        value_type: ValueType::StringMap,
                        optional: false,
                    },
                    NamedValue {
                        name: "bool_params".to_string(),
                        value_type: ValueType::BooleanMap,
                        optional: false,
                    },
                    NamedValue {
                        name: "position_params".to_string(),
                        value_type: ValueType::StringList,
                        optional: false,
                    },
                ],
            }),
            handlers: vec![
                // The logging handlers.
                EventFunc::imm_msg("trace"),
                EventFunc::imm_msg("debug"),
                EventFunc::imm_msg("verbose"),
                EventFunc::imm_msg("info"),
                EventFunc::imm_msg("notice"),
                EventFunc::imm_msg("warning"),
                EventFunc::imm_msg("error"),
            ],
        }),
    }
}

pub struct ShellModuleCompileParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub usage_line: Option<String>,
    pub start_help: Option<Vec<String>>,
    pub end_help: Option<Vec<String>>,

    /// Defines
    pub parameters: Vec<HashMap<String, String>>,

    /// Parameter for the main program to pass to this module the command-line arguments.
    pub argv: Vec<String>,

    /// Parameter for the main program to pass to this module the environment variables.
    pub environ: HashMap<String, String>,
}

pub struct ShellModuleStreams {
    pub fd_0: std::os::fd::OwnedFd,
    pub fd_1: std::os::fd::OwnedFd,
    pub fd_2: std::os::fd::OwnedFd,
    pub input_fds: Vec<std::os::fd::OwnedFd>,
    pub output_fds: Vec<std::os::fd::OwnedFd>,
}

#[derive(Clone, Debug)]
pub struct ShellModuleState {
    pub string_parameters: std::collections::HashMap<String, String>,
    pub list_parameters: HashMap<String, Vec<String>>,
    pub flag_parameters: HashMap<String, u16>,
    pub problem: Option<ScriptExit>,
}

pub struct ShellModule {
    source: Source,
    state: ShellModuleState,
}

impl ShellModule {
    pub fn new(
        source: Source,
        ctx: &mut dyn InitCtx,
        compile_params: ShellModuleCompileParams,
    ) -> Self {
        let width = termion::terminal_size().map_or(80, |(w, _)| w as usize);
        let state = parse_params(compile_params, &mut std::io::stderr(), width);
        ShellModule { state, source }
    }

    pub fn state(&self) -> ShellModuleState {
        self.state.clone()
    }

    /// Event listener
    pub fn trace(
        &self,
        ctx: &mut dyn ExecCtx,
        _: EventRef,
        msg: &String,
    ) -> Result<(), ScriptExit> {
        log::trace!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn debug(
        &self,
        ctx: &mut dyn ExecCtx,
        _: EventRef,
        msg: &String,
    ) -> Result<(), ScriptExit> {
        log::debug!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn verbose(
        &self,
        ctx: &mut dyn ExecCtx,
        _: EventRef,
        msg: &String,
    ) -> Result<(), ScriptExit> {
        log::info!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn info(&self, _: &mut dyn ExecCtx, _: EventRef, msg: &String) -> Result<(), ScriptExit> {
        log::info!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn notice(&self, _: &mut dyn ExecCtx, _: EventRef, msg: &String) -> Result<(), ScriptExit> {
        // TODO notice is supposed to be a report-only-once thing, so this should
        // cache the messages and ensure it doesn't send duplicates.
        log::error!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn warning(
        &self,
        _: &mut dyn ExecCtx,
        _: EventRef,
        msg: &String,
    ) -> Result<(), ScriptExit> {
        log::warn!("{}", msg);
        Ok(())
    }

    /// Event listener
    pub fn error(&self, _: &mut dyn ExecCtx, _: EventRef, msg: &String) -> Result<(), ScriptExit> {
        log::error!("{}", msg);
        Ok(())
    }
}

struct ParameterArgs {
    params: ShellModuleCompileParams,
    state: ShellModuleState,
}

impl clap::FromArgMatches for ShellModuleState {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {}

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        todo!()
    }
}

impl clap::Args for ShellModuleState {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        todo!()
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        todo!()
    }
}

/// Parse command line arguments + environment variables.
fn parse_params<W: std::io::Write>(
    params: ShellModuleCompileParams,
    out: &mut W,
    width: usize,
) -> ShellModuleState {
    let mut problems: Vec<String> = vec![];
    let mut requested_help = false;

    let mut args = params.argv.expect("main did not set argv").into_iter();
    let cmd_name = args.next().expect("Command name should always be present");

    if let Some(params) = &params.required_value_parameters {
        for name in params {
            value_param_names.insert(name.clone());
        }
    }
    if let Some(params) = &params.optional_value_parameters {
        for name in params {
            value_param_names.insert(name.clone());
        }
    }
    if let Some(params) = &params.boolean_parameters {
        for name in params {
            bool_param_names.insert(name.clone());
            bool_params.insert(name.clone(), false);
        }
    }

    loop {
        let arg = match args.next() {
            Some(a) => a,
            None => break,
        };
        if arg == "--help" || arg == "-h" {
            requested_help = true;
            continue;
        }
        if arg == "--version" || arg == "-V" {
            let _ = print_line(
                out,
                &format!(
                    "{} {}",
                    params.name.unwrap_or(cmd_name),
                    params.version.unwrap_or("0.0.0".to_string())
                ),
                width,
            );
            return Err(0);
        }
        if arg.starts_with("--") {
            // Long option
            let parts: Vec<&str> = arg[2..].splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                if value_param_names.contains(&name) {
                    value_params.insert(name.clone(), parts[1].to_string());
                } else if bool_param_names.contains(&name) {
                    problems.push(format!("Parameter cannot have value: {}", &name));
                } else {
                    problems.push(format!("Unknown parameter: {}", &name));
                }
            } else if parts.len() == 1 {
                let name = parts[0].to_string();
                if bool_param_names.contains(&name) {
                    bool_params.insert(name.clone(), true);
                } else if value_param_names.contains(&name) {
                    match args.next() {
                        Some(value) => value_params.insert(name.clone(), value),
                        None => {
                            problems.push(format!("Parameter {} requires a value", &name));
                            None
                        }
                    };
                } else {
                    problems.push(format!("Unknown parameter: {}", &name));
                }
            }
        } else {
            // Position parameter
            position_params.push(arg);
        }
    }
    let min_position_param_count = params.position_parameter_min.unwrap_or(0.0) as i64;
    let mut max_position_param_count = params.position_parameter_max.unwrap_or(0.0) as i64;
    if max_position_param_count < min_position_param_count {
        max_position_param_count = min_position_param_count;
    }
    let position_param_count = position_params.len() as i64;
    if position_param_count < min_position_param_count {
        problems.push(format!(
            "Too few position parameters: expected {}, found {}",
            min_position_param_count,
            position_params.len()
        ));
    }
    if position_param_count > max_position_param_count {
        problems.push(format!(
            "Too many position parameters: expected {}, found {}",
            max_position_param_count,
            position_params.len()
        ));
    }
    if let Some(params) = &params.required_value_parameters {
        for name in params {
            if !value_params.contains_key(name) {
                problems.push(format!("Required parameter not provided: --{}", name));
            }
        }
    }

    if requested_help {
        if let Some(lines) = params.start_help {
            let _ = print_lines(out, &lines, width);
            let _ = writeln!(out);
        }
        if let Some(desc) = params.description {
            let _ = print_line(out, &desc, width);
            let _ = writeln!(out);
        }
        if let Some(usage_line) = params.usage_line {
            let usage_line = format!("Usage: {} {}", cmd_name, usage_line);
            let _ = print_line(out, &usage_line, width);
        } else {
            let usage_line = format!("Usage: {}", cmd_name);
            let _ = print_line(out, &usage_line, width);
            let _ = print_str(out, "Where:", width);
        }
        if let Some(help) = params.parameter_help {
            let key_size = find_max_key_width(&help) + 2;
            for (param, help_text) in &help {
                let _ = print_key_val(out, param, &help_text, key_size, width);
            }
        } else {
            let key_col_width = find_max_width(&params.required_value_parameters)
                .max(find_max_width(&params.optional_value_parameters))
                .max(find_max_width(&params.boolean_parameters))
                + 2;
            let _ = print_key_val(out, "-h/--help", "Display this help", key_col_width, width);
            if let Some(params) = &params.required_value_parameters {
                for param in params {
                    let _ = print_key_val(out, &param, "<value> (required)", key_col_width, width);
                }
            }
            if let Some(params) = &params.optional_value_parameters {
                for param in params {
                    let _ = print_key_val(out, &param, "<value> (optional)", key_col_width, width);
                }
            }
            if let Some(params) = &params.boolean_parameters {
                let _ = print_lines(out, &params, width);
            }
        }
        if let Some(lines) = params.end_help {
            let _ = writeln!(out);
            let _ = print_lines(out, &lines, width);
        }
        return Err(0);
    }

    if !problems.is_empty() {
        // Note: does not display help unless explicitly asked.
        let _ = print_str(out, "Script invocation failed.", width);
        let _ = print_lines(out, &problems, width);
        let _ = print_str(
            out,
            "Run with '--help' for details on how to run the script.",
            width,
        );
        return Err(1);
    }

    Ok((value_params, bool_params, position_params))
}

fn print_lines<W: std::io::Write>(
    out: &mut W,
    lines: &Vec<String>,
    width: usize,
) -> std::io::Result<()> {
    for line in lines {
        print_line(out, line, width)?;
    }
    Ok(())
}

fn print_line<W: std::io::Write>(out: &mut W, line: &String, width: usize) -> std::io::Result<()> {
    print_str(out, line.as_str(), width)
}

fn print_str<W: std::io::Write>(out: &mut W, line: &str, width: usize) -> std::io::Result<()> {
    let wrapped = textwrap::wrap(line, width);
    for wrapped_line in wrapped {
        writeln!(out, "{}", wrapped_line)?;
    }
    Ok(())
}

fn print_key_val<W: std::io::Write>(
    out: &mut W,
    key: &str,
    val: &str,
    key_col_width: usize,
    width: usize,
) -> std::io::Result<()> {
    let key_width = key.len();
    let space = " ".repeat(key_col_width - key_width);
    let val_lines = textwrap::wrap(val, width - key_col_width - 2);
    if val_lines.is_empty() {
        writeln!(out, "{}", key)?;
    }
    writeln!(out, "{}{}{}", key, space, val_lines[0])?;
    let space = " ".repeat(key_col_width);
    for val in &val_lines[1..] {
        writeln!(out, "{}{}", space, val)?;
    }
    Ok(())
}

fn find_max_key_width(vals: &HashMap<String, String>) -> usize {
    vals.keys().map(|k| k.len()).max().unwrap_or(0)
}

fn find_max_width(vals: &Option<Vec<String>>) -> usize {
    match vals {
        Some(vals) => vals.iter().map(|v| v.len()).max().unwrap_or(0),
        None => 0,
    }
}
*/
