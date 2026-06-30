//SPDX:MIT

//! Manually constructed code to show how the builder might turn the LLS into a shell program.

use crate::shell_lib::{internal::scheduler, structure};
use std::{collections::HashMap, sync::Arc};

mod jobs;
mod modules;
mod threads;

pub fn main(argv: Vec<String>, environ: HashMap<String, String>) -> i32 {
    // Run the main function and handle any errors.
    env_logger::init();
    let res = match run_main(argv, environ) {
        Ok(e) => e,
        Err(e) => e,
    };
    if let Some(m) = &res.message {
        if (res.code != 0) {
            eprintln!("Error: {}", m);
        } else {
            eprintln!("{}", m);
        }
    }
    std::process::exit(res.code);
}

fn run_main(
    argv: Vec<String>,
    environ: HashMap<String, String>,
) -> Result<structure::ScriptExit, structure::ScriptExit> {
    // 1. Parse the arguments based on the possible set of main threads.
    // TODO have a proper argument parser.
    let params = modules::default::JobdefaultParameters {
        source: argv.get(1).expect("required 'source'").into(),
        target: argv.get(2).expect("required 'target'").into(),
    };

    let mut event_builder = scheduler::eventbus::JobEventBuilder::new();

    // 2. Create the job objects.
    let runtime = jobs::runtime::Runtime {
        jobs: Arc::new(jobs::runtime::Jobs {
            default: jobs::default::Jobdefault::new(event_builder.as_ctx(), params)?,
            j0x0: jobs::j0x0::Jobj0x0::new(event_builder.as_ctx())?,
            j0x1: jobs::j0x1::Jobj0x1::new(event_builder.as_ctx())?,
        }),
    };

    // 4. Create the threads.
    let threads = threads::create_threads();

    // 5. Run it!
    let (_, bus) = event_builder.close();
    let mut runner = scheduler::runner::ScheduleRunner::new(
        vec![0], // start_threads; default == 0
        vec![
            // jobs, in order.  Note that the 'default' job isn't present.
            // 'default' is a main thread state, so never has a job associated with it.
            //
            // TODO the 'default' job should be a real job.
            //      It should run in parallel to every other job.
            //      It installs OS signal listeners, registers stdin and stdout and stderr
            //      as its streams, and so on.
            //
            runtime.jobs.j0x0.runner(runtime.clone()), // job id 0: j0x0
            runtime.jobs.j0x1.runner(runtime.clone()), // job id 1: j0x1
        ],
        threads,
        bus,
    )?;
    Ok(runner.run(std::time::Duration::from_mins(100)))

    // TODO:
    //    This should collect the 'run' result.
    //    Then call the 'abort' event.
    //    Then call runner.wait_for_jobs()
}
