//! Sends constructed text into a stream.

use std::io::Write;

use crate::shell_lib::{
    helpers::log::Logger,
    structure::{ExecCtx, InitCtx, ScriptExit, Source, job, meta},
};

pub fn module_meta() -> meta::ModuleMeta {
    meta::ModuleMeta {
        name: "echo".to_string(),
        description: "Echo text to the stream".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec![
            "shell_lib".to_string(),
            "modules".to_string(),
            "echo".to_string(),
        ],
        dependencies: vec![],
        os_dependencies: vec![],
        job: Some(meta::JobModuleStruct {
            instance_struct: "EchoModule".to_string(),
            compile_param_struct: None,
            runtime_param_struct: Some(meta::ModuleStructure {
                name: "EchoModuleRuntimeParams".to_string(),
                new: None,
                fields: vec![meta::NamedValue {
                    name: "text".to_string(),
                    value_type: meta::ValueType::String,
                    optional: false,
                }],
            }),
            state_struct: None,
            stream_struct: Some(meta::ModuleStreamStructure {
                name: "EchoModuleStream".to_string(),
                fixed_streams: vec![meta::FixedStreamDef {
                    name: Some("output".to_string()),
                    fd_index: Some(0),
                    stream_type: meta::StreamType::Output(meta::StreamInterface::ReadWrite),
                    required: true,
                }],
                input_variable: None,
                output_variable: None,
            }),
            handlers: vec![],
        }),
        command: None,
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
    logger: Logger,
}

impl EchoModule {
    pub fn new(source: Source, ctx: &mut dyn InitCtx) -> Self {
        EchoModule {
            logger: Logger::new(&source, ctx),
            source,
        }
    }

    pub fn exec(
        &self,
        ctx: &mut dyn ExecCtx,
        params: EchoModuleRuntimeParams,
        mut streams: EchoModuleStream,
    ) -> Result<job::ExitCode, ScriptExit> {
        self.logger
            .debug(ctx, format_args!("Echoing text: {}", params.text))?;
        match streams.fd_0.write_all(params.text.as_bytes()) {
            Ok(_) => Ok(0),
            Err(e) => Err(format!("Failed to write to output stream: {}", e).into()),
        }
    }
}
