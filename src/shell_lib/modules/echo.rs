//! Sends constructed text into a stream.

use std::io::Write;

use crate::shell_lib::{compile::{job, meta, source::Source}, runtime::event_bus};

pub fn module_meta() -> meta::ModuleMeta {
    meta::ModuleMeta {
        name: "echo".to_string(),
        description: "Echo text to the stream".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec!["shell_lib".to_string(), "modules".to_string(), "echo".to_string()],
        dependencies: vec![],
        os_dependencies: vec![],
        instance_struct: "EchoModule".to_string(),
        compile_param_struct: None,
        runtime_param_struct: Some(meta::ModuleStructure {
            name: "EchoModuleRuntimeParams".to_string(),
            new: None,
            fields: vec![
                meta::NamedValue {
                    name: "text".to_string(),
                    value_type: meta::ValueType::String,
                    optional: false,
                },
            ],
        }),
        state_struct: None,
        stream_struct: Some(meta::ModuleStreamStructure {
            name: "EchoModuleStream".to_string(),
            fixed_streams: vec![
                meta::FixedStreamDef {
                    name: Some("output".to_string()),
                    fd_index: Some(0),
                    stream_type: meta::StreamType::Output(meta::StreamInterface::ReadWrite),
                    required: true,
                },
            ],
            input_variable: None,
            output_variable: None,
        }),
        handlers: vec![],
    }
}

#[derive(Clone, Debug)]
pub struct EchoModuleRuntimeParams {
    pub text: String,
}

pub struct EchoModuleStream {
    pub fd_0: Box<dyn Write + Send + Sync>,
}

pub struct EchoModule {
    source: Source,
}

impl EchoModule {
    pub fn new(source: Source) -> Self {
        EchoModule { source }
    }

    pub fn exec(&self, context: Box<dyn job::JobRunnerContext>, params: EchoModuleRuntimeParams, mut streams: EchoModuleStream) -> Result<job::ExitCode, String> {
        event_bus::send_debug_event(&context, &self.source, format!("Echoing text: {}", params.text))?;
        match streams.fd_0.write_all(params.text.as_bytes()) {
            Ok(_) => Ok(0),
            Err(e) => Err(format!("Failed to write to output stream: {}", e)),
        }
    }

    pub fn abort(&self) -> bool {
        // This module does not have any state to abort.
        // It will stop writing when the output pipe is closed.
        true
    }
}
