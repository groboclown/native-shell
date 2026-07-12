//! Generate the main.rs file for a script.

use super::helpers::rust_file_header;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::BuilderError;
use crate::server_shell::builder::sequence::SequenceGen;

/// Generate the CLI entrypoint.
pub fn write_main_rs<'a, SW: SourceWriter, SG: SequenceGen<'a>>(
    sgen: &'a SG,
    out: &SW,
) -> Result<(), BuilderError> {
    let mut out = out.writer_for("src/main.rs")?;
    out.write_all(rust_file_header().as_bytes())?;

    out.write_all(b"
mod shell_lib;
mod runtime;
")?;
    for seq in sgen.seq_range() {
        out.write_fmt(format_args!("mod seq{};\n", seq))?;
    }
    out.write_all(b"

use std::collections::HashMap;

pub fn main(argv: Vec<String>, environ: HashMap<String, String>) {
    // Run the main function and handle any errors.
    env_logger::init();
    shell_lib::helpers::exit::handle_exit(
        run_main(argv, environ),
    );
}

fn run_main(argv: Vec<String>, environ: HashMap<String, String>) -> Result<i32, String> {
    let runtime = runtime::Runtime::new(argv, environ);
")?;

    let main_seq_id = super::special::get_main_node_seq_id(sgen)?;
    let main_node = super::special::get_main_node(sgen)?;

    // Load the sequences into the scheduler.
    // Register event listeners.
    // Run the main node's 'start' function.
    // Setup the global completion listeners.
    // Run the main's start listener as a named signal event.

    // Collect the on-exit codes.
    // Exit with the highest exit code.

    out.write_all(b"\n    // FIXME this needs a full implementation.\n    Ok(0)\n")?;

    out.write_all(b"}\n")?;
    Ok(())
}
