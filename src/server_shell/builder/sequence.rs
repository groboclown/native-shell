//! Manage sequences defined within nodes.

use std::cell::RefCell;

use super::errors;
use super::node_graph;
use super::assemble;
use crate::server_shell::lls::model;

pub type SeqIndex = usize;
pub type JobIndex = usize;

/// Adds a sequence to the generation.
pub trait SequenceGen<'a> {
    /// Mark an ordered action list as a sequence.
    /// This will cause the later generation of the sequence.
    fn add_ordered_sequence(&'a self, actions: &model::OrderedActions) -> SeqIndex;

    /// Mark the existence of a "job0" for a graph sequence, and create its associated global job index.
    fn gen_graph_seq_job0(&'a self, seq_idx: SeqIndex) -> JobIndex;

    /// Set the ordered actions that run a particular node, within the
    /// node's graph sequence.  This will generate a sub-sequence with its own sequence ID.
    /// The seq_index and seq_job_idx must align between the script generator that produces
    /// the job runner implementing structure and the script generator that creates the
    /// new instance in the job list.
    fn set_node_execution_sequence(
        &'a self,
        node_idx: assemble::NodeIndex,
        actions: &model::OrderedActions,
        seq_idx: SeqIndex,
        seq_job_idx: usize,
    ) -> (SeqIndex, JobIndex);

    fn get_node_named(
        &'a self,
        source: &model::Source,
        name: &String,
    ) -> Result<&'a assemble::ModuleNode, errors::BuilderError>;

    fn node_at(&'a self, index: assemble::NodeIndex) -> &'a assemble::ModuleNode;

    /// Find the primary sequence index responsible for running the named node.
    /// This will always be the graph sequence associated with the node.
    fn graph_seq_for_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<SeqIndex, errors::BuilderError>;

    /// Find the sub-sequence index responsible for running the named node's
    /// ordered actions.
    fn execution_seq_for_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<SeqIndex, errors::BuilderError>;

    fn seq_range(&self) -> std::ops::Range<SeqIndex>;
}

pub struct StdSequenceStore {
    nodes: Vec<assemble::ModuleNode>,
    seq: RefCell<Vec<model::OrderedActions>>,
    node_exec_seq_map: RefCell<std::collections::HashMap<assemble::NodeIndex, SeqIndex>>,
    graph: node_graph::ScriptGraph,
    seq_graph_indicies: Vec<SeqIndex>,
    main_graph_idx: Option<SeqIndex>,
    main_node_idx: Option<assemble::NodeIndex>,
    start_seq_idx: SeqIndex,
    jobs: RefCell<Vec<(SeqIndex, usize)>>,
}

impl StdSequenceStore {
    pub fn new(nodes: Vec<assemble::ModuleNode>, graph: node_graph::ScriptGraph) -> Self {
        let mut seq_graph_indicies = Vec::new();
        let mut main_graph_idx = None;
        let mut main_node_idx = None;
        for (idx, graph) in graph.graphs().iter().enumerate() {
            let mut is_main = false;
            for node_idx in &graph.stream_order {
                if super::special::is_main_module_node(&nodes[*node_idx]) {
                    is_main = true;
                    main_graph_idx = Some(idx);
                    main_node_idx = Some(*node_idx);
                    break;
                }
            }
            if !is_main {
                seq_graph_indicies.push(idx);
            }
        }
        StdSequenceStore {
            nodes,
            seq: RefCell::new(Vec::new()),
            start_seq_idx: graph.graphs().len(),
            node_exec_seq_map: RefCell::new(std::collections::HashMap::new()),
            jobs: RefCell::new(Vec::new()),
            graph,
            seq_graph_indicies,
            main_graph_idx,
            main_node_idx,
        }
    }

    /// Get the node graphs.
    pub fn graphs(&self) -> &Vec<node_graph::NodeGraph> {
        self.graph.graphs()
    }

    /// Get the sequenced node graphs.
    pub fn sequence_graphs(&self) -> Vec<&node_graph::NodeGraph> {
        let mut ret = Vec::with_capacity(self.seq_graph_indicies.len());
        for idx in &self.seq_graph_indicies {
            ret.push(&self.graph.graphs()[*idx]);
        }
        ret
    }

    /// Get main's graph.
    pub fn main_graph(&self) -> Option<&node_graph::NodeGraph> {
        match self.main_graph_idx {
            Some(idx) => Some(&self.graph.graphs()[idx]),
            None => None,
        }
    }

    /// Get the main node.
    pub fn main_node(&self) -> Option<&assemble::ModuleNode> {
        match self.main_node_idx {
            Some(idx) => Some(&self.nodes[idx]),
            None => None,
        }
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

    pub fn global_jobs(&self) -> Vec<(SeqIndex, usize)> {
        self.jobs.borrow().clone()
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
    ) -> Result<&'a assemble::ModuleNode, errors::BuilderError> {
        self.graph.find_node_by_name(source, name, &self.nodes)
    }

    fn node_at(&'a self, index: assemble::NodeIndex) -> &'a assemble::ModuleNode {
        self.nodes.get(index).expect("bad indexing")
    }

    fn graph_seq_for_node_named(
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

    fn seq_range(&self) -> std::ops::Range<SeqIndex> {
        0..(self.seq.borrow().len() + self.seq_graph_indicies.len())
    }

    fn set_node_execution_sequence(
        &'a self,
        node_idx: assemble::NodeIndex,
        actions: &model::OrderedActions,
        seq_idx: SeqIndex,
        sequence_job_idx: usize,
    ) -> (SeqIndex, JobIndex) {
        let mut map = self.node_exec_seq_map.borrow_mut();
        if map.contains_key(&node_idx) {
            panic!("node already has an execution sequence");
        }
        let ordered_seq_idx = self.add_ordered_sequence(actions);
        map.insert(node_idx, ordered_seq_idx);
        let global_job_id = self.jobs.borrow().len();
        self.jobs.borrow_mut().push((seq_idx, sequence_job_idx));
        (ordered_seq_idx, global_job_id)
    }

    fn execution_seq_for_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<SeqIndex, errors::BuilderError> {
        let node = self.get_node_named(source, name)?;
        match self.node_exec_seq_map.borrow().get(&node.node_idx) {
            Some(seq_idx) => Ok(*seq_idx),
            None => Err(errors::BuilderError::NoSuchNode(errors::ErrorDetails {
                message: format!("no execution sequence for node: '{}'", name),
                source: source.clone(),
                related: vec![],
            })),
        }
    }

    fn gen_graph_seq_job0(&'a self, seq_idx: SeqIndex) -> JobIndex {
        let global_job_id = self.jobs.borrow().len();
        self.jobs.borrow_mut().push((seq_idx, 0));
        global_job_id
    }
}
