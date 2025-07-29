//! Manually constructed code to show how the builder might turn the AST into a shell program.

use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::usize;

use crate::shell_lib::compile::{job, source};
use crate::shell_lib::compile::source::Source;
use crate::shell_lib::helpers;
use crate::shell_lib::modules::{cat, echo, merge, shell, tee};
use crate::shell_lib::internal::scheduler;
use crate::shell_lib::runtime::event_bus;

mod runtime;
mod seq0;
mod seq1;


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
    let runtime = runtime::Runtime {
        // Create the nodes that represent the modules.
        // This includes adding the compile-time / initial parameters.
        nodes: Arc::new(runtime::Nodes {
            main: shell::ShellModule::new(source::Source::new("script.ns", 1, 1), shell::ShellModuleCompileParams {
                // TODO this is all cut-and-paste and needs to be made for the ast.
                name: None,
                description: Some("Sends the contents of files and text and stdin into stdout.".to_string()),
                version: Some("1.0.0".to_string()),
                authors: None,
                start_event: "start".to_string(),
                required_value_parameters: Some(vec!["file1".to_string(), "file2".to_string(), "text1".to_string(), "text2".to_string()]),
                optional_value_parameters: None,
                boolean_parameters: None,
                position_parameter_min: None,
                position_parameter_max: None,
                usage_line: Some("--file1=SOURCE --file2=SOURCE --text1=TEXT --text2=TEXT".to_string()),
                parameter_help: Some(HashMap::from([
                    ("--file1".to_string(), "(required) The file to read from.".to_string()),
                    ("--file2".to_string(), "(required) The file to read from.".to_string()),
                    ("--text1".to_string(), "(required) Text to use as a source.".to_string()),
                    ("--text2".to_string(), "(required) Text to use as a source.".to_string()),
                ])),
                start_help: None,
                end_help: Some(vec!["Performs merge of five pieces of data across multiple manipulations (stdin is also read).  Uses '+' as a record separator.".to_string()]),
                argv: Some(argv),
                environ: Some(environ),
            }),
            data_file_1: cat::CatModule::new(source::Source::new("script.ns", 2, 1)),
            data_file_2: cat::CatModule::new(source::Source::new("script.ns", 3, 1)),
            data_text_1: echo::EchoModule::new(source::Source::new("script.ns", 4, 1)),
            data_text_2: echo::EchoModule::new(source::Source::new("script.ns", 5, 1)),
            tee1: tee::TeeModule::new(source::Source::new("script.ns", 6, 1)),
            merge1: merge::MergeModule::new(source::Source::new("script.ns", 7, 1)),
            merge2: merge::MergeModule::new(source::Source::new("script.ns", 8, 1)),
            merge3: merge::MergeModule::new(source::Source::new("script.ns", 9, 1)),
        }),

        // Create the runtime parameters for the modules.
        // Module execution can dynamically update these.
        params: helpers::state_guard::StateGuard::new(runtime::RuntimeParams {
            data_file_1: cat::CatModuleRuntimeParams {
                filenames: vec![],
            },
            data_file_2: cat::CatModuleRuntimeParams {
                filenames: vec![],
            },
            data_text_1: echo::EchoModuleRuntimeParams {
                text: String::new(),
            },
            data_text_2: echo::EchoModuleRuntimeParams {
                text: String::new(),
            },
            merge1: merge::MergeModuleRuntimeParams {
                separator: None,
                stream_prefix: None,
                max_record_length: None,
            },
            merge2: merge::MergeModuleRuntimeParams {
                separator: None,
                stream_prefix: None,
                max_record_length: None,
            },
            merge3: merge::MergeModuleRuntimeParams {
                separator: None,
                stream_prefix: None,
                max_record_length: None,
            },
        }),
    };

    let seq0_state = seq0::new_seq0();
    let scheduler = scheduler::run_state::Scheduler::new(
        vec![
            seq0::Seq0Job0::new_job(runtime.clone(), seq0_state.clone()), // 0
            seq0::Seq0Job1::new_job(runtime.clone(), seq0_state.clone()), // 1
            seq0::Seq0Job2::new_job(runtime.clone(), seq0_state.clone()), // 2
            seq0::Seq0Job3::new_job(runtime.clone(), seq0_state.clone()), // 3
            seq0::Seq0Job4::new_job(runtime.clone(), seq0_state.clone()), // 4
            seq0::Seq0Job5::new_job(runtime.clone(), seq0_state.clone()), // 5
            seq0::Seq0Job6::new_job(runtime.clone(), seq0_state.clone()), // 6
            seq0::Seq0Job7::new_job(runtime.clone(), seq0_state.clone()), // 7
            seq0::Seq0Job8::new_job(runtime.clone(), seq0_state.clone()), // 8
            // seq1 has no jobs of its own.
        ],
        vec![
            // Sequence 0
            job::JobSequenceDescription {
                // Node graph around data_file_1.
                name: "@seq0".to_string(),
                source: Source::new("script.ns", 1, 1),
                steps: vec![
                    // Start the first job, and wait for it to end.
                    job::ScheduleStep::SpawnJob(0),
                    job::ScheduleStep::WaitForJob(0, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),

                    // Start all jobs in the node graph.  As a node graph, they all run in parallel.
                    // The graph construction should also look at the exit behaviors, as these may
                    // mark that some jobs in the node graph must run after another completes.
                    job::ScheduleStep::SpawnJob(1),
                    job::ScheduleStep::SpawnJob(2),
                    job::ScheduleStep::SpawnJob(3),
                    job::ScheduleStep::SpawnJob(4),
                    job::ScheduleStep::SpawnJob(5),
                    job::ScheduleStep::SpawnJob(6),
                    job::ScheduleStep::SpawnJob(7),
                    job::ScheduleStep::SpawnJob(8),

                    // Implicitly wait for all jobs in the node graph to end.
                    job::ScheduleStep::WaitForJob(1, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(2, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(3, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(4, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(5, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(6, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(7, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                    job::ScheduleStep::WaitForJob(8, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                ],
            },

            // Sequence 1
            job::JobSequenceDescription {
                // The 'main' node event listener for 'start'.
                name: "event::main::start".to_string(),
                source: Source::new("script.ns", 1, 1),
                steps: vec![
                    // The action in the main -> start event handler is simply 'spawn-node data_file_1'.
                    // As this is a job within seq0, this triggers seq0 to run.
                    job::ScheduleStep::SpawnJobSequence(0),

                    // The entry event handler implies waiting for all sequences to end.
                    job::ScheduleStep::WaitForJobSequence(0, job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::RunNext,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }),
                ],
            }
        ],
        event_bus::with_default_event_groups(
            vec![
                job::EventGroup{ name: "start".to_string() },
            ],
        ),
    );

    // Register event listeners.
    scheduler.add_signal_event_listener(
        usize::MAX,  // Note that 'main' does not have a job id.
        &"start".to_string(),
        job::SignalEventHandler::new(
            vec![],
            1, // run sequence 1
        ),
    );

    // Start the main node's run function, to start monitoring OS interactions.
    let (completion_tx, on_exit) = mpsc::channel();
    let start_event = runtime.nodes.main.start(Box::new(scheduler.context(0)), on_exit)?;
    scheduler.add_global_completion_listener(completion_tx);
    let (tx, rx) = std::sync::mpsc::channel();
    scheduler.add_global_completion_listener(tx);

    // Start the execution by running the start event.
    scheduler.run_signal_event_named(usize::MAX, &start_event, 0)?;

    let codes = rx.recv().map_err(|e| format!("Failed to receive completion: {}", e))?;

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
