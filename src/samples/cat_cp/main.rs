//! Manually constructed code to show how the builder might turn the AST into a shell program.
//! This shows the full connection in a single file.  Real scripts are expected to get very large,
//! and will be broken into multiple files.

use std::collections::HashMap;
use std::sync::{Arc, mpsc};

use crate::shell_lib::helpers;
use crate::shell_lib::helpers::state_guard::StateGuard;
use crate::shell_lib::internal::scheduler;
use crate::shell_lib::modules::{cat, file_sink, shell};
use crate::shell_lib::structure::source::Source;
use crate::shell_lib::structure::{job, source};

pub fn main(argv: Vec<String>, environ: HashMap<String, String>) -> i32 {
    // Run the main function and handle any errors.
    env_logger::init();
    let res = run_main(argv, environ);
    if let Err(e) = &res {
        eprintln!("Error: {}", e);
    }
    std::process::exit(res.unwrap_or(1));
}

fn run_main(argv: Vec<String>, environ: HashMap<String, String>) -> Result<i32, String> {
    // Create the nodes that represent the modules.
    // This includes adding the compile-time / initial parameters.
    let nodes = Nodes {
        main: shell::ShellModule::new(source::Source::new("script.ns", 1, 1), shell::ShellModuleCompileParams {
            name: None,
            description: Some("Sends the contents of a file through a pipe into another file.".to_string()),
            version: Some("1.0.0".to_string()),
            authors: None,
            start_event: "start".to_string(),
            required_value_parameters: Some(vec!["source".to_string(), "target".to_string()]),
            optional_value_parameters: None,
            boolean_parameters: None,
            position_parameter_min: None,
            position_parameter_max: None,
            usage_line: Some("--source=SOURCE --target=TARGET".to_string()),
            parameter_help: Some(HashMap::from([
                ("--source".to_string(), "(required) The file to read from.".to_string()),
                ("--target".to_string(), "(required) The file to write to.".to_string()),
            ])),
            start_help: None,
            end_help: Some(vec!["Copies the contents of the source file to the target file.  It does not create directories, but will overwrite the target file if it exists.".to_string()]),
            argv: Some(argv),
            environ: Some(environ),
        }),
        cat: cat::CatModule::new(source::Source::new("script.ns", 1, 1)),
        output: file_sink::FileSinkModule::new(source::Source::new("script.ns", 1, 1)),
    };

    // Create the runtime parameters for the modules.
    // Module execution can dynamically update these.
    let runtime_params = RuntimeParams {
        cat: cat::CatModuleRuntimeParams {
            // This will reach into the shell module's parameters to get the filename.
            filenames: vec![],
        },
        output: file_sink::FileSinkModuleRuntimeParams {
            // This will reach into the shell module's parameters to get the filename.
            filename: "output.txt".to_string(),
            // Hard-coded to false.
            append: Some(false),
        },
    };

    // Implementation will probably initialize all this in this single block.
    let runtime = Runtime {
        nodes: Arc::new(nodes),
        params: StateGuard::new(runtime_params),
    };

    // Construct the job groups from the node groups.
    let seq0_state = Seq0State::new(Seq0StateInner {
        streams_cat: None,
        streams_output: None,
    });

    // Construct the job scheduler.
    // This comes from splitting the AST into job groups based on discovering connected graphs.
    // What this will look like:
    //    - The cat node links to the output, so that represents a job sequence.
    //    - A single node kicks off the job sequence by creating the stream between the two,
    //      and stores each half in separate job objects.  It also populates the runtime parameters.
    let scheduler = scheduler::run_state::Scheduler::new(
        vec![
            // job0: coordinator for the cat -> file sink node group.
            job::JobDescription {
                name: "@cat-output".to_string(),
                source: Source::new("script.ns", 1, 1),
                listen: None,
                runner: Box::new(Seq0Job0 {
                    runtime: runtime.clone(),
                    state: seq0_state.clone(),
                }),
            },
            // job1: cat
            job::JobDescription {
                name: "cat".to_string(),
                source: Source::new("script.ns", 1, 1),
                listen: None,
                runner: Box::new(Seq0Job1 {
                    runtime: runtime.clone(),
                    state: seq0_state.clone(),
                }),
            },
            // job2: file sink
            job::JobDescription {
                name: "output".to_string(),
                source: Source::new("script.ns", 1, 1),
                listen: None,
                runner: Box::new(Seq0Job2 {
                    runtime: runtime.clone(),
                    state: seq0_state.clone(),
                }),
            },
        ],
        vec![
            // Job sequences represent two distinct types of action lists.
            // The first is a one-for-one with the OrderedAction type as
            // defined in the AST.
            // The second represents a node group, which all connect via
            // streams (minus shell output streams).  The sequence
            // has an initial job to construct the streams, then
            // one job per node execution to construct the runtime
            // parameters and run the job.
            // This isn't 100% accurate.  The event listeners must each
            // have their own sequence, but with no associated jobs.

            // seq0: cat -> file sink node group
            job::JobThreadDescription {
                name: "@seq0".to_string(),
                source: Source::new("script.ns", 1, 1),
                steps: vec![
                    // Start the coordinator job and wait for it to finish.
                    // Automatically added, and not part of the AST.
                    job::ScheduleStep::SpawnJob(0),
                    job::ScheduleStep::WaitForJob(
                        0,
                        job::ExitBehavior {
                            never_started: job::OnExitBehavior::AbortScript,
                            exit_code_behaviors: vec![job::ExitCodeRangeBehavior {
                                code: job::ExitCodeRange::Exact(0),
                                behavior: job::OnExitBehavior::RunNext,
                            }],
                            default_behavior: job::OnExitBehavior::AbortScript,
                        },
                    ),
                    // AST defines the sequence as spawning the cat node and waiting for
                    // the cat node to finish.
                    // This is where the complex logic of the builder comes into play.
                    // Because job sequences represent a job group, when an action requests
                    // starting one job in the group, all jobs in the group are started.
                    // Likewise, the job sequence waits for all jobs in the group to finish.
                    // It's possible for a sequence to wait on a job in another group,
                    // in which case it only waits on that one job.
                    job::ScheduleStep::SpawnJob(1),
                    job::ScheduleStep::SpawnJob(2),
                    job::ScheduleStep::WaitForJob(
                        1,
                        job::ExitBehavior {
                            never_started: job::OnExitBehavior::AbortScript,
                            exit_code_behaviors: vec![job::ExitCodeRangeBehavior {
                                code: job::ExitCodeRange::Exact(0),
                                behavior: job::OnExitBehavior::RunNext,
                            }],
                            default_behavior: job::OnExitBehavior::AbortScript,
                        },
                    ),
                    job::ScheduleStep::WaitForJob(
                        2,
                        job::ExitBehavior {
                            never_started: job::OnExitBehavior::AbortScript,
                            exit_code_behaviors: vec![job::ExitCodeRangeBehavior {
                                code: job::ExitCodeRange::Exact(0),
                                behavior: job::OnExitBehavior::RunNext,
                            }],
                            default_behavior: job::OnExitBehavior::AbortScript,
                        },
                    ),
                ],
            },
            // seq1: the 'main' node 'start' pseudo event listener.
            job::JobThreadDescription {
                name: "start".to_string(),
                source: Source::new("script.ns", 1, 1),
                steps: vec![
                    // The start has one action, which starts a single node.
                    // Starting a node means starting its associated node group,
                    // which is a job sequence.
                    // The scheduler will implicitly wait for it to stop async of
                    // the sequence that started it.
                    job::ScheduleStep::SpawnJobThread(0),
                ],
            },
        ],
        vec![], // TODO: event groups
    );

    // TODO this should register the main event listeners.

    // Start the main node's run function, to start monitoring OS interactions.
    let (completion_tx, on_exit) = mpsc::channel();
    let start_event = runtime
        .nodes
        .main
        .start(Box::new(scheduler.context(0)), on_exit)?;
    scheduler.add_global_completion_listener(completion_tx);
    let (tx, rx) = std::sync::mpsc::channel();
    scheduler.add_global_completion_listener(tx);

    scheduler.start_job_sequence_named(&start_event)?;

    let codes = rx
        .recv()
        .map_err(|e| format!("Failed to receive completion: {}", e))?;

    let mut exit_code = 0;
    for code in codes {
        if let Some(code) = code {
            if code > exit_code {
                exit_code = code as i32;
            }
        }
    }
    Ok(exit_code)
}

/// All the nodes described by the AST.
struct Nodes {
    main: shell::ShellModule,
    cat: cat::CatModule,
    output: file_sink::FileSinkModule,
}

/// All the runtime parameters described by the AST.
struct RuntimeParams {
    // shell defines these as None
    cat: cat::CatModuleRuntimeParams,
    output: file_sink::FileSinkModuleRuntimeParams,
}

#[derive(Clone)]
struct Runtime {
    nodes: Arc<Nodes>,
    params: helpers::state_guard::StateGuard<RuntimeParams>,
}

struct Seq0StateInner {
    streams_cat: Option<cat::CatModuleStream>,
    streams_output: Option<file_sink::FileSinkModuleStream>,
}

type Seq0State = helpers::state_guard::StateGuard<Seq0StateInner>;

/// The first job in the sequence: the connector between the nodes.
struct Seq0Job0 {
    runtime: Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job0 {
    fn run(&self, _context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        // Regardless of the current stream state, overwrite it.
        let (r, w) = helpers::fd::mk_pipe();
        match self.state.run_mut(|state| {
            state.streams_cat.replace(cat::CatModuleStream { fd_0: w });
            state
                .streams_output
                .replace(file_sink::FileSinkModuleStream { fd_0: r });
            Ok::<(), String>(())
        }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(_) => {}
        }

        // Lookups happen outside the runtime.params.run_mut.

        // map-key-string:
        //    map: lookup-string-map (main, value_params)
        //    key: constant-string (source)
        //    default: ""
        let main_source = self
            .runtime
            .nodes
            .main
            .state()
            .value_params
            .get("source")
            .unwrap_or(&"".to_string())
            .clone();

        // map-key-string:
        //    map: lookup-string-map (main, value_params)
        //    key: constant-string (target)
        //    default: ""
        let main_target = self
            .runtime
            .nodes
            .main
            .state()
            .value_params
            .get("target")
            .unwrap_or(&"".to_string())
            .clone();
        match self.runtime.params.run_mut(move |params| {
            // Constant construction happens inside the runtime.params.run_mut.
            params.cat.filenames = vec![main_source];
            params.output.filename = main_target;
            params.output.append = Some(false);
            Ok::<(), String>(())
        }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(_) => {}
        }

        Ok(0)
    }

    fn abort(&self) -> Result<(), String> {
        // Nothing to abort.
        Ok(())
    }
}

// The first job sequence, the 'cat' node.
struct Seq0Job1 {
    runtime: Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job1 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self
            .runtime
            .params
            .run_mut(|params| Ok::<cat::CatModuleRuntimeParams, String>(params.cat.clone()))
        {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self
            .state
            .run_mut(|state| Ok::<Option<cat::CatModuleStream>, String>(state.streams_cat.take()))
        {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.cat.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        let success = self.runtime.nodes.cat.abort();
        // May want to do this differently?
        if success {
            Ok(())
        } else {
            Err("Failed to abort cat job".to_string())
        }
    }
}

// The first job sequence, the 'output' node.
struct Seq0Job2 {
    runtime: Runtime,
    state: Seq0State,
}

impl job::JobRunner for Seq0Job2 {
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
        let params = match self.runtime.params.run_mut(|params| {
            Ok::<file_sink::FileSinkModuleRuntimeParams, String>(params.output.clone())
        }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on runtime parameters".to_string());
            }
            helpers::state_guard::ExecState::Ran(params) => params?,
        };
        let streams = match self.state.run_mut(|state| {
            Ok::<Option<file_sink::FileSinkModuleStream>, String>(state.streams_output.take())
        }) {
            helpers::state_guard::ExecState::LockContention => {
                return Err("Failed to acquire lock on state".to_string());
            }
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect("stream not found"),
        };
        self.runtime.nodes.output.exec(context, params, streams)
    }

    fn abort(&self) -> Result<(), String> {
        let success = self.runtime.nodes.output.abort();
        // May want to do this differently?
        if success {
            Ok(())
        } else {
            Err("Failed to abort cat job".to_string())
        }
    }
}
