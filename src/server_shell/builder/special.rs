//! Special case logic the generators need to handle.

use crate::server_shell::lls::model;

const MAIN_NODE: &str = "main";

pub fn is_main_node_name(node_name: &String) -> bool {
    node_name.as_str() == MAIN_NODE
}

pub fn is_main_module_node(node: &super::parse_node::ModuleNode) -> bool {
    is_main_node_name(&node.node.name)
}

pub fn is_main_seq<'a, SG: super::sequence::SequenceGen<'a>>(
    graph: &super::node_graph::NodeGraph,
    sgen: &'a SG,
) -> bool {
    if graph.stream_order.len() != 1 {
        return false;
    }
    let node = sgen.node_at(graph.stream_order[0]);
    super::special::is_main_module_node(node)
}

pub fn get_main_node<'a, SG: super::sequence::SequenceGen<'a>>(
    sgen: &'a SG,
) -> Result<&'a super::parse_node::ModuleNode, super::errors::BuilderError> {
    sgen.get_node_named(
        &model::Source {
            file: "--generator--".to_string(),
            column: 0,
            line: 0,
            text: None,
        },
        &MAIN_NODE.to_string(),
    )
}

pub fn get_main_node_seq_id<'a, SG: super::sequence::SequenceGen<'a>>(
    sgen: &'a SG,
) -> Result<usize, super::errors::BuilderError> {
    sgen.graph_seq_for_node_named(
        &model::Source {
            file: "--generator--".to_string(),
            column: 0,
            line: 0,
            text: None,
        },
        &MAIN_NODE.to_string(),
    )
}
