//! Generate a seq*.rs file for a script.

use super::helpers::{as_mod_expr, rust_file_header};
use super::node_graph::NodeGraph;
use super::parse_node::ModuleNode;
use super::sequence::{SeqIndex, SequenceGen};
use super::writer::SourceWriter;
use super::stream_pair;
use crate::server_shell::builder::errors::{BuilderError, ErrorDetails};
use crate::server_shell::builder::helpers;
use crate::server_shell::builder::stream_pair::VariableNodeStream;
use crate::shell_lib::compile::meta;

pub fn write_graph_seq<'a, SW: SourceWriter, SG: SequenceGen<'a>>(
    seq_idx: SeqIndex,
    graph: &NodeGraph,
    sgen: &'a SG,
    out: &SW,
) -> Result<(), BuilderError> {
    if super::special::is_main_seq(graph, sgen) {
        // The main module is not part of a sequence.
        return Ok(());
    }

    let mut out = out.writer_for(&format!("src/seq{}.rs", seq_idx))?;
    out.write_all(rust_file_header().as_bytes())?;
    out.write_all(
        b"use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use crate::shell_lib::compile::job;
use crate::shell_lib::compile::source::Source;
use crate::shell_lib::helpers;
use crate::runtime;

",
    )?;
    out.write_fmt(format_args!("pub struct Seq{}StateInner {{\n", seq_idx))?;
    for node_idx in &graph.stream_order {
        let node = sgen.node_at(*node_idx);
        if super::special::is_main_module_node(node) {
            // The main module must be in its own sequence, not handled here.
            return Err(BuilderError::MainModuleInSequence(ErrorDetails {
                message: "The main module cannot be part of a sequence.".to_string(),
                source: node.node.source.clone(),
                related: vec![],
            }));
        }
        if let Some(ss) = &node.module.stream_struct {
            out.write_fmt(format_args!(
                "    {}: Option<{}{}>,\n",
                &node.node.name,
                as_mod_expr(node),
                ss.name,
            ))?;
        }
    }
    out.write_all(b"}\n\n")?;
    out.write_fmt(format_args!(
        "pub type Seq{}State = helpers::state_guard::StateGuard<Seq{}StateInner>;\n",
        seq_idx, seq_idx
    ))?;
    out.write_fmt(format_args!(
        "pub fn new_seq{}() -> Seq{}State {{\n    helpers::state_guard::StateGuard::new(Seq{}StateInner {{\n",
        seq_idx, seq_idx, seq_idx
    ))?;
    for node_idx in &graph.stream_order {
        let node = sgen.node_at(*node_idx);
        if node.module.stream_struct.is_some() {
            out.write_fmt(format_args!("    {}: None,\n", &node.node.name))?;
        }
    }
    out.write_all(b"    })\n}\n\n")?;

    write_job0(seq_idx, graph, sgen, &mut out)?;
    let mut job_idx = 0;
    for node_idx in &graph.stream_order {
        job_idx += 1;
        let node = sgen.node_at(*node_idx);
        write_job_n(seq_idx, job_idx, node, sgen, &mut out)?;
    }

    Ok(())
}

/// Job 0: constructs all the streams.
fn write_job0<'a, SG: SequenceGen<'a>>(
    seq_idx: usize,
    graph: &NodeGraph,
    sgen: &'a SG,
    out: &mut Box<dyn std::io::Write>,
) -> Result<(), BuilderError> {
    out.write_fmt(format_args!(
        "
pub struct Seq{}Job0 {{
    runtime: runtime::Runtime,
    state: Seq{}State,
}}

impl job::JobRunner for Seq{}Job0 {{
    fn run(&self, _context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {{
        match self.state.run_mut(|s| {{
",
        seq_idx, seq_idx, seq_idx
    ))?;

    write_stream_creation(graph, sgen, out)?;

    out.write_all(
        b"        }) {
            helpers::state_guard::ExecState::Ran(Ok(())) => (),
            helpers::state_guard::ExecState::LockContention => {
                return Err(\"Failed to acquire lock on SeqState\".to_string());
            }
            helpers::state_guard::ExecState::Ran(Err(e)) => {
                return Err(format!(\"Failed to update streams: {}\", e));
            }
        };

        Ok(0)
    }

    fn abort(&self) -> Result<(), String> {
        // As job0 of a sequence, it does not represent a module, so has no abort.
        Ok(())
    }
}
",
    )?;

    Ok(())
}

fn write_job_n<'a, SG: SequenceGen<'a>>(
    seq_idx: usize,
    job_idx: usize,
    node: &ModuleNode,
    sgen: &'a SG,
    out: &mut Box<dyn std::io::Write>,
) -> Result<(), BuilderError> {
    out.write_fmt(format_args!(
        "
pub struct Seq{}Job{} {{
    runtime: runtime::Runtime,
    state: Seq0State,
}}

impl job::JobRunner for Seq{}Job{} {{
    fn run(&self, context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {{
",
        seq_idx, job_idx, seq_idx, job_idx
    ))?;

    if let Some(rps) = &node.module.runtime_param_struct {
        write_params(
            out,
            node,
            rps,
            sgen,
            "        ",
        )?;
    }
    if node.module.stream_struct.is_some() {
        out.write_fmt(format_args!(
            "
        let streams = match self.state.run_mut(|state| {{
            Ok::<Option<cat::CatModuleStream>, String>(state.{}.take())
        }}) {{
            helpers::state_guard::ExecState::LockContention => {{
                return Err(\"Failed to acquire lock on state\".to_string());
            }}
            helpers::state_guard::ExecState::Ran(stream) => stream?.expect(\"stream not found\"),
        }};
",
            &node.node_id,
        ))?;
    }

    out.write_fmt(format_args!(
        "        self.runtime.nodes.{}.exec(context",
        node.node_id
    ))?;
    if node.module.runtime_param_struct.is_some() {
        out.write_all(b", params")?;
    }
    if node.module.stream_struct.is_some() {
        out.write_all(b", streams")?;
    }
    out.write_fmt(format_args!(
        ");
    }}

    fn abort(&self) -> Result<(), String> {{
        if !self.runtime.nodes.{}.abort() {{
            Err(\"Failed to abort {}\".to_string())
        }} else {{
            Ok(())
        }}
    }}
}}

impl Seq{}Job{} {{
    pub fn new_job(runtime: runtime::Runtime, state: Seq{}State) -> job::JobDescription {{
",
        node.node_id, // self.runtime.nodes.{}.abort()
        node.node_id, // Failed to abort {}
        seq_idx,
        job_idx, // impl Seq{}Job{}
        seq_idx, // state: Seq{}State
    ))?;

    let mut listeners = "None";
    if !node.node.event_listeners.is_empty() {
        out.write_all(b"        let listeners = std::collections::HashMap::from([\n")?;
        for listener in &node.node.event_listeners {
            out.write_fmt(format_args!("            (\"{}\", ", listener.name))?;
            let new_sequence_id = sgen.add_ordered_sequence(&listener.actions);
            out.write_fmt(format_args!(
                "job::EventHandler::Sequence({})),\n",
                new_sequence_id,
            ))?;
        }
        out.write_all(b"        ]);\n")?;
        listeners = "Some(listeners)";
    }

    out.write_fmt(format_args!(
        "
        job::JobDescription {{
            name: \"{}\".to_string(),
            source: Source::new(\"{}\", {}, {}),
            listen: \"{}\",
            runner: Box::new(Seq{}Job{} {{ runtime, state }}),
        }}
    }}
}}
",
        node.node.name, // name: \"{}\"
        node.node.source.file,
        node.node.source.line,
        node.node.source.column, // Source::new(\"{}\", {}, {}),
        listeners,               // listen: \"{}\",
        seq_idx,
        job_idx, // Seq{}Job{}
    ))?;

    Ok(())
}

fn write_stream_creation<'a, SG: SequenceGen<'a>>(
    graph: &NodeGraph,
    sgen: &'a SG,
    out: &mut Box<dyn std::io::Write>,
) -> Result<(), BuilderError> {
    // todo!("generate stream creation")
    // Step 1: create the stream variables.  Generally, this will run helpers::fd::mk_pipe().
    //   If they're both Read/Write, then use helpers::mem_rw::make_mem_read_write().
    let streams = stream_pair::BoundStream::from_graph(graph, sgen)?;
    for (stream_idx, stream) in streams.iter().enumerate() {
        match stream {
            stream_pair::BoundStream::MainNamed(std_stream) => {
                if stream.is_strictly_rw() {
                    out.write_fmt(format_args!(
                        "            let s_{} = std::io::{}();\n",
                        stream_idx, std_stream.name,
                    ))?;
                } else {
                    out.write_fmt(format_args!(
                        "            let s_{} = unsafe {{ OwnedFd::from_raw_fd(std::io::{}().as_raw_fd()) }};\n",
                        stream_idx, std_stream.name,
                    ))?;
                }
            }
            stream_pair::BoundStream::MainFd(std_stream) => {
                if stream.is_strictly_rw() {
                    out.write_fmt(format_args!(
                        "            let s_{} = helpers::fd::file_from_fd({});\n",
                        stream_idx, std_stream.fd
                    ))?;
                } else {
                    out.write_fmt(format_args!(
                        "            let s_{} = unsafe {{ OwnedFd::from_raw_fd({}) }};\n",
                        stream_idx, std_stream.fd,
                    ))?;
                }
            }
            crate::server_shell::builder::stream_pair::BoundStream::Pipe(_) => {
                if stream.is_strictly_rw() {
                    out.write_fmt(format_args!(
                        "            let (read_{}, write_{}) = helpers::mem_rw::make_mem_read_write();\n",
                        stream_idx, stream_idx,
                    ))?;
                } else {
                    out.write_fmt(format_args!(
                        "            let (read_{}, write_{}) = helpers::fd::mk_pipe();\n",
                        stream_idx, stream_idx,
                    ))?;
                }
            }
        }
    }

    // Step 2: set the stream state values based on the created variables.
    for ns in stream_pair::NodeStreamStruct::from_streams(
        &streams,
        graph,
        sgen,
    )? {
        let s_struct = &ns.module.as_ref().stream_struct.clone().expect("must have streams");
        out.write_fmt(format_args!(
            "            state.{}.replace({}::{} {{\n",
            ns.node_id,  helpers::module_as_mod_expr(&ns.module), &s_struct.name
        ))?;
        for fixed in &ns.fixed_streams {
            let mut pref = "";
            let mut suff = "";
            if ! fixed.required {
                pref = "Some(";
                suff = ")";
            }
            let v_pre = match &fixed.bound {
                stream_pair::BoundStream::MainNamed(_) => "s",
                stream_pair::BoundStream::MainFd(_) => "s",
                stream_pair::BoundStream::Pipe(_) => match fixed.direction {
                    meta::StreamDirection::Input => "read",
                    meta::StreamDirection::Output => "write",
                }
            };
            if fixed.field_interface == fixed.stream_interface {
                out.write_fmt(format_args!(
                    "                {}: {}{}_{}{},\n",
                    fixed.field_name, pref, v_pre, fixed.stream_idx, suff,
                ))?;
            } else {
                match fixed.field_interface {
                    meta::StreamInterface::ReadWrite => {
                        // Convert from FD to ReadWrite.
                        out.write_fmt(format_args!(
                            "                {}: {}helpers::fd::file_from_fd({}_{}){},\n",
                            fixed.field_name, pref, v_pre, fixed.stream_idx, suff,
                        ))?;
                    }
                    meta::StreamInterface::Fd => {
                        // Convert from ReadWrite to FD.
                        // Based on the logic above, this shouldn't be a valid scenario.
                        out.write_fmt(format_args!(
                            "                {}: {}helpers::fd::owned_from_file({}_{}){},\n",
                            fixed.field_name, pref, v_pre, fixed.stream_idx, suff,
                        ))?;
                    }
                }
            }
        }
        if let Some(var) = &ns.input_variable {
            write_variable_stream_field(var, "read", out)?;
        }
        if let Some(var) = &ns.output_variable {
            write_variable_stream_field(var, "write", out)?;
        }
        out.write_all(b"            });\n")?;
    }

    Ok(())
}

fn write_variable_stream_field(
    var: &VariableNodeStream, mode: &'static str, out: &mut Box<dyn std::io::Write>,
) -> Result<(), BuilderError> {
    out.write_fmt(format_args!(
        "                {}: vec![\n",
        var.field_name,
    ))?;
    for (stream_idx, bound_stream) in &var.streams {
        let v_pre = match &bound_stream {
            stream_pair::BoundStream::MainNamed(_) => "s",
            stream_pair::BoundStream::MainFd(_) => "s",
            // hard-coded to read, since this is the input variable.
            stream_pair::BoundStream::Pipe(_) => mode,
        };
        if var.field_interface == var.stream_interface {
            out.write_fmt(format_args!(
                "                    {}_{},\n",
                v_pre, stream_idx,
            ))?;
        } else {
            match var.field_interface {
                meta::StreamInterface::ReadWrite => {
                    // Convert from FD to ReadWrite.
                    out.write_fmt(format_args!(
                        "                    helpers::fd::file_from_fd({}_{}) as _,\n",
                        v_pre, stream_idx,
                    ))?;
                }
                meta::StreamInterface::Fd => {
                    // Convert from ReadWrite to FD.
                    out.write_fmt(format_args!(
                        "                    helpers::fd::owned_from_file({}_{}) as _,\n",
                        v_pre, stream_idx,
                    ))?;
                }
            }
        }
    }
    out.write_all(b"                ],\n")?;
    Ok(())
}

fn write_params<'a, SG: SequenceGen<'a>>(
    out: &mut Box<dyn std::io::Write>,
    node: &ModuleNode,
    rps: &meta::ModuleStructure,
    sgen: &'a SG,
    indent: &str,
) -> Result<(), BuilderError> {
    // Lookup parameters should happen outside the fetching, in case of reuse.
    // All lookup parameters are assigned to a local variable named "lookup_(node_id)_(state.field)".
    // These look up states, which are called "state_(node_id)".
    let mut value_state = super::values::ConstructValueState::new(sgen, indent);
    let mut values = std::collections::HashMap::new();
    for field in &node.node.runtime_parameters.0 {
        // TODO ensure the runtime parameters has this field, and that it aligns with the value type.
        values.insert(field.name.clone(), value_state.construct_value(&field.value)?);
    }

    out.write_all(value_state.state_values().as_bytes())?;
    out.write_fmt(format_args!(
        "{}let params = {}{} {{\n",
        indent, as_mod_expr(node), &rps.name
    ))?;

    for (param, value) in values {
        out.write_fmt(format_args!(
            "{}    {}: {},\n",
            indent, param, value
        ))?;
    }

    out.write_all(b"        };\n")?;
    Ok(())
}
