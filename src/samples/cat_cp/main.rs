//SPDX:MIT

//! Manually constructed code to show how the builder might turn the LLS into a shell program.

use crate::shell_lib::structure::mod_impl::CommandHandler as _;
use crate::shell_lib::structure::mod_impl::CommandSetup as _;
use crate::shell_lib::{internal::scheduler, structure};
use std::{collections::HashMap, ffi::OsString, sync::Arc};

mod commands;
mod jobs;
mod modules;
mod threads;

pub fn main(argv: Vec<String>, environ: HashMap<OsString, OsString>) -> i32 {
    // Run the main function and handle any errors.
    env_logger::init();
    let res = match run_main(argv, environ) {
        Ok(e) => e,
        Err(e) => e,
    };
    if let Some(m) = &res.message {
        if res.code != 0 {
            eprintln!("Error: {}", m);
        } else {
            eprintln!("{}", m);
        }
    }
    std::process::exit(res.code);
}

fn run_main(
    argv: Vec<String>,
    environ: HashMap<OsString, OsString>,
) -> Result<structure::ScriptExit, structure::ScriptExit> {
    let mut event_builder = scheduler::eventbus::JobEventBuilder::new();

    // 1. Discover the sub-command to run.
    //    This script has just one command, 'default', so sub-commands are disabled.

    // 2. Create the job objects.
    let runtime = jobs::runtime::Runtime {
        jobs: Arc::new(jobs::runtime::Jobs {
            command: jobs::runtime::SubCommand::Cdefault(commands::c_default::Commanddefault::new(
                structure::Source::new("script.ns", 0, 0),
                event_builder.as_ctx(),
                commands::c_default::CommanddefaultParameters { argv },
            )),
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
            runtime.jobs.j0x0.runner(runtime.clone()), // job id 0: j0x0
            runtime.jobs.j0x1.runner(runtime.clone()), // job id 1: j0x1
        ],
        threads,
        bus,
    )?;
    Ok(runner.full_run(std::time::Duration::from_hours(24 * 365 * 1000), &runtime))
}
