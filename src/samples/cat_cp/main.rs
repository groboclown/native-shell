//! Manually constructed code to show how the builder might turn the AST into a shell program.

use std::collections::HashMap;

use crate::shell_lib::modules::{cat, file_sink, shell};
use crate::shell_lib::internal::scheduler;

pub fn main() {
    // Run the main function and handle any errors.
    let res = run_main();
    if let Err(e) = &res {
        eprintln!("Error: {}", e);
    }
    std::process::exit(res.unwrap_or(1));
}

fn run_main() -> Result<i32, String> {
    // Create the nodes that represent the modules.
    // This includes adding the compile-time / initial parameters.
    let nodes = Nodes {
        main: shell::ShellModule::new(shell::ShellModuleCompileParams {
            name: None,
            description: Some("Sends the contents of a file through a pipe into another file.".to_string()),
            version: Some("1.0.0".to_string()),
            authors: None,
            start_event: "start".to_string(),
            value_parameters: Some(vec!["source".to_string(), "target".to_string()]),
            boolean_parameters: None,
            position_parameter_min: None,
            position_parameter_max: None,
            usage_line: Some("--source=SOURCE --target=TARGET".to_string()),
            parameter_help: Some(HashMap::from([
                ("--source".to_string(), "The file to read from.".to_string()),
                ("--target".to_string(), "The file to write to.".to_string()),
            ])),
            start_help: None,
            end_help: Some(vec!["Copies the contents of the source file to the target file.  It does not create directories, but will overwrite the target file if it exists.".to_string()]),
        }),
        cat: cat::CatModule::new(),
        output: file_sink::FileSinkModule::new(),
    };

    // Create the runtime parameters for the modules.
    // Module execution can dynamically update these.
    let mut runtime_params = RuntimeParams {
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

    // Construct the job scheduler.
    // The meat of the AST builder goes here.
    // What this will look like:
    //    - The cat node links to the output, so that represents a job sequence.
    //    - A single node kicks off the job sequence by creating the stream between the two,
    //      and stores each half in separate job objects.  It also populates the runtime parameters.
    let scheduler = scheduler::run_state::Scheduler::new(
        vec![], // TODO: jobs
        vec![], // TODO: sequences
        vec![], // TODO: event groups
    );

    // Start the main node's run function, to start monitoring OS interactions.
    let (start_event, completion_tx) = nodes.main.run(Box::new(scheduler.context(0)))?;
    scheduler.add_global_completion_listener(completion_tx);
    let (tx, rx) = std::sync::mpsc::channel();
    scheduler.add_global_completion_listener(tx);

    scheduler.start_job_sequence_named(&start_event)?;

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
