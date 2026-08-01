//SPDX:MIT

use std::ops::Deref;
use std::sync::Arc;

use crate::server_shell::lls;
use crate::server_shell::lls::convert::*;
use crate::server_shell::meta;
use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;
use super::helpers;
use super::names;

pub fn gen_stream_job_file(
    settings: super::jobs::JobSettings,
    job: Arc<lls::model::StreamJob>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let mut out = issues.consume(out.writer_for(&settings.job_file))?;
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
    inner: Arc<Mutex<Job{}StateInner>>,
}}

struct Job{}StateInner {{
"#,
            settings.job_ref, settings.job_ref, settings.job_ref,
        ),
    )?;

    // Construct the Job structure.
    // It has a 'new()' without initialization parameters.
    //    This must register the 'abort' listener to close the state.
    // It has 'runner()' and 'state()' methods.

    // Construct the JobRunner structure.
    // Its 'run()' function constructs the streams and passes them to the
    // state's 'open()' method.

    // Construct the Abort event listener structure.

    todo!()
}

struct StreamSetup {
    out: Option<StreamDir>,
    inp: Option<StreamDir>,
    create_code: String,
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
    // ----------------------------------------------------------------
    // from file -> something
    match &stream.from {
        lls::model::StreamLocation::File {
            filename,
            mode,
            source,
        } => {
            let filename: lls::model::ComputedValue = filename.into();
            let param = match lls::model::ActionParameter::try_from(
                lls::model::ActionParameter::builder()
                    .source(source.clone())
                    .value(filename),
            ) {
                Ok(p) => p,
                Err(_) => {
                    return None;
                }
            };
            let param = match super::values::conv_action_parameter(issues, col, &param) {
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
            match &stream.to {
                // --------------------------------------------------------
                // from file -> FD
                lls::model::StreamLocation::Fd { fd, job, source } => {
                    let to_job_ref = match issues.add_result(col.get_job_ref_checked(&job, &source))
                    {
                        Some(r) => r,
                        None => {
                            return None;
                        }
                    };
                    Some(StreamSetup {
                        inp: Some(StreamDir {
                            name: format!("stream{}_{}_inp", to_job_ref, &fd),
                            rust_type: "crate::shell_lib::stream::fd::FdStdio".to_string(),
                            closer_name: format!("stream{}_{}", to_job_ref, &fd),
                            closer_rust_type: "crate::shell_lib::stream::fd::FdCloser".to_string(),
                        }),
                        out: None,
                        create_code: format!(
                            // TODO pass in the file mode.
                            r#"
        let (stream{}_{}, stream{}_{}_out) = stream::fd::FdIn::from_file(std::fs::File::open({})?).into_fd();
"#,
                            to_job_ref, &fd, to_job_ref, &fd, param,
                        ),
                    })
                }
                // --------------------------------------------------------
                // from file -> named
                lls::model::StreamLocation::Named { job, name, source } => {
                    let (to_job_ref, to_job) = match job_ref_and_job(&job, &source, &col, &issues) {
                        Some(r) => r,
                        None => {
                            return None;
                        }
                    };
                    let name: &String = name.deref();
                    //   TODO the named stream might be a variable type, in which case this needs to
                    //        wrap the values in a vector.
                    //        It also means adding in some extra context.
                    Some(StreamSetup {
                        out: Some(StreamDir {
                            name: format!("stream{}_{}_out", to_job_ref, name),
                            rust_type: "crate::shell_lib::stream::fd::FdStdio".to_string(),
                            closer_name: format!("stream{}_{}", to_job_ref, name),
                            closer_rust_type: "crate::shell_lib::stream::fd::FdCloser".to_string(),
                        }),
                        inp: None,
                        create_code: format!(
                            // TODO pass in the file mode.
                            r#"
        let (stream{}_{}, stream{}_{}_out) = stream::fd::FdIn::from_file(std::fs::File::open({})?).into_fd();
"#,
                            to_job_ref, name, to_job_ref, name, param,
                        ),
                    })
                }
                lls::model::StreamLocation::File {
                    filename: _,
                    mode: _,
                    source,
                } => {
                    // Can't have a from/to being both files.  That doesn't make sense.
                    issues.add_err(errors::BuilderError::InvalidStreamUse(
                        errors::ErrorDetails {
                            message: "both to and from ends of the stream point to files"
                                .to_string(),
                            source: source.into(),
                            related: Vec::new(),
                        },
                    ));
                    None
                }
            }
        }
        lls::model::StreamLocation::Fd { fd, job, source } => todo!(),
        lls::model::StreamLocation::Named { job, name, source } => todo!(),
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
