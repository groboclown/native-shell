//! Maintain stream pairs between nodes.

use std::collections::HashMap;
use std::rc::Rc;
use crate::server_shell::ast::model;
use crate::server_shell::builder::parse_node::ModuleNode;
use crate::shell_lib::compile::meta;
use super::parse_node;
use super::errors::{BuilderError, RelatedSource, Relationship, ErrorDetails};
use super::node_graph;
use super::sequence;
use super::special;

#[derive(Debug, Clone)]
pub enum BoundStream {
    MainNamed(StdNamedStream),
    MainFd(StdFdStream),
    Pipe(Rc<parse_node::NodeStream>),
}

#[derive(Debug, Clone)]
pub struct StdNamedStream {
    pub name: String,
    pub node_idx: parse_node::NodeIndex,
    pub stream: Rc<parse_node::NodeStream>,
    pub stream_type: meta::StreamType,
}

#[derive(Debug, Clone)]
pub struct StdFdStream {
    pub fd: u16,
    pub node_idx: parse_node::NodeIndex,
    pub stream: Rc<parse_node::NodeStream>,
    pub stream_type: meta::StreamType,
}

impl BoundStream {
    pub fn from_graph<'a, SG: sequence::SequenceGen<'a>>(
        graph: &node_graph::NodeGraph,
        sgen: &'a SG,
    ) -> Result<Vec<Self>, BuilderError> {
        let mut errs = Vec::new();
        let mut streams = Vec::new();

        // TODO this is filled with lots of early exit '?' code bits, when instead it should
        // push into errs and continue on the loop.

        // This builds first through the node output streams.
        // It then looks through the node input streams to find missing streams for the main module, whose streams
        // come as a single item.

        // Pass 1: the sink side of the streams.
        for node_idx in &graph.stream_order {
            let node = sgen.node_at(*node_idx);
            if special::is_main_module_node(node) {
                // The main module's streams are not explicit.
                continue;
            }
            for sink in &node.input_streams {
                assert_eq!(sink.dest_idx, *node_idx, "Sink stream destination index must match node index");
                let source_name = &sgen.node_at(sink.source_idx).node.name;
                if special::is_main_node_name(source_name) {
                    println!("Adding main stream from (main) {} ({}) to {} ({})", sink.source_idx, sink.source_idx, sink.dest_id, sink.dest_idx);
                    match &sink.source_decl {
                        meta::StreamDeclaration::Name(name) => {
                            streams.push(
                                BoundStream::MainNamed(StdNamedStream {
                                    name: name.clone(),
                                    node_idx: *node_idx,
                                    stream: sink.clone(),
                                    stream_type: sink.source_type.clone(),
                                }),
                            );
                        }
                        meta::StreamDeclaration::FdIndex(fd) => {
                            streams.push(
                                BoundStream::MainFd(StdFdStream {
                                    fd: *fd,
                                    node_idx: *node_idx,
                                    stream: sink.clone(),
                                    stream_type: sink.source_type.clone(),
                                }),
                            );
                        }
                        _ => {
                            errs.push(BuilderError::InvalidStreamUse(ErrorDetails {
                                message: "Main node stream must be a named or file descriptor".to_string(),
                                source: node.node.source.clone(),
                                related: vec![RelatedSource {
                                    relation: Relationship::StreamTarget,
                                    source: node.node.source.clone(),
                                }],
                            }));
                        }
                    }
                } else {
                    println!("Adding pipe stream from {} ({}) to {} ({})", sink.source_id, sink.source_idx, sink.dest_id, sink.dest_idx);
                    streams.push(BoundStream::Pipe(sink.clone()));
                }
            }
        }

        // Pass 2: the gen side of the streams, to find the missing main.
        for node_idx in &graph.stream_order {
            let node = sgen.node_at(*node_idx);
            if special::is_main_module_node(node) {
                // The main module's streams are not explicit.
                continue;
            }
            for gener in &node.output_streams {
                assert_eq!(gener.source_idx, *node_idx, "Sink stream destination index must match node index");
                let dest_name = &sgen.node_at(gener.dest_idx).node.name;
                if special::is_main_node_name(dest_name) {
                    println!("Adding main stream from {} ({}) to (main) {} ({})", gener.source_id, gener.source_idx, gener.dest_id, gener.dest_idx);
                    match &gener.dest_decl {
                        meta::StreamDeclaration::Name(name) => {
                            streams.push(
                                BoundStream::MainNamed(StdNamedStream {
                                    name: name.clone(),
                                    node_idx: *node_idx,
                                    stream: gener.clone(),
                                    stream_type: gener.dest_type.clone(),
                                }),
                            );
                        }
                        meta::StreamDeclaration::FdIndex(fd) => {
                            streams.push(
                                BoundStream::MainFd(StdFdStream {
                                    fd: *fd,
                                    node_idx: *node_idx,
                                    stream: gener.clone(),
                                    stream_type: gener.dest_type.clone(),
                                }),
                            );
                        }
                        _ => {
                            errs.push(BuilderError::InvalidStreamUse(ErrorDetails {
                                message: "Main node stream must be a named or file descriptor".to_string(),
                                source: node.node.source.clone(),
                                related: vec![RelatedSource {
                                    relation: Relationship::StreamSource,
                                    source: node.node.source.clone(),
                                }],
                            }));
                        }
                    }
                }
            }
        }

        if errs.is_empty() {
            Ok(streams)
        } else {
            Err(BuilderError::Collection(errs))
        }
    }

    /// Check if the stream only uses Read/Write streams.
    /// If false, then at least one uses file descriptors.
    pub fn is_strictly_rw(&self) -> bool {
        match self {
            BoundStream::Pipe(stream) => {
                match &stream.source_type {
                    meta::StreamType::Input(_) => {
                        panic!("source_type is marked as input")
                    }
                    meta::StreamType::Output(stream_interface) => {
                        match stream_interface {
                            meta::StreamInterface::ReadWrite => (),
                            meta::StreamInterface::Fd => {
                                return false;
                            }
                        }
                    }
                }
                match &stream.dest_type {
                    meta::StreamType::Input(stream_interface) => {
                        match &stream_interface {
                            meta::StreamInterface::ReadWrite => {
                                true
                            }
                            meta::StreamInterface::Fd => {
                                false
                            }
                        }
                    }
                    meta::StreamType::Output(_) => {
                        panic!("dest_type is marked as output")
                    }
                }
            }
            BoundStream::MainNamed(std_stream) => {
                match &std_stream.stream_type {
                    meta::StreamType::Input(stream_interface) => {
                        match stream_interface {
                            meta::StreamInterface::ReadWrite => true,
                            meta::StreamInterface::Fd => false,
                        }
                    }
                    meta::StreamType::Output(stream_interface) => {
                        match stream_interface {
                            meta::StreamInterface::ReadWrite => true,
                            meta::StreamInterface::Fd => false,
                        }
                    }
                }
            }
            BoundStream::MainFd(std_stream) => {
                match &std_stream.stream_type {
                    meta::StreamType::Input(stream_interface) => {
                        match stream_interface {
                            meta::StreamInterface::ReadWrite => true,
                            meta::StreamInterface::Fd => false,
                        }
                    }
                    meta::StreamType::Output(stream_interface) => {
                        match stream_interface {
                            meta::StreamInterface::ReadWrite => true,
                            meta::StreamInterface::Fd => false,
                        }
                    }
                }
            }
        }
    }
}

pub struct FixedNodeStream {
    pub field_name: String,
    pub required: bool,
    pub field_interface: meta::StreamInterface,
    pub stream_interface: meta::StreamInterface,
    pub stream_idx: usize,
    pub bound: BoundStream,
    pub direction: meta::StreamDirection,
}

pub struct VariableNodeStream {
    pub field_name: String,
    pub field_interface: meta::StreamInterface,
    pub stream_interface: meta::StreamInterface,
    pub streams: Vec<(usize, BoundStream)>,
}

/// Records the mapping of a node's streams to the module's stream structure,
/// with references to the source stream index + type.
pub struct NodeStreamStruct {
    pub node_id: String,
    pub module: Rc<meta::ModuleMeta>,
    pub fixed_streams: Vec<FixedNodeStream>,
    pub input_variable: Option<VariableNodeStream>,
    pub output_variable: Option<VariableNodeStream>,
}

impl NodeStreamStruct {
    /// Reverse the bound streams to reference the modules' stream structure.
    pub fn from_streams<'a, SG: sequence::SequenceGen<'a>>(
        streams: &Vec<BoundStream>,
        graph: &node_graph::NodeGraph,
        sgen: &'a SG,
    ) -> Result<Vec<Self>, BuilderError> {
        // Match up the streams to the node's stream via the RC stream id.
        let mut ret = vec![];
        for node_idx in &graph.stream_order {
            let node = sgen.node_at(*node_idx);
            let mut fixed_streams = vec![];
            let mut inputs = vec![];
            let mut outputs = vec![];
            for node_stream in node.input_streams {
                let bound = find_bound_stream_for(&node.node.source, streams, &node_stream)?;

                // Coming into this node, so it's an input (dest).
                match node_stream.dest_decl {
                    meta::StreamDeclaration::Name(name) => {
                        let mod_stream = find_named_fixed_stream(node, &name)?;
                        fixed_streams.push(FixedNodeStream {
                            field_name: name,
                            required: mod_stream.required,
                            field_interface: get_type_interface(node_stream.dest_type),
                            stream_interface: node_stream.source_type.interface(),
                            stream_idx: node_stream.source_idx.index(),
                            bound: BoundStream::Pipe(Rc::new(node_stream)),
                            direction: meta::StreamDirection::Input,
                        });
                    }
                    meta::StreamDeclaration::FdIndex(fd) => format!("fd_{}", fd),
                    meta::StreamDeclaration::VariableStream(name, _) => name,
                };
            }
            for node_stream in node.output_streams {
                let bound = find_bound_stream_for(&node.node.source, streams, &node_stream)?;

                // Going out of this node, so it's an output (source).
                let name = match node_stream.source_decl {
                    meta::StreamDeclaration::Name(name) => name,
                    meta::StreamDeclaration::FdIndex(fd) => format!("fd_{}", fd),
                    meta::StreamDeclaration::VariableStream(name, _) => name,
                };
            }

            // TODO validate the node's streams against the module's streams.
        }

        Ok(ret)
    }
}


fn find_named_fixed_stream(node: &ModuleNode, name: &str) -> Result<meta::FixedStreamDef, BuilderError> {
    if let Some(stream) = &node.module.stream_struct {
        for fixed_stream in &stream.fixed_streams {
            if let Some(f_name) = &fixed_stream.name {
                if f_name == name {
                    return Ok(fixed_stream.clone());
                }
            }
        }
    }
    Err(BuilderError::InvalidStreamUse(ErrorDetails {
        message: format!("Named stream '{}' not found in module '{}'", name, node.module.name),
        source: node.node.source.clone(),
        related: vec![],
    }))
}


fn find_fd_fixed_stream(node: &ModuleNode, fd: &u16) -> Result<meta::FixedStreamDef, BuilderError> {
    if let Some(stream) = &node.module.stream_struct {
        for fixed_stream in &stream.fixed_streams {
            if let Some(f_fd) = &fixed_stream.fd_index {
                if *f_fd == *fd as usize {
                    return Ok(fixed_stream.clone());
                }
            }
        }
    }
    Err(BuilderError::InvalidStreamUse(ErrorDetails {
        message: format!("FD stream '{}' not found in module '{}'", *fd, node.module.name),
        source: node.node.source.clone(),
        related: vec![],
    }))
}

fn get_type_interface(s_type: &meta::StreamType) -> meta::StreamInterface {
    match s_type {
        meta::StreamType::Input(stream_interface) => stream_interface.clone(),
        meta::StreamType::Output(stream_interface) => stream_interface.clone(),
    }
}

fn find_bound_stream_for<'a>(
    source: &model::Source, bounds: &'a Vec<BoundStream>, node: &Rc<parse_node::NodeStream>,
) -> Result<&'a BoundStream, BuilderError> {
    for stream in bounds {
        match stream {
            BoundStream::Pipe(s) => {
                if s.stream_id == node.stream_id {
                    return Ok(stream);
                }
            }
            BoundStream::MainNamed(s) => {
                if s.stream.stream_id == node.stream_id {
                    return Ok(stream);
                }
            }
            BoundStream::MainFd(s) => {
                if s.stream.stream_id == node.stream_id {
                    return Ok(stream);
                }
            }
        }
    }
    Err(BuilderError::InvalidStreamUse(ErrorDetails {
        message: format!("Bound stream for node {} not found", node.source_id),
        source: source.clone(),
        related: vec![RelatedSource {
            relation: Relationship::StreamSource,
            source: source.clone(),
        }],
    }))
}


#[cfg(test)]
mod tests {
    use crate::server_shell::builder::from_ast;

    use super::*;

    
    #[test]
    fn test_cat_cp_json() {
        // Load the AST.
        let json = std::str::from_utf8(include_bytes!("../../samples/cat_cp/ast.json")).expect("failed to utf8 convert json");
        let ast = crate::server_shell::ast::astio::read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation failed: {:?}", ast_errors);
        let nodes = parse_node::convert_nodes(&ast.nodes, &from_ast::get_available_modules()).expect("failed to convert nodes");

        // Ensure expected node order.
        assert_eq!(nodes.get(0).expect("node 0 must exist").node.name, "main");
        assert_eq!(nodes.get(1).expect("node 1 must exist").node.name, "cat");
        assert_eq!(nodes.get(2).expect("node 2 must exist").node.name, "output");

        // Load the script graph and sequence generator.
        let sg = node_graph::ScriptGraph::load(&nodes).expect("failed to load graph");
        let graphs = sg.graphs().clone();
        assert_eq!(graphs.len(), 2, "Expected a single graph, found: {}", graphs.len());
        let sgen = sequence::StdSequenceStore::new(nodes, sg);

        // Check bound streams from graph 0.
        // Graph 0 should contain just the main node, which has no bound streams.
        assert_eq!(graphs.get(0).expect("graph 0 must exist").stream_order.len(), 1);
        assert_eq!(*graphs.get(0).expect("graph 0 must exist").stream_order.get(0).expect("graph 0 node 0 must exist"), 0);
        let streams = BoundStream::from_graph(graphs.get(0).expect("must exist"), &sgen).expect("failed to build streams");
        assert!(streams.is_empty(), "Main node graph should have no streams, found {}", streams.len());

        // Check bound streams from graph 1.
        let streams = BoundStream::from_graph(graphs.get(1).expect("must exist"), &sgen).expect("failed to build streams");
        assert_eq!(streams.len(), 1);
        let stream = streams.get(0).expect("stream 0 must exist");
        match stream {
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 1, "Expected stream gen to be 0, found: {}", stream.source_idx);
                assert_eq!(stream.dest_idx, 2, "Expected stream sink to be 1, found: {}", stream.dest_idx);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
    }
    
    #[test]
    fn test_tee_merge_json() {
        // Load the AST.
        let json = std::str::from_utf8(include_bytes!("../../samples/tee_merge/ast.json")).expect("failed to utf8 convert json");
        let ast = crate::server_shell::ast::astio::read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation failed: {:?}", ast_errors);
        let nodes = parse_node::convert_nodes(&ast.nodes, &from_ast::get_available_modules()).expect("failed to convert nodes");

        // Ensure expected node order.
        assert_eq!(nodes.get(0).expect("node 0 must exist").node.name, "main");
        assert_eq!(nodes.get(1).expect("node 1 must exist").node.name, "data_file_1");
        assert_eq!(nodes.get(2).expect("node 2 must exist").node.name, "data_file_2");
        assert_eq!(nodes.get(3).expect("node 3 must exist").node.name, "data_text_1");
        assert_eq!(nodes.get(4).expect("node 4 must exist").node.name, "data_text_2");
        assert_eq!(nodes.get(5).expect("node 5 must exist").node.name, "tee1");
        assert_eq!(nodes.get(6).expect("node 6 must exist").node.name, "merge1");
        assert_eq!(nodes.get(7).expect("node 7 must exist").node.name, "merge2");
        assert_eq!(nodes.get(8).expect("node 8 must exist").node.name, "merge3");

        // Load the script graph and sequence generator.
        let sg = node_graph::ScriptGraph::load(&nodes).expect("failed to load graph");
        let graphs = sg.graphs().clone();
        assert_eq!(graphs.len(), 2, "Expected a single graph, found: {}", graphs.len());
        let sgen = sequence::StdSequenceStore::new(nodes, sg);

        // Check bound streams from graph 0.
        // Graph 0 should contain just the main node, which has no bound streams.
        assert_eq!(graphs.get(0).expect("graph 0 must exist").stream_order.len(), 1);
        assert_eq!(*graphs.get(0).expect("graph 0 must exist").stream_order.get(0).expect("graph 0 node 0 must exist"), 0);
        let streams = BoundStream::from_graph(graphs.get(0).expect("must exist"), &sgen).expect("failed to build streams");
        assert_eq!(streams.len(), 0, "Incorrect number of streams built, expected 0, found: {}", streams.len());

        // Check bound streams from graph 1.
        let streams = BoundStream::from_graph(graphs.get(1).expect("must exist"), &sgen).expect("failed to build streams");
        // FIXME there is currently a bug where the merge3 -> stdout is not generated.
        assert_eq!(streams.len(), 10);

        // Unfortunately, the stream ordering is not fixed.
        /*
        // Bound stream 1 stream 0.
        let stream = streams.get(0).expect("stream 0 must exist");
        match stream {
            // data_file_1 -> tee1
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 1, "Expected stream gen to be 0, found: {}", stream.source_idx);
                assert_eq!(stream.dest_idx, 5, "Expected stream sink to be 5, found: {}", stream.dest_idx);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 1.
        let stream = streams.get(1).expect("stream 0 must exist");
        match stream {
            // stdin -> merge1
            BoundStream::Main(stream) => {
                assert_eq!(stream.fd, 0);
                assert!(matches!(stream.direction, meta::StreamDirection::Input));
                assert_eq!(stream.node_idx, 1);
                assert_eq!(stream.stream.source_idx, 0);
                assert_eq!(stream.stream.dest_idx, 6);
            }
            _ => panic!("Expected a main stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 2.
        let stream = streams.get(2).expect("stream 0 must exist");
        match stream {
            // tee1 -> merge1
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 5);
                assert_eq!(stream.dest_idx, 6);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 3.
        let stream = streams.get(3).expect("stream 0 must exist");
        match stream {
            // data_text_1 -> merge1
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 3);
                assert_eq!(stream.dest_idx, 6);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 4.
        let stream = streams.get(4).expect("stream 0 must exist");
        match stream {
            // data_file_2 -> merge2
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 2);
                assert_eq!(stream.dest_idx, 7);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 5.
        let stream = streams.get(5).expect("stream 0 must exist");
        match stream {
            // data_text_2 -> merge2
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 4);
                assert_eq!(stream.dest_idx, 7);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 6.
        let stream = streams.get(6).expect("stream 0 must exist");
        match stream {
            // tee1 -> merge2
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 5);
                assert_eq!(stream.dest_idx, 8);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 7.
        let stream = streams.get(7).expect("stream 0 must exist");
        match stream {
            // merge1 -> merge3
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 6);
                assert_eq!(stream.dest_idx, 7);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        // Bound stream 1 stream 8.
        let stream = streams.get(8).expect("stream 0 must exist");
        match stream {
            // merge2 -> merge3
            BoundStream::Pipe(stream) => {
                assert_eq!(stream.source_idx, 7);
                assert_eq!(stream.dest_idx, 8);
            }
            _ => panic!("Expected a pipe stream, found: {:?}", stream),
        }
        */
    }
}
