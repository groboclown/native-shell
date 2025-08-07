//! Manage sequences defined within nodes.

use std::cell::RefCell;

use super::errors;
use super::node_graph;
use super::parse_node;
use crate::server_shell::ast::model;

pub type SeqIndex = usize;

/// Adds a sequence to the generation.
pub trait SequenceGen<'a> {
    /// Mark an ordered action list as a sequence.
    /// This will cause the later generation of the sequence.
    fn add_ordered_sequence(&'a self, actions: &model::OrderedActions) -> SeqIndex;

    fn get_node_named(
        &'a self,
        source: &model::Source,
        name: &String,
    ) -> Result<&'a parse_node::ModuleNode, errors::BuilderError>;

    fn node_at(&'a self, index: parse_node::NodeIndex) -> &'a parse_node::ModuleNode;

    /// Find the sequence index responsible for running the named node.
    /// This will always be a graph sequence.
    fn seq_for_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<SeqIndex, errors::BuilderError>;

    /// Get the number of sequences stored so far.
    fn seq_count(&self) -> usize;

    fn seq_range(&self) -> std::ops::Range<SeqIndex>;
}

pub struct StdSequenceStore {
    nodes: Vec<parse_node::ModuleNode>,
    seq: RefCell<Vec<model::OrderedActions>>,
    graph: node_graph::ScriptGraph,
    start_seq_idx: SeqIndex,
}

impl StdSequenceStore {
    pub fn new(nodes: Vec<parse_node::ModuleNode>, graph: node_graph::ScriptGraph) -> Self {
        StdSequenceStore {
            nodes,
            seq: RefCell::new(Vec::new()),
            start_seq_idx: graph.graphs().len(),
            graph,
        }
    }

    /// Get the node graphs.
    pub fn graphs(&self) -> &Vec<node_graph::NodeGraph> {
        self.graph.graphs()
    }

    /// Return the ordered sequences along with the assigned sequence ID.
    pub fn ordered_sequences(&self) -> Vec<(SeqIndex, model::OrderedActions)> {
        let seq = self.seq.borrow();
        let mut ret = Vec::with_capacity(seq.len());
        for (idx, oa) in seq.iter().enumerate() {
            ret.push((idx + self.start_seq_idx, oa.clone()));
        }
        ret
    }
}

impl<'a> SequenceGen<'a> for StdSequenceStore {
    fn add_ordered_sequence(&'a self, actions: &model::OrderedActions) -> SeqIndex {
        let mut seq = self.seq.borrow_mut();
        let ret = seq.len() + self.start_seq_idx;
        seq.push(actions.clone());
        ret
    }

    fn get_node_named(
        &'a self,
        source: &model::Source,
        name: &String,
    ) -> Result<&'a parse_node::ModuleNode, errors::BuilderError> {
        self.graph.find_node_by_name(source, name, &self.nodes)
    }

    fn node_at(&'a self, index: parse_node::NodeIndex) -> &'a parse_node::ModuleNode {
        self.nodes.get(index).expect("bad indexing")
    }

    fn seq_for_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<SeqIndex, errors::BuilderError> {
        for (seq_idx, graph) in self.graph.graphs().iter().enumerate() {
            for node_idx in &graph.stream_order {
                let node = self.node_at(*node_idx);
                if &node.node.name == name {
                    return Ok(seq_idx);
                }
            }
        }
        Err(errors::BuilderError::NoSuchNode(errors::ErrorDetails {
            message: format!("no such node: '{}'", name),
            source: source.clone(),
            related: vec![],
        }))
    }

    fn seq_count(&self) -> usize {
        self.start_seq_idx + self.seq.borrow().len()
    }

    fn seq_range(&self) -> std::ops::Range<SeqIndex> {
        0..(self.start_seq_idx + self.seq.borrow().len() - 1)
    }
}
