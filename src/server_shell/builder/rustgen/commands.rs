//SPDX:MIT

use std::sync::Arc;
use std::thread;

use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;

/// Write the rust files related to commands.
/// Returns an empty result for quick returns.
pub fn gen_commands(
    issues: errors::ScriptIssues,
    collector: collect::Collector,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    // Each command can be written independently.
    let mut jobs = Vec::new();
    for (name, job_ref, cmd) in collector.ordered_jobs().iter() {
        if cmd.is_cmd {
            let name = name.clone();
            let job_ref = *job_ref;
            let c2 = cmd.clone();
            let i2 = issues.clone();
            let o2 = out.clone();
            jobs.push(thread::spawn(move || {
                let _ = gen_command_file(name, job_ref, c2, i2, o2);
            }));
        }
    }

    // Create the command index while the commands run.
    let _ = gen_command_index(collector, &issues, out);

    // Wait for the commands to complete.
    for j in jobs {
        issues.consume(
            j.join()
                .map_err(|e| errors::BuilderError::General(format!("{:?}", e))),
        );
    }
    Ok(())
}

fn gen_command_index(
    collector: collect::Collector,
    issues: &errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let out = issues.consume(out.writer_for("commands.rs").map_err(|e| e.into()))?;

    // Add in the 'pub mod ' line for each command.

    // If there is just one command:
    //   Create the 'CommandState'
    // If there are multiple commands:
    //   Add in the SubCommand enum
    todo!()
}

/// Generate the command file.
/// Commands don't need to lookup other jobs, so the collector isn't passed in.
fn gen_command_file(
    name: String,
    job_ref: structure::JobRef,
    cmd: Arc<collect::JobSource>,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    todo!()
}
