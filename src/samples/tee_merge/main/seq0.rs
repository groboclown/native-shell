//! Job sequence 0.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

use crate::shell_lib::compile::job;
use crate::shell_lib::compile::source::Source;
use crate::shell_lib::modules::{cat, echo, merge, tee};
use crate::shell_lib::helpers;
use super::runtime;

pub struct Seq0StateInner {
    // Even though main's streams are used, they are handled via dup2 of the OS file descriptors.
    streams_data_file_1: Option<cat::CatModuleStream>,
    streams_data_file_2: Option<cat::CatModuleStream>,
    streams_data_text_1: Option<echo::EchoModuleStream>,
    streams_data_text_2: Option<echo::EchoModuleStream>,
    streams_tee1: Option<tee::TeeModuleStream>,
    streams_merge1: Option<merge::MergeModuleStream>,
    streams_merge2: Option<merge::MergeModuleStream>,
    streams_merge3: Option<merge::MergeModuleStream>,
}

pub type Seq0State = helpers::state_guard::StateGuard<Seq0StateInner>;

/// Create a new Seq0State with default values.
/// The JobSequence is managed by the main, as it embeds in it knowledge of the assigned
/// job index to the corresponding job.
pub fn new_seq0() -> Seq0State {
    helpers::state_guard::StateGuard::new(Seq0StateInner {
        streams_data_file_1: None,
        streams_data_file_2: None,
        streams_data_text_1: None,
        streams_data_text_2: None,
        streams_tee1: None,
        streams_merge1: None,
        streams_merge2: None,
        streams_merge3: None,
    })
}

/// The first job in the sequence: the connector between the nodes in sequence 0.
pub struct Seq0Job0 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job0 {
    fn run(&self, _context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        // Set up the streams for the nodes.
        match self.state.run_mut(|s| {
            // From the AST:
            //                          +----------------+
            //                          |                |
            //   stdout   <- merge3   <-+ merge1   <-+ <-+ tee1   <- file1 (as data_file_1)
            //                          |            + text1 (as data_text_1)
            //                          + merge2   <-+ file2 (as data_file_2)
            //                                       + text2 (as data_text_2)
            //                                       + stdin
            // Note that stdout, while it's defined as part of the main module, is a special case
            // (being a stream on the main module).  The builder uses the OS built-ins.

            // file1 (cat) -> tee1 (tee).
            //   fd -> fd
            let (p1r, p1w) = helpers::fd::mk_pipe();

            // tee1 (tee) -> merge1 (merge)
            //   writer -> fd
            let (p2r, p2w) = helpers::fd::mk_pipe();

            // text1 (echo) -> merge1 (merge).
            //   writer -> fd
            let (p3r, p3w) = helpers::fd::mk_pipe();

            // file2 (cat) -> merge2 (merge).
            //   fd -> fd
            let (p4r, p4w) = helpers::fd::mk_pipe();

            // tee1 (tee) -> merge3 (merge).
            //   writer -> fd
            let (p5r, p5w) = helpers::fd::mk_pipe();

            // merge1 (merge) -> merge3 (merge).
            //   writer -> fd
            let (p6r, p6w) = helpers::fd::mk_pipe();

            // merge2 (merge) -> merge3 (merge).
            //   writer -> fd
            let (p7r, p7w) = helpers::fd::mk_pipe();

            // text2 (echo) -> merge2 (merge).
            //   writer -> fd
            let (p8r, p8w) = helpers::fd::mk_pipe();

            s.streams_data_file_1 = Some(cat::CatModuleStream {
                fd_0: p1w,  // -> tee1
            });
            s.streams_data_file_2 = Some(cat::CatModuleStream {
                fd_0: p4w,  // -> merge2
            });
            s.streams_data_text_1 = Some(echo::EchoModuleStream {
                fd_0: Box::new(helpers::fd::file_from_fd(p3w)),  // -> merge1
            });
            s.streams_data_text_2 = Some(echo::EchoModuleStream {
                fd_0: Box::new(helpers::fd::file_from_fd(p8w)),  // -> merge2
            });
            s.streams_tee1 = Some(tee::TeeModuleStream {
                fd_0: p1r,  // <- file1
                output: vec![
                    Box::new(helpers::fd::file_from_fd(p2w)),  // -> merge1
                    Box::new(helpers::fd::file_from_fd(p5w)),  // -> merge3
                ],
            });
            s.streams_merge1 = Some(merge::MergeModuleStream {
                fd_0: Box::new(helpers::fd::file_from_fd(p6w)),  // -> merge3
                input: vec![
                    p2r,  // <- tee1
                    p3r,  // <- text1
                ],
            });
            s.streams_merge2 = Some(merge::MergeModuleStream {
                fd_0: Box::new(helpers::fd::file_from_fd(p7w)),  // -> merge3
                input: vec![
                    p4r,  // <- file2
                    p8r,  // <- text2

                    // Unsafe stdio extraction, but supposed to be safe because the Builder should
                    // only allow at most 1 stdin usage.
                    unsafe { OwnedFd::from_raw_fd(std::io::stdin().as_raw_fd()) },  // <- stdin
                ],
            });

            s.streams_merge3 = Some(merge::MergeModuleStream {
                fd_0: Box::new(std::io::stdout()),  // -> stdout
                input: vec![
                    p5r,  // <- tee1
                    p6r,  // <- merge1
                    p7r,  // <- merge2
                ],
            });

            Ok::<(), String>(())
        }) {
            helpers::state_guard::ExecState::Ran(Ok(())) => (),
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on Seq0State".to_string());
            }
            helpers::state_guard::ExecState::Ran(Err(e)) => {
                return Err(format!("Failed to update streams: {}", e));
            }
        };

        // Lookups used by runtime parameters happen outside the runtime.params.run_mut.
        // lookup-string-map(
        //    map: {node: constant-string "main", name: constant-string "value_params"},
        //    key: {constant-string "file1"},
        //    default: {constant-string ""},
        // )
        let main_state = self.runtime.nodes.main.state();
        let default_0 = String::new();
        let data_file_1_0 = main_state.value_params.get("file1").unwrap_or(&default_0);
        let data_file_2_0 = main_state.value_params.get("file2").unwrap_or(&default_0);
        let data_text_1_0 = main_state.value_params.get("text1").unwrap_or(&default_0);
        let data_text_2_0 = main_state.value_params.get("text2").unwrap_or(&default_0);

        // Set up the runtime parameters for the nodes.
        match self.runtime.params.run_mut(move |params| {
            params.data_file_1.filenames = vec![
                // Constant string list
                //  - value 1: data_file_1_0
                data_file_1_0.to_string(),
            ];
            params.data_file_2.filenames = vec![
                // Constant string list
                //  - value 1: data_file_2_0
                data_file_2_0.to_string(),
            ];
            params.data_text_1.text = data_text_1_0.to_string();
            params.data_text_2.text = data_text_2_0.to_string();
            params.merge1.separator = Some("+".to_string());
            params.merge1.stream_prefix = None;  // Not set
            params.merge1.max_record_length = None;  // Not set
            params.merge2.separator = Some("+".to_string());
            params.merge2.stream_prefix = None;  // Not set
            params.merge2.max_record_length = None;  // Not set
            params.merge3.separator = Some("+".to_string());
            params.merge3.stream_prefix = None;  // Not set
            params.merge3.max_record_length = None;  // Not set

            Ok::<(), String>(())
        }) {
            helpers::state_guard::ExecState::Ran(Ok(())) => (),
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on Runtime".to_string());
            }
            helpers::state_guard::ExecState::Ran(Err(e)) => {
                return Err(format!("Failed to set runtime parameters: {}", e));
            }
        };

        Ok(0)
    }

    fn abort(&self) -> Result<(), String> {
        // As job0 of a sequence, it does not represent a module, so has no abort.
        Ok(())
    }
}

impl Seq0Job0 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "@cat-output".to_string(),
            source: Source::new("script.ns", 1, 1),
            listen: None,
            runner: Box::new(Seq0Job0 { runtime, state }),
        }
    }
}


// Sequence 0, job 1.  The data_file_1 node.
pub struct Seq0Job1 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job1 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<cat::CatModuleRuntimeParams, String>(params.data_file_1.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<cat::CatModuleStream>, String>(state.streams_data_file_1.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.data_file_1.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.data_file_1.abort() {
            Err("Failed to abort data_file_1".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job1 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "data_file_1".to_string(),
            source: Source::new("script.ns", 2, 1),
            listen: None,
            runner: Box::new(Seq0Job1 { runtime, state }),
        }
    }
}


// Sequence 0, job 2.  The data_file_2 node.
pub struct Seq0Job2 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job2 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<cat::CatModuleRuntimeParams, String>(params.data_file_2.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<cat::CatModuleStream>, String>(state.streams_data_file_2.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.data_file_2.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.data_file_2.abort() {
            Err("Failed to abort data_file_2".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job2 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "data_file_2".to_string(),
            source: Source::new("script.ns", 3, 1),
            listen: None,
            runner: Box::new(Seq0Job2 { runtime, state }),
        }
    }
}


// Sequence 0, job 3.  The data_text_1 node.
pub struct Seq0Job3 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job3 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<echo::EchoModuleRuntimeParams, String>(params.data_text_1.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<echo::EchoModuleStream>, String>(state.streams_data_text_1.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.data_text_1.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.data_text_1.abort() {
            Err("Failed to abort data_text_1".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job3 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "data_text_1".to_string(),
            source: Source::new("script.ns", 4, 1),
            listen: None,
            runner: Box::new(Seq0Job3 { runtime, state }),
        }
    }
}


// Sequence 0, job 4.  The data_text_2 node.
pub struct Seq0Job4 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job4 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<echo::EchoModuleRuntimeParams, String>(params.data_text_2.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<echo::EchoModuleStream>, String>(state.streams_data_text_2.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.data_text_2.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.data_text_2.abort() {
            Err("Failed to abort data_text_2".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job4 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "data_text_2".to_string(),
            source: Source::new("script.ns", 5, 1),
            listen: None,
            runner: Box::new(Seq0Job4 { runtime, state }),
        }
    }
}


// Sequence 0, job 5.  The tee1 node.
pub struct Seq0Job5 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job5 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let streams = match self.state.run_mut(|state| { Ok::<Option<tee::TeeModuleStream>, String>(state.streams_tee1.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.tee1.exec(context, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.tee1.abort() {
            Err("Failed to abort tee1".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job5 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "tee1".to_string(),
            source: Source::new("script.ns", 6, 1),
            listen: None,
            runner: Box::new(Seq0Job5 { runtime, state }),
        }
    }
}


// Sequence 0, job 6.  The merge1 node.
pub struct Seq0Job6 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job6 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<merge::MergeModuleRuntimeParams, String>(params.merge1.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<merge::MergeModuleStream>, String>(state.streams_merge1.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.merge1.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.merge1.abort() {
            Err("Failed to abort merge1".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job6 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "merge1".to_string(),
            source: Source::new("script.ns", 7, 1),
            listen: None,
            runner: Box::new(Seq0Job6 { runtime, state }),
        }
    }
}


// Sequence 0, job 7.  The merge1 node.
pub struct Seq0Job7 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job7 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<merge::MergeModuleRuntimeParams, String>(params.merge2.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<merge::MergeModuleStream>, String>(state.streams_merge2.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.merge2.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.merge2.abort() {
            Err("Failed to abort merge2".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job7 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "merge2".to_string(),
            source: Source::new("script.ns", 8, 1),
            listen: None,
            runner: Box::new(Seq0Job7 { runtime, state }),
        }
    }
}


// Sequence 0, job 8.  The merge1 node.
pub struct Seq0Job8 {
    runtime: runtime::Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job8 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| Ok::<merge::MergeModuleRuntimeParams, String>(params.merge3.clone())) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| { Ok::<Option<merge::MergeModuleStream>, String>(state.streams_merge3.take()) }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.merge3.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        if ! self.runtime.nodes.merge3.abort() {
            Err("Failed to abort merge3".to_string())
        } else {
            Ok(())
        }
    }
}

impl Seq0Job8 {
    pub fn new_job(runtime: runtime::Runtime, state: Seq0State) -> job::JobDescription {
        job::JobDescription {
            name: "merge3".to_string(),
            source: Source::new("script.ns", 9, 1),
            listen: None,
            runner: Box::new(Seq0Job8 { runtime, state }),
        }
    }
}
