//! Generate the node graphs.
//!
//! This takes two forms: the first comes from grouping nodes by their
//! connected streams, to create the formal "node graph".  However, within this
//! it also needs to construct the graph based on nodes waiting for other nodes.
//! This second graph also must be a DAG, as two nodes waiting on each other turns
//! into a deadlock.  This also is needed for determining parallelism - if a node's
//! exit-code steps require conditional execution, then this means the ordering
//! must be preserved and not run in parallel.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::vec;

use super::errors;
use super::parse_node;
use crate::server_shell::ast::model;
use crate::server_shell::builder::parse_node::NodeIndex;
use crate::server_shell::builder::special::is_main_node_name;

#[derive(Debug, Clone)]
pub struct NodeGraph {
    /// Topological sort of the nodes within the graph based on streams.
    pub stream_order: Vec<parse_node::NodeIndex>,
    /// List of nodes to initially launch based on spawn and wait ordering.
    pub initial_exec: Vec<parse_node::NodeIndex>,
}

/// A forest of node graphs parsed for easier translation into generated code.
/// It references the ModuleNode list via the indices.
#[derive(Debug, Clone)]
pub struct ScriptGraph {
    node_graphs: Vec<NodeGraph>,
    node_name_indicies: HashMap<String, parse_node::NodeIndex>,
}

impl ScriptGraph {
    /// Turn the module node into separate graphs by stream.
    ///
    /// This expects the nodes to start at index 0 and increment
    /// by 1 for each node.
    pub fn load(nodes: &Vec<parse_node::ModuleNode>) -> Result<Self, errors::BuilderError> {
        assert!(!nodes.is_empty());
        let count = nodes.len();
        debug_assert_eq!(0, nodes.first().unwrap().node_idx);
        debug_assert_eq!(count - 1, nodes.last().unwrap().node_idx);
        let node_name_indicies = map_node_index(nodes);
        let node_graphs = stream_topo_sort(nodes, &node_name_indicies)?;

        // In the far future, this could also inspect inter-dependencies between
        // graphs.  That would only affect compile-time race condition checks.

        Ok(ScriptGraph {
            node_graphs,
            node_name_indicies,
        })
    }

    pub fn graphs(&self) -> &Vec<NodeGraph> {
        &self.node_graphs
    }

    /// Quick lookup of the node from the list by name.
    pub fn find_node_by_name<'a>(
        &self,
        source: &model::Source,
        name: &String,
        nodes: &'a Vec<parse_node::ModuleNode>,
    ) -> Result<&'a parse_node::ModuleNode, errors::BuilderError> {
        let index = get_node_index(source, name, &self.node_name_indicies)?;
        match nodes.get(index) {
            Some(v) => Ok(&v),
            None => Err(errors::BuilderError::NoSuchNode(errors::ErrorDetails {
                message: format!("no such node: '{}'", name),
                source: source.clone(),
                related: vec![],
            })),
        }
    }
}

enum TopoItem {
    // node index, cluster index
    Enter((parse_node::NodeIndex, parse_node::NodeIndex)),
    Exit((parse_node::NodeIndex, parse_node::NodeIndex)),
}

fn stream_topo_sort(
    nodes: &Vec<parse_node::ModuleNode>,
    node_map: &HashMap<String, parse_node::NodeIndex>,
) -> Result<Vec<NodeGraph>, errors::BuilderError> {
    // Non-recursive topo sort.  The implicit call stack is made explicit.
    // This first has a visiting node visit its source streams,
    // then it visits the destination streams, and finally it marks the node as visited.
    // This is done in a depth-first manner, so the source streams are visited first.

    let count = nodes.len();
    let mut clusters = StreamClusters::new(count);
    let mut visited = vec![None; count];
    let mut visiting = vec![false; count];
    let mut depth = Vec::with_capacity(count);
    let mut main_node = NodeIndex::MAX; // placeholder
    // This will visit every node, to ensure all spanning trees are visited.
    for node_idx in 0..count {
        let cluster = clusters.next_cluster();

        // The main node has its own special handling.
        // Main node does not participate in the node graph, as it will nearly always introduce cycles,
        // and its stream handling is its own thing.
        if is_main_node_name(&nodes.get(node_idx).expect("wrong counts").node.name) {
            clusters.push(cluster, node_idx);
            main_node = node_idx;
        } else {
            depth.push(TopoItem::Enter((node_idx, cluster)));
        }
    }

    while depth.len() > 0 {
        match depth.pop().expect("depth wasn't empty") {
            TopoItem::Enter((current_node, current_cluster)) => {
                if current_node == main_node {
                    // Skip the main node, as it doesn't participate in the node graph.
                    continue;
                }
                if let Some(descendant_cluster) = *visited.get(current_node).expect("wrong counts") {
                    // Already visited.
                    // This means we need to change the current cluster to point to the visited cluster.
                    clusters.join_clusters(current_cluster, descendant_cluster);
                    continue;
                }
                if *visiting.get(current_node).expect("wrong counts") {
                    // If this implementation was really good, it would also report how
                    // the cycle happened in the 'related' field.  It's in the cluster's topo sort.
                    let node = nodes.get(current_node).expect("wrong counts");
                    return Err(errors::BuilderError::StreamCycle(errors::ErrorDetails {
                        message: "streams reference back on each other".to_string(),
                        source: node.node.source.clone(),
                        related: vec![],
                    }));
                }
                visiting[current_node] = true;
                let node = nodes.get(current_node).expect("wrong counts");

                // After destination streams, re-visit the parent to mark it as visited.
                // Because it's pushed first, it's visited last.
                // The alternative approach reverses the returned list
                // before return.
                depth.push(TopoItem::Exit((current_node, current_cluster)));

                // Note that, because this visits from a node to its destination streams,
                // it means that the source stream must be added to the returned list before
                // the destination streams.
                clusters.push(current_cluster, current_node);

                // Visit destination streams.
                for stream in &node.output_streams {
                    depth.push(TopoItem::Enter((stream.dest_idx, current_cluster)));
                }
            }
            TopoItem::Exit((current_node, current_cluster)) => {
                visited[current_node] = Some(current_cluster);
            }
        }
    }
    clusters.streams(nodes, node_map)
}

fn find_initial_spawned_group(
    tree: &Vec<parse_node::NodeIndex>,
    nodes: &Vec<parse_node::ModuleNode>,
    node_map: &HashMap<String, parse_node::NodeIndex>,
) -> Result<Vec<parse_node::NodeIndex>, errors::BuilderError> {
    // Anything that spawns a node within the tree, based on exit codes, requires the spawned
    // node to run outside the initial group.
    // Waiting on another node as an action doesn't itself mean that it must be outside
    // the initial group.
    // Likewise, having an event listener spawn another job doesn't mean that it must
    // be outside the initial group.  It's only the exit code conditional that
    // determines this.

    let mut in_group = vec![false; nodes.len()];
    for node_idx in tree {
        // Default to all nodes in the tree are allowed.
        in_group[*node_idx] = true;
    }
    for node_idx in tree {
        if !in_group[*node_idx] {
            continue;
        }
        let node = nodes.get(*node_idx).expect("wrong counts");
        // If this spawns another node, then the other node is not allowed.
        // Note that it only has a meaning if the spawned node is within the
        // current group.  If it's outside the group then this is a no-op.
        for spawned_idx in spawns_index(node, node_map)? {
            in_group[spawned_idx] = false;
        }
    }

    let mut ret = Vec::new();
    for node_idx in tree {
        if in_group[*node_idx] {
            ret.push(*node_idx);
        }
    }
    Ok(ret)
}

fn spawns_index(
    mod_node: &parse_node::ModuleNode,
    node_map: &HashMap<String, parse_node::NodeIndex>,
) -> Result<Vec<parse_node::NodeIndex>, errors::BuilderError> {
    let mut ret = Vec::new();
    for action_list in &mod_node.node.exit_actions {
        for action in &action_list.actions.0 {
            match &action.run {
                model::ActionRun::SpawnNode { node, source } => {
                    ret.push(get_node_index(&source, &node, node_map)?);
                }
                model::ActionRun::EnsureNodeStartedAtLeastOnce { node, source } => {
                    ret.push(get_node_index(&source, &node, node_map)?);
                }
                _ => (),
            }
        }
    }
    Ok(ret)
}

/* Right now, the initial execution scan logic only cares about spawned nodes.
   If it comes out it also needs to examine whether a node waits on another one,
   then this can be added to the initial_exec list.
fn waits_for_index(mod_node: &parse_node::ModuleNode) -> bool {
    for action_list in &mod_node.node.exit_actions {
        for action in &action_list.actions.0 {
            match &action.run {
                model::ActionRun::WaitForNode {
                    for_exit: _,
                    if_not_started: _,
                    node: _,
                    source: _,
                } => {
                    return true;
                }
                _ => (),
            }
        }
    }
    false
}
*/

fn map_node_index(nodes: &Vec<parse_node::ModuleNode>) -> HashMap<String, parse_node::NodeIndex> {
    let mut ret = HashMap::new();
    for (node_idx, node) in nodes.iter().enumerate() {
        debug_assert_eq!(node_idx, node.node_idx);
        ret.insert(node.node.name.clone(), node.node_idx);
    }
    ret
}

struct LinkedEl {
    node_idx: parse_node::NodeIndex,
    next: Option<Rc<RefCell<LinkedEl>>>,
}

struct SingleLinkedList {
    head: LinkedEl,
    tail: Option<Rc<RefCell<LinkedEl>>>,
    count: usize,
}

impl SingleLinkedList {
    fn new() -> Self {
        SingleLinkedList {
            head: LinkedEl { node_idx: 0, next: None },
            tail: None,
            count: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn as_vec(&self) -> Vec<parse_node::NodeIndex> {
        let mut ret = Vec::with_capacity(self.count);
        let mut next = self.head.next.clone();
        while let Some(el) = next {
            let el = el.borrow();
            ret.push(el.node_idx);
            next = el.next.clone();
        }
        ret
    }

    fn push(&mut self, node_idx: parse_node::NodeIndex) {
        let new_el = Rc::new(RefCell::new(LinkedEl {
            node_idx,
            next: None,
        }));
        match &self.tail {
            Some(tail) => {
                // If the tail exists, we can link the new element to it.
                // This is safe because we hold a read lock on the tail.
                let mut tail_ptr = tail.borrow_mut();
                tail_ptr.next = Some(new_el.clone());
            },
            None => {
                // If the tail doesn't exist, this is the first element.
                self.head.next = Some(new_el.clone());
            }
        }
        self.tail = Some(new_el);
        self.count += 1;
    }

    fn prepend(&mut self, other: &SingleLinkedList) {
        if let Some(other_head) = &other.head.next {
            let other_tail = other.tail.clone().expect("list with head must have tail");
            if let Some(head) = self.head.next.clone() {
                // If self has a head, link the other tail to it.
                other_tail.borrow_mut().next = Some(head);
            } else {
                // If self doesn't have a head, then it doesn't have a tail.
                self.tail = Some(other_tail.clone());
            }
            self.head.next = Some(other_head.clone());
            self.count += other.count;
        }
    }

    fn clear(&mut self) {
        self.head = LinkedEl { node_idx: 0, next: None };
        self.tail = None;
        self.count = 0;
    }
}

/// Each possible stream group's topological sort is stored as an index in a vector.
/// Uses thread safe structures.
struct StreamTopoSet {
    streams: Vec<Arc<RwLock<SingleLinkedList>>>,
}

impl StreamTopoSet {
    fn new(count: usize) -> Self {
        let mut streams = Vec::with_capacity(count);
        for _ in 0..count {
            streams.push(Arc::new(RwLock::new(SingleLinkedList::new())));
        }
        StreamTopoSet {
            streams,
        }
    }

    /// Add a node to the end of a stream graph.
    fn push(&mut self, stream_idx: parse_node::NodeIndex, node_idx: parse_node::NodeIndex) {
        assert!(stream_idx < self.streams.len(), "stream index out of bounds");
        let mut list = self.streams[stream_idx as usize].write().expect("failed to lock stream list");
        list.push(node_idx);
    }

    /// Prepends a source stream on a stream, and clears out the source.
    fn prepend(&mut self, src_stream: parse_node::NodeIndex, dest_stream: parse_node::NodeIndex) {
        assert!(src_stream < self.streams.len(), "source stream index out of bounds");
        assert!(dest_stream < self.streams.len(), "destination stream index out of bounds");
        let mut dest = self.streams[dest_stream as usize].write().expect("failed to lock stream list");
        let mut src = self.streams[src_stream as usize].write().expect("failed to lock stream list");
        dest.prepend(&src);
        src.clear();
    }
}

/// A two-jump reference to a stream topo index.
struct StreamClusters {
    streams: StreamTopoSet,
    clusters: Vec<RwLock<parse_node::NodeIndex>>,
    node_to_cluster: RwLock<Vec<parse_node::NodeIndex>>,
    next: RwLock<parse_node::NodeIndex>,
}

const CLUSTER_UNASSIGNED: parse_node::NodeIndex = parse_node::NodeIndex::MAX;

impl StreamClusters {
    fn new(count: usize) -> Self {
        let mut clusters = Vec::with_capacity(count);
        for _ in 0..count {
            clusters.push(RwLock::new(CLUSTER_UNASSIGNED));
        }
        StreamClusters {
            streams: StreamTopoSet::new(count),
            clusters,
            node_to_cluster: RwLock::new(vec![CLUSTER_UNASSIGNED; count]),
            next: RwLock::new(0),
        }
    }

    /// Get the next cluster index, which will also associate it with the next, unused stream.
    fn next_cluster(&mut self) -> parse_node::NodeIndex {
        // Cluster and stream always start as the same index.  As clusters join with others, they switch their
        // streams always to a lower index.
        let mut next = self.next.write().expect("failed to lock next cluster index");
        let cluster_idx = *next;
        *next += 1; 
        assert!(cluster_idx < self.clusters.len(), "cluster index out of bounds");
        let mut cluster = self.clusters[cluster_idx].write().expect("failed to lock cluster");
        *cluster = cluster_idx;
        cluster_idx
    }

    /// Add a node to the end of a cluster's stream.
    fn push(&mut self, cluster_idx: parse_node::NodeIndex, node_idx: parse_node::NodeIndex) {
        assert!(cluster_idx < self.clusters.len(), "cluster index out of bounds");

        // First, lock the cluster.
        {
            let cluster = self.clusters[cluster_idx].write().expect("failed to lock cluster");
            // Then, add the node to the cluster's stream.
            let stream_idx = *cluster;
            self.streams.push(stream_idx, node_idx);
        }

        // Assign the node to the cluster.
        // As the node is always assigned to the same cluster, and only the cluster can change, this node lock
        // can happen outside the cluster lock.
        let mut node_to_cluster = self.node_to_cluster.write().expect("failed to lock node to cluster");
        assert!(node_idx < node_to_cluster.len(), "node index out of bounds");
        node_to_cluster[node_idx] = cluster_idx;
    }

    /// Join two clusters.
    /// A cluster joined to another will always happen as the second is a dependency of the first.
    /// In order to avoid updating all clusters referencing a stream, the ancestor cluster will reference the
    /// the descendant cluster, ans the descendant cluster should have already been populated.
    fn join_clusters(&mut self, ancestor_cluster: parse_node::NodeIndex, descendant_cluster: parse_node::NodeIndex) {
        // Lock both clusters.
        assert!(ancestor_cluster < self.clusters.len(), "ancestor cluster index out of bounds");
        assert!(descendant_cluster < self.clusters.len(), "descendant cluster index out of bounds");
        let mut ancestor = self.clusters[ancestor_cluster].write().expect("failed to lock ancestor cluster");
        let descendant = self.clusters[descendant_cluster].read().expect("failed to lock descendant cluster");

        let src_stream = *ancestor;
        let dest_stream = *descendant;

        // Now that they're locked, we can prepend the descendant's stream to the ancestor's stream.
        if src_stream != dest_stream {
            self.streams.prepend(src_stream, dest_stream);

            // Finally, we can update the ancestor to point to the descendant to join the clusters.
            *ancestor = *descendant;
        }
    }

    /// Get the underlying stream's topological ordering of the nodes.
    fn streams(
        &self,
        nodes: &Vec<parse_node::ModuleNode>,
        node_map: &HashMap<String, parse_node::NodeIndex>,
    ) -> Result<Vec<NodeGraph>, errors::BuilderError> {
        let mut ret = Vec::with_capacity(self.streams.streams.len());
        for stream in &self.streams.streams {
            let list = stream.read().expect("failed to lock stream list");
            if ! list.is_empty() {
                // Only add non-empty streams.
                let stream_order = list.as_vec();
                let initial_exec = find_initial_spawned_group(&stream_order, nodes, &node_map)?;
                ret.push(NodeGraph {
                    stream_order,
                    initial_exec,
                });
            }
        }
        Ok(ret)
    }
}

fn get_node_index(
    source: &model::Source,
    name: &String,
    node_map: &HashMap<String, parse_node::NodeIndex>,
) -> Result<parse_node::NodeIndex, errors::BuilderError> {
    match node_map.get(name) {
        Some(v) => Ok(*v),
        None => Err(errors::BuilderError::NoSuchNode(errors::ErrorDetails {
            message: format!("referenced unknown node name: {}", name),
            source: source.clone(),
            related: vec![],
        })),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::server_shell::ast::model;
    use crate::server_shell::builder::{parse_node, from_ast};
    use crate::shell_lib::compile::meta;
    use crate::shell_lib::modules::{cat, file_sink, shell};

    #[test]
    fn test_cat_cp_graph() {
        let nodes = mk_cat_cp();
        let graph = ScriptGraph::load(&nodes).expect("failed to load graph");
        assert_eq!(graph.node_graphs.len(), 2);

        let tg = &graph.node_graphs[0];
        assert_eq!(tg.stream_order.len(), 1);
        assert_eq!(tg.initial_exec.len(), 1);
        assert_eq!(tg.stream_order[0], 0); // node_idx 0 == main
        assert_eq!(tg.initial_exec[0], 0); // node_idx 0 == main

        let tg = &graph.node_graphs[1];
        assert_eq!(tg.stream_order.len(), 2);
        assert_eq!(tg.initial_exec.len(), 2);  // Both can run in parallel.
        assert_eq!(tg.stream_order[0], 1); // node_idx 1 == cat
        assert_eq!(tg.stream_order[1], 2); // node_idx 2 == sink
        assert_eq!(tg.initial_exec[0], 1); // node_idx 1 == cat
        assert_eq!(tg.initial_exec[1], 2); // node_idx 2 == sink
    }

    #[test]
    fn test_cat_cp_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/cat_cp/ast.json")).expect("failed to utf8 convert json");
        let ast = crate::server_shell::ast::astio::read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation failed: {:?}", ast_errors);
        let nodes = parse_node::convert_nodes(&ast.nodes, &from_ast::get_available_modules()).expect("failed to convert nodes");
        let graph = ScriptGraph::load(&nodes).expect("failed to load graph");
        assert_eq!(graph.node_graphs.len(), 2);

        let tg = &graph.node_graphs[0];
        assert_eq!(tg.stream_order.len(), 1);
        assert_eq!(tg.initial_exec.len(), 1);
        assert_eq!(tg.stream_order[0], 0); // node_idx 0 == main
        assert_eq!(tg.initial_exec[0], 0); // node_idx 0 == main

        let tg = &graph.node_graphs[1];
        assert_eq!(tg.stream_order.len(), 2);
        assert_eq!(tg.initial_exec.len(), 2);  // Both can run in parallel.
        assert_eq!(tg.stream_order[0], 1); // node_idx 1 == cat
        assert_eq!(tg.stream_order[1], 2); // node_idx 2 == sink
        assert_eq!(tg.initial_exec[0], 1); // node_idx 1 == cat
        assert_eq!(tg.initial_exec[1], 2); // node_idx 2 == sink
    }

    #[test]
    fn test_tee_merge_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/tee_merge/ast.json")).expect("failed to utf8 convert json");
        let ast = crate::server_shell::ast::astio::read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation failed: {:?}", ast_errors);
        let nodes = parse_node::convert_nodes(&ast.nodes, &from_ast::get_available_modules()).expect("failed to convert nodes");
        let graph = ScriptGraph::load(&nodes).expect("failed to load graph");
        assert_eq!(graph.node_graphs.len(), 2);
        assert_eq!(nodes.get(0).expect("exists").node.name, "main".to_string());
        assert_eq!(nodes.get(0).expect("exists").node_id, "shell_0".to_string());
        assert_eq!(nodes.get(1).expect("exists").node.name, "data_file_1".to_string());
        assert_eq!(nodes.get(1).expect("exists").node_id, "cat_1".to_string());
        assert_eq!(nodes.get(2).expect("exists").node.name, "data_file_2".to_string());
        assert_eq!(nodes.get(2).expect("exists").node_id, "cat_2".to_string());
        assert_eq!(nodes.get(3).expect("exists").node.name, "data_text_1".to_string());
        assert_eq!(nodes.get(3).expect("exists").node_id, "echo_3".to_string());
        assert_eq!(nodes.get(4).expect("exists").node.name, "data_text_2".to_string());
        assert_eq!(nodes.get(4).expect("exists").node_id, "echo_4".to_string());
        assert_eq!(nodes.get(5).expect("exists").node.name, "tee1".to_string());
        assert_eq!(nodes.get(5).expect("exists").node_id, "tee_5".to_string());
        assert_eq!(nodes.get(6).expect("exists").node.name, "merge1".to_string());
        assert_eq!(nodes.get(6).expect("exists").node_id, "merge_6".to_string());
        assert_eq!(nodes.get(7).expect("exists").node.name, "merge2".to_string());
        assert_eq!(nodes.get(7).expect("exists").node_id, "merge_7".to_string());
        assert_eq!(nodes.get(8).expect("exists").node.name, "merge3".to_string());
        assert_eq!(nodes.get(8).expect("exists").node_id, "merge_8".to_string());
        assert_eq!(graph.find_node_by_name(&mk_src(1), &"main".to_string(), &nodes).expect("exists").node_id, "shell_0".to_string());

        let tg = &graph.node_graphs[0];
        assert_eq!(tg.stream_order.len(), 1);
        assert_eq!(tg.initial_exec.len(), 1);
        assert_eq!(tg.stream_order[0], 0); // node_idx 0 == main
        assert_eq!(tg.initial_exec[0], 0); // node_idx 0 == main

        let tg = &graph.node_graphs[1];
        assert_eq!(tg.stream_order.len(), 8);
        assert_eq!(tg.stream_order[0], 1); // node_idx 1 == data_file_1
        assert_eq!(tg.stream_order[1], 2); // node_idx 2 == data_file_2
        assert_eq!(tg.stream_order[2], 3); // node_idx 3 == data_text_1
        assert_eq!(tg.stream_order[3], 4); // node_idx 4 == data_text_2
        assert_eq!(tg.stream_order[4], 5); // node_idx 5 == tee1
        assert_eq!(tg.stream_order[5], 6); // node_idx 6 == merge1
        assert_eq!(tg.stream_order[6], 7); // node_idx 2 == merge2
        assert_eq!(tg.stream_order[7], 8); // node_idx 2 == merge3

        assert_eq!(tg.initial_exec.len(), 8);  // All can run in parallel.
        assert_eq!(tg.initial_exec[0], 1); // node_idx 1 == data_file_1
        assert_eq!(tg.initial_exec[1], 2); // node_idx 2 == data_file_2
        assert_eq!(tg.initial_exec[2], 3); // node_idx 3 == data_text_1
        assert_eq!(tg.initial_exec[3], 4); // node_idx 4 == data_text_2
        assert_eq!(tg.initial_exec[4], 5); // node_idx 5 == tee1
        assert_eq!(tg.initial_exec[5], 6); // node_idx 6 == merge1
        assert_eq!(tg.initial_exec[6], 7); // node_idx 7 == merge2
        assert_eq!(tg.initial_exec[7], 8); // node_idx 8 == merge3
    }

    fn mk_cat_cp() -> Vec<parse_node::ModuleNode> {
        let fs_mod = Rc::new(file_sink::module_meta());
        let cat_mod = Rc::new(cat::module_meta());
        let shell_mod = Rc::new(shell::module_meta());
        let cat_fs_stream = Rc::new(parse_node::NodeStream {
            stream_id: 1,
            source_id: "cat_1".to_string(),
            source_idx: 1,
            dest_id: "sink_2".to_string(),
            dest_idx: 2,
            source_type: meta::StreamType::Output(meta::StreamInterface::Fd),
            source_decl: meta::StreamDeclaration::FdIndex(0),
            dest_type: meta::StreamType::Input(meta::StreamInterface::Fd),
            dest_decl: meta::StreamDeclaration::FdIndex(0),
        });
        vec![
            parse_node::ModuleNode {
                node_idx: 0,
                node_id: "main_0".to_string(),
                module: shell_mod.clone(),
                input_streams: vec![],
                output_streams: vec![],
                node: model::Node {
                    name: "main".to_string(),
                    module: shell_mod.name.clone(),
                    source: mk_src(1),
                    runtime_parameters: model::NamedParameters {
                        0: vec![],
                    },
                    streams: vec![],
                    event_listeners: vec![
                        model::EventListener {
                            name: "start".to_string(),
                            source: mk_src(2),
                            actions: model::OrderedActions {
                                0: vec![model::Action {
                                    name: "cat src.txt > tgt.txt".to_string(),
                                    run: model::ActionRun::SpawnNode {
                                        node: "cat".to_string(),
                                        source: mk_src(12),
                                    },
                                    source: mk_src(2),
                                }],
                            },
                        },
                    ],
                    initial_parameters: HashMap::new(),
                    exit_actions: vec![],
                },
            },
            parse_node::ModuleNode {
                node_idx: 1,
                node_id: "cp_1".to_string(),
                module: cat_mod.clone(),
                input_streams: vec![],
                output_streams: vec![cat_fs_stream.clone()],
                node: model::Node {
                    name: "cat".to_string(),
                    module: cat_mod.name.clone(),
                    source: mk_src(3),
                    runtime_parameters: model::NamedParameters {
                        0: vec![model::ActionParameter {
                            name: "filenames".to_string(),
                            source: mk_src(7),
                            value: model::ComputedValue::StringListValue(
                                model::ComputedStringListValue::ConstantStringListValue(
                                    model::ConstantStringListValue {
                                        type_: "string_list".into(),
                                        value: vec![model::ConstantStringListValueValueItem::Value(
                                            model::ComputedStringValue::ConstantStringValue(model::ConstantStringValue {
                                                type_: "string".into(),
                                                value: "src.txt".to_string(),
                                                source: mk_src(8),
                                            })
                                        )],
                                        source: mk_src(9),
                                    },
                                ),
                            ),
                        }],
                    },
                    streams: vec![model::Stream {
                        fd: Some(0),
                        mode: model::StreamMode::Output,
                        name: None,
                        source: mk_src(10),
                        to_node: "sink_2".to_string(),
                        to_stream: model::StreamToStream::Integer(0),
                    }],
                    event_listeners: vec![],
                    initial_parameters: HashMap::new(),
                    exit_actions: vec![],
                },
            },
            parse_node::ModuleNode {
                node_idx: 2,
                node_id: "sink_2".to_string(),
                module: fs_mod.clone(),
                input_streams: vec![cat_fs_stream.clone()],
                output_streams: vec![],
                node: model::Node {
                    name: "sink".to_string(),
                    module: fs_mod.name.clone(),
                    source: mk_src(4),
                    runtime_parameters: model::NamedParameters{0: vec![
                        model::ActionParameter{
                            name: "filename".to_string(),
                            source: mk_src(5),
                            value: model::ComputedValue::StringValue(model::ComputedStringValue::ConstantStringValue(model::ConstantStringValue {
                                type_: "string".into(),
                                value: "tgt.txt".to_string(),
                                source: mk_src(6),
                            })),
                        }
                    ]},
                    streams: vec![],
                    event_listeners: vec![],
                    exit_actions: vec![],
                    initial_parameters: HashMap::new(),
                },
            },
        ]
    }

    fn mk_src(line: i64) -> model::Source {
        model::Source {
            file: "test".to_string(),
            line,
            column: 0,
            text: None,
        }
    }
}
