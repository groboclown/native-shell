//! Provide 'cat' functionality for the shell.

use std::{fs::File, io::{Read, Write}, os::fd::OwnedFd};

use crate::shell_lib::{compile::{job, meta::{FixedStreamDef, ModuleMeta, ModuleStreamStructure, ModuleStructure, NamedValue, StreamInterface, StreamType, ValueType}}, helpers::fd::file_from_fd};

const BUFFER_SIZE: usize = 8192;
const RETRY_TIME: std::time::Duration = std::time::Duration::from_millis(10);

pub fn module_meta() -> ModuleMeta {
    ModuleMeta {
        name: "cat".to_string(),
        description: "Concatenate and display files".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec!["shell_lib".to_string(), "modules".to_string(), "cat".to_string()],
        dependencies: vec![],
        os_dependencies: vec![],
        instance_struct: "CatModule".to_string(),
        compile_param_struct: None,
        runtime_param_struct: Some(ModuleStructure {
            name: "CatModuleRuntimeParams".to_string(),
            new: None,
            fields: vec![
                NamedValue {
                    name: "filenames".to_string(),
                    value_type: ValueType::StringList,
                    optional: false,
                },
            ],
        }),
        state_struct: None,
        stream_struct: Some(ModuleStreamStructure {
            name: "CatModuleStream".to_string(),
            fixed_streams: vec![
                FixedStreamDef {
                    name: Some("output".to_string()),
                    fd_index: Some(0),
                    stream_type: StreamType::Output(StreamInterface::Fd),
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
pub struct CatModuleRuntimeParams {
    pub filenames: Vec<String>,
}

pub struct CatModuleStream {
    pub fd_0: OwnedFd,
}

pub struct CatModule {
}

impl CatModule {
    pub fn new() -> Self {
        CatModule {}
    }

    pub fn exec(&self, _context: Box<dyn job::JobRunnerContext>, params: CatModuleRuntimeParams, mut streams: CatModuleStream) -> Result<job::ExitCode, String> {
        let mut out = file_from_fd(streams.fd_0);
        let mut buf = [0 as u8; BUFFER_SIZE];
        for filename in params.filenames {
            let mut file = std::fs::File::open(filename).map_err(|e| e.to_string())?;
            loop {
                let bytes_read = file.read(&mut buf).map_err(|e| e.to_string())?;
                if bytes_read == 0 {
                    break; // EOF
                }
                match out.write_all(&buf[..bytes_read]) {
                    Ok(_) => {},
                    Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                        // If the output pipe is broken, this must stop writing.
                        // Note that this isn't an error, just a signal to stop.
                        return Ok(0);
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                        // Take interrupted problems as notices to the
                        // executable to examine the current state.
                        continue;
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // If the write would block, we can retry.
                        // Give the process a bit of time.
                        std::thread::sleep(RETRY_TIME);
                        continue;
                    },
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Ok(0)
    }

    pub fn abort(&self) -> bool {
        // This module does not have any state to abort.
        // It will stop writing when the output pipe is closed.
        true
    }
}
