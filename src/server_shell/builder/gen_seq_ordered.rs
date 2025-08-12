//! Generate a seq*.rs file for a script.

use super::helpers::{as_mod_expr, rust_file_header};
use super::node_graph::NodeGraph;
use super::parse_node::{ModuleNode, NodeIndex};
use super::sequence::{SeqIndex, SequenceGen};
use super::writer::SourceWriter;
use crate::server_shell::ast::model;
use crate::server_shell::builder::errors::BuilderError;

/// Create a sequence file based on ordered actions.
pub fn write_ordered_seq<SW: SourceWriter>(
    seq_idx: SeqIndex,
    actions: &model::OrderedActions,
    out: &SW,
) -> Result<(), BuilderError> {
    let mut out = out.writer_for(&format!("src/seq{}.rs", seq_idx))?;
    out.write_all(rust_file_header().as_bytes())?;
    println!("TODO: write ordered sequence file {}", seq_idx);
    Ok(())
}
