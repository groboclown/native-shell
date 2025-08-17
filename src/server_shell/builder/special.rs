//! Special case logic the generators need to handle.

const MAIN_NODE: &str = "main";

pub fn is_main_node(node: &String) -> bool {
    node.as_str() == MAIN_NODE
}

pub fn is_main_module_node(node: &super::parse_node::ModuleNode) -> bool {
    node.node.name.as_str() == MAIN_NODE
}

pub fn is_main_seq<'a, SG: super::sequence::SequenceGen<'a>>(graph: &super::node_graph::NodeGraph, sgen: &'a SG) -> bool {
    if graph.stream_order.len() != 1 {
        return false;
    }
    let node = sgen.node_at(graph.stream_order[0]);
    super::special::is_main_module_node(node)
}
