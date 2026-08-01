//SPDX:MIT

use std::ops::Deref;
use std::sync::Arc;

use serde_json::from_str;

use crate::server_shell::lls;
use crate::server_shell::lls::convert::*;
use crate::server_shell::meta;
use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;
use super::helpers;
use super::names;
use super::streams;

pub fn gen_stream_job_file(
    settings: super::jobs::JobSettings,
    job: Arc<lls::model::StreamJob>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let mut out = issues.consume(out.writer_for(&settings.job_file))?;
    let abort_event_ref = issues.consume(collector.mark_event_ref(
        &"abort".to_string(),
        &job.source,
        &structure::EventKind::Signal,
    ))?;

    helpers::write_string(
        &mut out,
        &issues,
        helpers::rust_file_header(&collector.meta, &settings.now),
    )?;

    // Assemble the set of streams, which this function will use to
    let streams: Vec<StreamSetup> = job
        .streams
        .iter()
        .map(|s| setup_stream(s, &collector, &issues))
        .filter(|v| v.is_some())
        .map(|v| v.expect("bug: filtered out none"))
        .collect();

    // Construct the "module" state data.
    // These have explicit semantics used elsewhere:
    //    take_stream{job_ref}_{fd | name}()
    // The return value is one of:
    //    Result<crate::shell_lib::stream::fd::FdStdio, structure::ScriptExit>
    //    Result<Box<std::io::Read>, structure::ScriptExit>
    //    Result<Box<std::io::Write>, structure::ScriptExit>
    // See the 'streams.rs' file for its use.
    // It must also have a 'open()' call (to repopulate the streams)
    // and a 'close()' call ("Drop" like but with error reporting).
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
#[derive(Clone)]
pub struct Job{}State {{
    inner: ::std::sync::Arc<::std::sync::Mutex<Job{}StateInner>>,
}}

struct Job{}StateInner {{"#,
            settings.job_ref, settings.job_ref, settings.job_ref,
        ),
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
    {}: Option<{}>,
    {}: Option<{}>,
"#,
                    s.name, s.rust_type, s.closer_name, s.closer_rust_type
                ),
            )?;
        }
    }
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"}}

pub struct Job{}Open {{"#,
            settings.job_ref,
        ),
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
    {}: {},
    {}: {},
"#,
                    s.name, s.rust_type, s.closer_name, s.closer_rust_type
                ),
            )?;
        }
    }
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"}}

impl Job{}State {{
    pub fn new() -> Self {{
        Self {{
            inner: Arc::new(Mutex::new(Job{}StateInner {{"#,
            settings.job_ref, settings.job_ref
        ),
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
                {}: None,
                {}: None,
"#,
                    s.name, s.closer_name
                ),
            )?;
        }
    }
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
            }})),
        }}
    }}

    pub fn open(&self, streams: Job{}Open) -> Result<(), crate::shell_lib::structure::ScriptExit> {{
        let mut inner = self.inner.lock()?;

        // Closed check.
"#,
            settings.job_ref,
        ),
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
        if inner.{}.is_some() {{
            return Err("not closed".into());
        }}
        if let Some({}) = inner.{}.take() {{
            {}.stop()?;
        }}
"#,
                    s.name, s.closer_name, s.closer_name, s.closer_name
                ),
            )?;
        }
    }

    helpers::write_str(&mut out, &issues, "\n        // Setup")?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
        inner.{} = Some(streams.{});
        inner.{} = Some(streams.{});
"#,
                    s.name, s.name, s.closer_name, s.closer_name
                ),
            )?;
        }
    }
    helpers::write_str(
        &mut out,
        &issues,
        r#"
        Ok(())
    }


    pub fn close(&self) -> Result<(), crate::shell_lib::structure::ScriptExit> {
        // If this can't lock it, exit early.
        let mut inner = self.inner.lock()?;

        let mut col = helpers::se_collect::ScriptExitCollector::new();
"#,
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
        if let Some({}) = inner.{}.take() {{
            col.add_result({}.stop());
        }}
"#,
                    s.closer_name, s.closer_name, s.closer_name
                ),
            )?;
        }
    }
    helpers::write_str(&mut out, &issues, "\n        col.close_ok()\n    }\n")?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
    pub fn take_{}_out(&self) -> Result<{}, crate::shell_lib::structure::ScriptExit> {{
        let mut inner = self.inner.lock()?;
        match inner.{}.take() {{
            None => Err("already took {}".into()),
            Some(s) => Ok(s),
        }}
    }}
"#,
                    s.name, s.rust_type, s.name, s.name
                ),
            )?;
        }
    }
    helpers::write_str(&mut out, &issues, "}\n")?;

    // Construct the Job structure.
    // It has a 'new()' without initialization parameters.
    //    This must register the 'abort' listener to close the state.
    // It has 'runner()' and 'state()' methods.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
pub struct {} {{
    state: ::std::sync::Arc<Job{}State>,
}}

impl {} {{
    pub fn new(ctx: &mut dyn crate::shell_lib::structure::InitCtx) -> Result<Self, crate::shell_lib::structure::ScriptExit> {{
        // Create the state.
        let state = Arc::new(Job{}State::new());

        // Register the abort handler.
        ctx.callback_on_name(
            "abort",
            &crate::shell_lib::structure::EventKind::Signal,
            Arc::new(Box::new({} {{
                state: state.clone(),
            }}) as Box<dyn crate::shell_lib::structure::EventCallback>),
        );

        Ok(Self {{ state }})
    }}

    pub fn runner(&self, runtime: runtime::Runtime) -> crate::shell_lib::structure::job::JobDescription {{
        crate::shell_lib::structure::job::JobDescription {{
            source: crate::shell_lib::structure::source::Resource {{
                name: {}.into(),
                kind: "stream-job".into(),
                source: crate::shell_lib::structure::Source::new({}, {}, {}),
            }},
            rerunable: true,
            runner: Box::new({} {{
                state: self.state.clone(),
                runtime,
            }}),
        }}
    }}

    pub fn state(&self) -> Job{}State {{
        self.state.as_ref().clone()
    }}
}}
"#,
            names::job_ref_mod_struct(settings.job_ref), // pub struct {} {{
            settings.job_ref,                            // state: ::std::sync::Arc<Job{}State>,
            names::job_ref_mod_struct(settings.job_ref), // impl {} {{
            settings.job_ref,                            // let state = Arc::new(Job{}State::new());
            names::event_handler_struct(settings.job_ref, abort_event_ref), // Arc::new(Box::new({} {{
            helpers::as_rust_str(&settings.name),                           // name: {}.into(),
            helpers::as_rust_str(&settings.source.file), // source: crate::shell_lib::structure::Source::new({}, {}, {}),
            settings.source.line.unwrap_or(0), // source: crate::shell_lib::structure::Source::new({}, {}, {}),
            settings.source.column.unwrap_or(0), // source: crate::shell_lib::structure::Source::new({}, {}, {}),
            names::job_ref_run_struct(settings.job_ref), // runner: Box::new({} {{
            settings.job_ref,                    // pub fn state(&self) -> Job{}State {{
        ),
    )?;

    // Construct the JobRunner structure.
    // Its 'run()' function constructs the streams and passes them to the
    // state's 'open()' method.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"

struct {} {{
    state: Arc<Job{}State>,
    runtime: runtime::Runtime,
}}

impl crate::shell_lib::structure::job::JobRunner for {} {{
    fn run(
        &self,
        _ctx: Box<dyn crate::shell_lib::structure::job::JobRunnerContext>,
    ) -> Result<crate::shell_lib::structure::ScriptExit, crate::shell_lib::structure::ScriptExit> {{
        // Create the streams.
"#,
            names::job_ref_run_struct(settings.job_ref), // struct {} {
            settings.job_ref,                            // state: Arc<modules::j0x0::Job{}State>,
            names::job_ref_run_struct(settings.job_ref), // impl crate::shell_lib::structure::job::JobRunner for {} {
        ),
    )?;
    for stream in &streams {
        helpers::write_string(
            &mut out,
            &issues,
            format!("        {}\n", stream.create_code),
        )?;
    }
    helpers::write_string(
        &mut out,
        &issues,
        format!("        self.state.open(Job{}Open {{", settings.job_ref),
    )?;
    for stream in &streams {
        for s in stream.all() {
            helpers::write_string(
                &mut out,
                &issues,
                format!(
                    r#"
            {}: {},
            {}: {},
"#,
                    s.name, s.name, s.closer_name, s.closer_name
                ),
            )?;
        }
    }
    helpers::write_str(
        &mut out,
        &issues,
        "        })?;\n        Ok(0.into())\n    }\n}",
    )?;

    // Construct the Abort event listener structure.
    helpers::write_string(
        &mut out,
        &issues,
        format!(
            r#"
struct {} {{
    state: ::std::sync::Arc<Job{}State>,
}}

impl crate::shell_lib::structure::EventCallback for {} {{
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b dyn crate::shell_lib::structure::ExecCtx,
        _event_ref: crate::shell_lib::structure::EventRef,
        _payload: &'c crate::shell_lib::structure::EventPayload,
    ) -> Result<(), crate::shell_lib::structure::ScriptExit> {{
        self.state.close()
    }}
}}
"#,
            names::event_handler_struct(settings.job_ref, abort_event_ref),
            settings.job_ref,
            names::event_handler_struct(settings.job_ref, abort_event_ref),
        ),
    )?;

    Ok(())
}

struct StreamSetup {
    out: Option<StreamDir>,
    inp: Option<StreamDir>,
    create_code: String,
}

impl StreamSetup {
    fn all<'a>(&'a self) -> Vec<&'a StreamDir> {
        let mut ret = Vec::new();
        if let Some(out) = &self.out {
            ret.push(out);
        }
        if let Some(inp) = &self.inp {
            ret.push(inp);
        }
        ret
    }
}

struct StreamDir {
    name: String,
    rust_type: String,
    closer_name: String,
    closer_rust_type: String,
}

/// Assemble the set of streams, which this function will use to fill in all the
/// code bits.
/// This part visits each possible combination.
fn setup_stream<'a, 'b, 'c>(
    stream: &'a lls::model::Stream,
    col: &'b collect::Collector,
    issues: &'c errors::ScriptIssues,
) -> Option<StreamSetup> {
    let to_stream = match parse_stream_location(
        &stream.to,
        structure::meta::StreamDirection::Input,
        col,
        issues,
    ) {
        Some(s) => s,
        None => {
            return None;
        }
    };
    let from_stream = match parse_stream_location(
        &stream.from,
        structure::meta::StreamDirection::Output,
        col,
        issues,
    ) {
        Some(s) => s,
        None => {
            return None;
        }
    };

    match &to_stream {
        DerefStream::File {
            filename: to_filename,
            mode: to_mode,
            direction: to_direction,
            source: to_source,
        } => match &from_stream {
            // --------------------------------------------------------
            // file <- file
            DerefStream::File {
                filename: from_filename,
                mode: from_mode,
                direction: from_direction,
                source: from_source,
            } => {
                // Can't have a from/to being both files.  That doesn't make sense.
                issues.add_err(errors::BuilderError::InvalidStreamUse(
                    errors::ErrorDetails {
                        message: "both to and from ends of the stream point to files".to_string(),
                        source: to_source.into(),
                        related: Vec::new(),
                    },
                ));
                None
            }
            // ----------------------------------------------------------------
            // file <- stream
            DerefStream::Stream {
                field_name: from_field_name,
                job_name: from_job_name,
                job_ref: from_job_ref,
                os_fd: from_os_fd,
                stream_type: from_stream_type,
                direction: from_direction,
                source: from_source,
            } => {
                let to_param = match lls::model::ActionParameter::try_from(
                    lls::model::ActionParameter::builder()
                        .source(to_source.clone())
                        .value(to_filename),
                ) {
                    Ok(p) => p,
                    Err(_) => {
                        return None;
                    }
                };
                let to_param = match super::values::conv_action_parameter(issues, col, &to_param) {
                    Ok(s) => match s {
                        Some((_, s)) => s,
                        None => {
                            return None;
                        }
                    },
                    Err(_) => {
                        return None;
                    }
                };
                match from_stream_type {
                    structure::meta::StreamInterface::ReadWrite => todo!(),
                    structure::meta::StreamInterface::FD => Some(StreamSetup {
                        inp: Some(StreamDir {
                            name: format!("stream{}_{}_out", from_job_ref, &from_field_name),
                            rust_type: "crate::shell_lib::stream::fd::FdStdio".to_string(),
                            closer_name: format!("stream{}_{}", from_job_ref, &from_field_name),
                            closer_rust_type: "crate::shell_lib::stream::fd::FdCloser".to_string(),
                        }),
                        out: None,
                        create_code: format!(
                            // TODO pass in the file mode.
                            r#"
        let (stream{}_{}, stream{}_{}_out) = stream::fd::FdOut::from_file(std::fs::File::open({})?).into_fd();
"#,
                            from_job_ref,
                            &from_field_name,
                            from_job_ref,
                            &from_field_name,
                            to_param,
                        ),
                    }),
                }
            }
        },
        DerefStream::Stream {
            field_name: to_field_name,
            job_name: to_job_name,
            job_ref: to_job_ref,
            os_fd: to_os_fd,
            stream_type: to_stream_type,
            direction: to_direction,
            source: to_source,
        } => match &from_stream {
            // --------------------------------------------------------
            // stream <- file
            DerefStream::File {
                filename: from_filename,
                mode: from_mode,
                direction: from_direction,
                source: from_source,
            } => {
                let from_param = match lls::model::ActionParameter::try_from(
                    lls::model::ActionParameter::builder()
                        .source(from_source.clone())
                        .value(from_filename),
                ) {
                    Ok(p) => p,
                    Err(_) => {
                        return None;
                    }
                };
                let from_param =
                    match super::values::conv_action_parameter(issues, col, &from_param) {
                        Ok(s) => match s {
                            Some((_, s)) => s,
                            None => {
                                return None;
                            }
                        },
                        Err(_) => {
                            return None;
                        }
                    };
                match to_stream_type {
                    structure::meta::StreamInterface::ReadWrite => todo!(),
                    structure::meta::StreamInterface::FD => Some(StreamSetup {
                        inp: Some(StreamDir {
                            name: format!("stream{}_{}_inp", to_job_ref, &to_field_name),
                            rust_type: "crate::shell_lib::stream::fd::FdStdio".to_string(),
                            closer_name: format!("stream{}_{}", to_job_ref, &to_field_name),
                            closer_rust_type: "crate::shell_lib::stream::fd::FdCloser".to_string(),
                        }),
                        out: None,
                        create_code: format!(
                            // TODO pass in the file mode.
                            r#"
        let (stream{}_{}, stream{}_{}_inp) = stream::fd::FdIn::from_file(std::fs::File::open({})?).into_fd();
"#,
                            to_job_ref, &to_field_name, to_job_ref, &to_field_name, from_param,
                        ),
                    }),
                }
            }
            DerefStream::Stream {
                field_name: from_field_name,
                job_name: from_job_name,
                job_ref: from_job_ref,
                os_fd: from_os_fd,
                stream_type: from_stream_type,
                direction: from_direction,
                source: from_source,
            } => todo!(),
        },
    }
}

fn job_ref_and_job<'a, 'b, 'c>(
    job_name: &String,
    source: &lls::model::Source,
    col: &collect::Collector,
    issues: &errors::ScriptIssues,
) -> Option<(structure::JobRef, Arc<collect::JobSource>)> {
    let job = match issues.add_result(col.get_job_checked(&job_name, source)) {
        Some(r) => r,
        None => {
            return None;
        }
    };
    let job_ref = col
        .get_job_ref(&job_name)
        .expect("bug: job exists but ref is None");
    Some((job_ref, job))
}

enum DerefStream {
    File {
        filename: lls::model::ComputedValue,
        mode: Option<lls::model::ComputedValue>,
        direction: structure::meta::StreamDirection,
        source: lls::model::Source,
    },
    Stream {
        field_name: String,
        job_name: String,
        job_ref: structure::JobRef,
        os_fd: Option<i64>, // commands have an implicit FD reflecting OS descriptors which must be dup'd.
        stream_type: structure::meta::StreamInterface,
        direction: structure::meta::StreamDirection,
        source: lls::model::Source,
    },
}

/// Parse the stream location as defined in the stream job.
/// The direction indicates the direction in the stream job
/// (from == output / writer, to == input / reader).
/// This finds the corresponding stream for the job.
fn parse_stream_location(
    loc: &lls::model::StreamLocation,
    direction: structure::meta::StreamDirection,
    col: &collect::Collector,
    issues: &errors::ScriptIssues,
) -> Option<DerefStream> {
    match loc {
        lls::model::StreamLocation::Fd {
            fd,
            job: job_name,
            source,
        } => {
            let fd: i64 = *(fd.deref());
            let job_name: &String = &job_name;

            if job_name.as_str() == super::lookup::ANY_COMMAND_NAME {
                // Special case for command FD.
                // Commands have an implicit set of FDs that come from the owning process.
                // The directions are from the user's perspective: a user
                // types (writes) into stdin, and views (reads) from stdout/err.
                let stream_dir = match fd {
                    0 => structure::meta::StreamDirection::Output, // stdin
                    1 => structure::meta::StreamDirection::Input,  // stdout
                    2 => structure::meta::StreamDirection::Input,  // stderr
                    other => {
                        issues.add_err(errors::BuilderError::InvalidStreamUse(errors::ErrorDetails {
                            message: format!("commands only allow FD values 0 (stdin), 1 (stdout), and 2 (stderr), found {}.", other),
                            source: source.into(),
                            related: Vec::new(),
                        }));
                        return None;
                    }
                };
                if !matches!(stream_dir.clone(), direction) {
                    issues.add_err(errors::BuilderError::InvalidStreamUse(errors::ErrorDetails {
                        message: format!(
                            "stream definition references FD {}, but that must be used as {:?}, but it is declared as {:?}",
                            fd, direction, stream_dir
                        ),
                        source: source.into(),
                        related: Vec::new(),
                    }));
                    return None;
                }
                return Some(DerefStream::Stream {
                    field_name: format!("fd_{}", fd),
                    job_name: job_name.clone(),
                    job_ref: 999999,
                    os_fd: Some(fd),
                    stream_type: structure::meta::StreamInterface::FD,
                    direction: stream_dir,
                    source: source.clone(),
                });
            }

            // The FD references a fixed stream in a job.
            let job = match issues.add_result(col.get_job_checked(job_name, source)) {
                Some(j) => j,
                None => {
                    return None;
                }
            };
            let job_ref = col
                .get_job_ref(job_name)
                .expect("bug: has job but not job ref");

            match &job.structure {
                collect::JobStructure::Inline(inline_job) => todo!(),
                collect::JobStructure::Module(module_job) => {
                    let job_streams = match get_job_streams(&module_job.1.job) {
                        Some(j) => j,
                        None => {
                            issues.add_err(errors::BuilderError::StreamNotFound(
                                errors::ErrorDetails {
                                    message: format!(
                                        "job {} (module {}) does not have FD stream {}",
                                        job_name, module_job.1.name, fd
                                    ),
                                    source: source.into(),
                                    related: Vec::new(),
                                },
                            ));
                            return None;
                        }
                    };
                    for stream in &job_streams.fixed_streams {
                        if let Some(j_fd) = stream.fd_index {
                            if j_fd == fd as usize {
                                // Found it!
                                let stream_interface = match &stream.stream_type {
                                    structure::meta::StreamType::Input(stream_interface) => {
                                        if !matches!(
                                            direction,
                                            structure::meta::StreamDirection::Input
                                        ) {
                                            issues.add_err(errors::BuilderError::InvalidStreamUse(errors::ErrorDetails {
                                                message: format!("stream definition references FD {}, but that must be used as input, but it is declared as output", fd),
                                                source: source.into(),
                                                related: Vec::new(),
                                            }));
                                            return None;
                                        }
                                        stream_interface
                                    }
                                    structure::meta::StreamType::Output(stream_interface) => {
                                        if !matches!(
                                            direction,
                                            structure::meta::StreamDirection::Output
                                        ) {
                                            issues.add_err(errors::BuilderError::InvalidStreamUse(errors::ErrorDetails {
                                                message: format!("stream definition references FD {}, but that must be used as output, but it is declared as input", fd),
                                                source: source.into(),
                                                related: Vec::new(),
                                            }));
                                            return None;
                                        }
                                        stream_interface
                                    }
                                };
                                return Some(DerefStream::Stream {
                                    field_name: format!("fd_{}", fd),
                                    job_name: job_name.clone(),
                                    job_ref,
                                    os_fd: None,
                                    stream_type: stream_interface.clone(),
                                    direction,
                                    source: source.clone(),
                                });
                            }
                        }
                    }
                    // Does not exist.
                    issues.add_err(errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                        message: format!(
                            "job {} (module {}) does not have FD stream {}",
                            job_name, module_job.1.name, fd
                        ),
                        source: source.into(),
                        related: Vec::new(),
                    }));
                    None
                }
                collect::JobStructure::Macro(macro_job) => todo!(),
                collect::JobStructure::Stream(stream_job) => todo!(),
                collect::JobStructure::Unknown => None,
            }
        }
        lls::model::StreamLocation::Named {
            job: job_name,
            name: stream_name,
            source,
        } => {
            let stream_name: &String = &stream_name;
            let job_name: &String = &job_name;
            let job = match issues.add_result(col.get_job_checked(job_name, source)) {
                Some(j) => j,
                None => {
                    return None;
                }
            };
            let job_ref = col
                .get_job_ref(job_name)
                .expect("bug: has job but not job ref");

            // TODO need to check whether the name references a FD stream alias.
            match &job.structure {
                collect::JobStructure::Inline(inline_job) => todo!(),
                collect::JobStructure::Module(module_job) => todo!(),
                collect::JobStructure::Macro(macro_job) => todo!(),
                collect::JobStructure::Stream(stream_job) => todo!(),
                collect::JobStructure::Unknown => None,
            }
        }
        lls::model::StreamLocation::File {
            filename,
            mode,
            source,
        } => {
            let filename: lls::model::ComputedValue = filename.into();
            let mode: Option<lls::model::ComputedValue> = match mode {
                Some(c) => Some(c.as_ref().into()),
                None => None,
            };
            Some(DerefStream::File {
                filename,
                mode,
                direction,
                source: source.clone(),
            })
        }
    }
}

fn get_job_streams(
    job: &Option<structure::meta::JobModuleStruct>,
) -> Option<&structure::meta::ModuleStreamStructure> {
    match job {
        Some(job) => match &job.stream_struct {
            Some(s) => Some(s),
            None => None,
        },
        None => None,
    }
}
