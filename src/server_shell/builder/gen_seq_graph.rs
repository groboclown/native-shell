//! Generate a seq*.rs file for a script.

use super::helpers::{as_mod_expr, rust_file_header};
use super::node_graph::NodeGraph;
use super::parse_node::ModuleNode;
use super::sequence::{SeqIndex, SequenceGen};
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::BuilderError;
use crate::shell_lib::compile::meta;

pub fn write_graph_seq<'a, SW: SourceWriter, SG: SequenceGen<'a>>(
    seq_idx: SeqIndex,
    graph: &NodeGraph,
    sgen: &'a SG,
    out: &SW,
) -> Result<(), BuilderError> {
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

    write_job0(seq_idx, graph, &mut out)?;
    let mut job_idx = 0;
    for node_idx in &graph.stream_order {
        job_idx += 1;
        let node = sgen.node_at(*node_idx);
        write_job_n(seq_idx, job_idx, node, sgen, &mut out)?;
    }

    Ok(())
}

/// Job 0: constructs all the streams.
fn write_job0(
    seq_idx: usize,
    graph: &NodeGraph,
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

    write_stream_creation(graph, out)?;

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

fn write_stream_creation(
    graph: &NodeGraph,
    out: &mut Box<dyn std::io::Write>,
) -> Result<(), BuilderError> {
    // todo!("generate stream creation")
    println!("TODO: generate stream creation.");
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
