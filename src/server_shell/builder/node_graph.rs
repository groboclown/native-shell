//! Generate the node graphs.
//!
//! This takes two forms: the first comes from grouping nodes by their
//! connected streams, to create the formal "node graph".  However, within this
//! it also needs to construct the graph based on nodes waiting for other nodes.
//! This second graph also must be a DAG, as two nodes waiting on each other turns
//! into a deadlock.  This also is needed for determining parallelism - if a node's
//! exit-code steps require conditional execution, then this means the ordering
//! must be preserved and not run in parallel.

use std::collections::HashMap;

use super::errors;
use super::parse_node;
use crate::server_shell::ast::model;

pub struct NodeGraph {
    /// Topological sort of the nodes within the graph based on streams.
    pub stream_order: Vec<parse_node::NodeIndex>,
    /// List of nodes to initially launch based on spawn and wait ordering.
    pub initial_exec: Vec<parse_node::NodeIndex>,
}

/// A forest of node graphs parsed for easier translation into generated code.
/// It references the ModuleNode list via the indices.
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
        let node_map = map_node_index(nodes);

        let mut ret = vec![];
        let mut visited = vec![false; count];
        for node in nodes {
            // Build up the graph forest.
            if *visited.get(node.node_idx).unwrap() {
                continue;
            }
            let stream_order = stream_topo_sort(node, nodes, &mut visited)?;
            let initial_exec = find_initial_spawned_group(&stream_order, nodes, &node_map)?;
            ret.push(NodeGraph {
                stream_order,
                initial_exec,
            });
        }

        // In the far future, this could also inspect inter-dependencies between
        // graphs.  That would only affect compile-time race condition checks.

        Ok(ScriptGraph {
            node_graphs: ret,
            node_name_indicies: node_map,
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
    Enter(parse_node::NodeIndex),
    Exit(parse_node::NodeIndex),
}

fn stream_topo_sort(
    root: &parse_node::ModuleNode,
    nodes: &Vec<parse_node::ModuleNode>,
    visited: &mut Vec<bool>,
) -> Result<Vec<parse_node::NodeIndex>, errors::BuilderError> {
    // Non-recursive topo sort.  The implicit call stack is made explicit.
    let count = visited.len();
    let mut ret = Vec::new();
    let mut visiting = vec![false; count];
    let mut depth = Vec::with_capacity(count);
    depth.push(TopoItem::Enter(root.node_idx));
    while depth.len() > 0 {
        match depth.pop().expect("depth wasn't empty") {
            TopoItem::Enter(current) => {
                if *visited.get(current).expect("wrong counts") {
                    // Already visited
                    continue;
                }
                if *visiting.get(current).expect("wrong counts") {
                    // If we were really good, we'd also report how
                    // the cycle happened in the 'related' field.
                    let node = nodes.get(current).expect("wrong counts");
                    return Err(errors::BuilderError::StreamCycle(errors::ErrorDetails {
                        message: "streams reference back on each other".to_string(),
                        source: node.node.source.clone(),
                        related: vec![],
                    }));
                }
                visiting[current] = true;
                let node = nodes.get(current).expect("wrong counts");

                // After destination streams, re-visit the parent to mark it as visited.
                // Because it's pushed first, it's visited last.
                // The alternative approach reverses the returned list
                // before return.
                depth.push(TopoItem::Exit(current));

                // Note that, because this visits from a node to its destination streams,
                // it means that the source stream must be added to the returned list before
                // the destination streams.
                ret.push(current);

                // Visit destination streams.
                for stream in &node.dest_streams {
                    depth.push(TopoItem::Enter(stream.dest_idx));
                }
            }
            TopoItem::Exit(current) => {
                visited[current] = true;
            }
        }
    }
    Ok(ret)
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

    fn mk_cat_cp() -> Vec<parse_node::ModuleNode> {
        let fs_mod = Rc::new(file_sink::module_meta());
        let cat_mod = Rc::new(cat::module_meta());
        let shell_mod = Rc::new(shell::module_meta());
        let cat_fs_stream = Rc::new(parse_node::NodeStream {
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
                src_streams: vec![],
                dest_streams: vec![],
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
                src_streams: vec![],
                dest_streams: vec![cat_fs_stream.clone()],
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
                src_streams: vec![cat_fs_stream.clone()],
                dest_streams: vec![],
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