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

    out.write_all(b"\nmod runtime;\n")?;
    for seq in sgen.seq_range() {
        out.write_fmt(format_args!("mod seq{};\n", seq))?;
    }

    out.write_all(b"\n\nfn main() {}\n")?;

    todo!("implement")
}
