//! Generate a seq*.rs file for a script.

use super::helpers::rust_file_header;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::BuilderError;
use crate::server_shell::builder::sequence::SeqIndex;
use crate::server_shell::lls::model;

/// Create a sequence file based on ordered actions.
pub fn write_ordered_seq<SW: SourceWriter>(
    seq_idx: SeqIndex,
    actions: &model::OrderedActions,
    out: &SW,
) -> Result<(), BuilderError> {
    let mut out = out.writer_for(&format!("src/seq{}.rs", seq_idx))?;
    out.write_all(rust_file_header().as_bytes())?;
    println!("TODO: write ordered sequence file {}", seq_idx);
    out.write_all(b"// TODO write ordered sequence file\n")?;
    Ok(())
}
