//SPDX:MIT

//! Generate the streams argument for modules.

use std::collections::HashMap;
use std::ops::Deref;

use super::super::{collect, errors};
use super::{helpers, names};
use crate::server_shell::lls;
use crate::shell_lib::structure;

pub fn gen_streams_struct(
    mod_path: &Vec<String>,
    job_ref: structure::JobRef,
    streams_struct: &structure::meta::ModuleStreamStructure,
    streams: StreamsForJob,
    issues: &errors::ScriptIssues,
) -> String {
    let mut ret = String::new();
    ret.push_str(&helpers::qualify_name(mod_path, &streams_struct.name));
    ret.push_str(" { ");
    let mut first = true;

    // Fixed streams first.
    for fixed in &streams_struct.fixed_streams {
        if first {
            first = false;
        } else {
            ret.push_str(", ");
        }

        let (into, is_input) = match &fixed.stream_type {
            structure::meta::StreamType::Input(si) => match si {
                structure::meta::StreamInterface::ReadWrite => {
                    (".into::<Box<dyn std::io::Read + Send + Sync>>()", true)
                }
                structure::meta::StreamInterface::Fd => {
                    (".into::<std::io::Result<std::os::fd::OwnedFd>>()?", true)
                }
            },
            structure::meta::StreamType::Output(si) => match si {
                structure::meta::StreamInterface::ReadWrite => {
                    (".into::<Box<dyn std::io::Write + Send + Sync>>()", false)
                }
                structure::meta::StreamInterface::Fd => {
                    (".into::<std::io::Result<std::os::fd::OwnedFd>>()?", false)
                }
            },
        };

        // The FD takes precedence for the field name.
        if let Some(fd) = fixed.fd_index {
            ret.push_str(
                format!(
                    "fd_{}: self.{}.{}.state().take_stream{}_{}(){}",
                    fd,
                    names::RUNNER_RUNTIME_FIELD_NAME,
                    streams
                        .job_ref_for_fd(fd as i64, is_input)
                        .unwrap_or(999999),
                    job_ref,
                    fd,
                    into,
                )
                .as_str(),
            );
        } else if let Some(name) = &fixed.name {
            ret.push_str(
                format!(
                    "{}: self.{}.{}.state().take_stream{}_{}(){}",
                    name,
                    names::RUNNER_RUNTIME_FIELD_NAME,
                    streams.job_ref_for_name(name, is_input).unwrap_or(999999),
                    job_ref,
                    name,
                    into,
                )
                .as_str(),
            );
        } else {
            issues.add_err(errors::BuilderError::General(format!(
                "bug in module {} definition: no name or FD index",
                mod_path.join("::"),
            )));
        }
    }

    if let Some(input) = &streams_struct.input_variable {
        todo!()
    }

    if let Some(output) = &streams_struct.output_variable {
        todo!()
    }

    ret
}

/// Map the stream with this job to the stream job that contains that stream in its state.
pub struct DirectionStreams {
    fd: HashMap<i64, structure::JobRef>,
    named: HashMap<String, Vec<structure::JobRef>>,
}

pub struct StreamsForJob {
    input: DirectionStreams,
    output: DirectionStreams,
}

impl StreamsForJob {
    fn job_ref_for_fd(&self, fd: i64, is_input: bool) -> Option<structure::JobRef> {
        match is_input {
            true => &self.input,
            false => &self.output,
        }
        .fd
        .get(&fd)
        .map(|v| *v)
    }

    fn all_job_refs_for_name(&self, name: &String, is_input: bool) -> Vec<structure::JobRef> {
        match is_input {
            true => &self.input,
            false => &self.output,
        }
        .named
        .get(name)
        .map(|v| v.clone())
        .unwrap_or(Vec::new())
    }

    /// Return the job reference for the named stream if, and only if, exactly one stream registered for it.
    fn job_ref_for_name(&self, name: &String, is_input: bool) -> Option<structure::JobRef> {
        match match is_input {
            true => &self.input,
            false => &self.output,
        }
        .named
        .get(name)
        {
            Some(v) => {
                if v.len() == 1 {
                    v.first().map(|v| *v)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

pub fn find_streams_for_job(job_ref: structure::JobRef, col: &collect::Collector) -> StreamsForJob {
    let mut ret = StreamsForJob {
        input: DirectionStreams {
            fd: HashMap::new(),
            named: HashMap::new(),
        },
        output: DirectionStreams {
            fd: HashMap::new(),
            named: HashMap::new(),
        },
    };
    for (_name, other_job_ref, job) in col.ordered_jobs() {
        #[cfg(test)]
        assert!(!job.is_cmd);

        if let collect::JobStructure::Stream(s_job) = &job.structure {
            for s in &s_job.streams {
                stream_location(&s.from, &col, &mut ret.output, job_ref, other_job_ref);
                stream_location(&s.to, &col, &mut ret.input, job_ref, other_job_ref);
            }
        }
    }

    ret
}

fn stream_location(
    loc: &lls::model::StreamLocation,
    col: &collect::Collector,
    dir: &mut DirectionStreams,
    for_job: structure::JobRef,
    stream_job: structure::JobRef,
) {
    match loc {
        lls::model::StreamLocation::Fd { fd, job, source: _ } => {
            match col.get_job_ref(job) {
                Some(j) => {
                    if j == for_job {
                        dir.fd.insert(*(fd.deref()), stream_job);
                    }
                }

                // None means that the job wasn't found.  This situation should
                // be handled by the job_stream code, not here; otherwise, it would
                // the same source error would pop up multiple times.
                // Could add it as a 'here's where it's referenced'?
                None => (),
            }
        }
        lls::model::StreamLocation::Named {
            job,
            name,
            source: _,
        } => {
            match col.get_job_ref(job) {
                Some(j) => {
                    if j == for_job {
                        let name = name.deref().clone();
                        match dir.named.get_mut(&name) {
                            Some(v) => {
                                v.push(stream_job);
                            }
                            None => {
                                dir.named.insert(name, vec![stream_job]);
                            }
                        }
                    }
                }

                // None means that the job wasn't found.  This situation should
                // be handled by the job_stream code, not here; otherwise, it would
                // the same source error would pop up multiple times.
                // Could add it as a 'here's where it's referenced'?
                None => (),
            }
        }

        // files are built by the stream job, but not used directly by the job;
        // instead, the job consumes the other end of the stream.
        lls::model::StreamLocation::File {
            filename: _,
            mode: _,
            source: _,
        } => (),
    }
}
