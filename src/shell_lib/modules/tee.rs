//! Splits one input stream into 0 or more output streams.
//! Because of this, it also acts like a `>/dev/null`.

use std::{collections::VecDeque, io::Read, os::fd::OwnedFd};

use crate::shell_lib::runtime::event_bus;
use crate::shell_lib::{
    helpers::abort_handler,
    structure::{job, meta, source::Source},
};

const BUFFER_SIZE: usize = 8192;

pub fn module_meta() -> meta::ModuleMeta {
    meta::ModuleMeta {
        name: "tee".to_string(),
        description: "Split one input stream into zero or more output streams".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Native Shell Developers".to_string()],
        mod_name: vec![
            "shell_lib".to_string(),
            "modules".to_string(),
            "tee".to_string(),
        ],
        dependencies: vec![],
        os_dependencies: vec![],
        instance_struct: "TeeModule".to_string(),
        compile_param_struct: None,
        runtime_param_struct: None,
        state_struct: None,
        stream_struct: Some(meta::ModuleStreamStructure {
            name: "TeeModuleStream".to_string(),
            fixed_streams: vec![meta::FixedStreamDef {
                name: Some("input".to_string()),
                fd_index: Some(0),
                stream_type: meta::StreamType::Input(meta::StreamInterface::Fd),
                required: true,
            }],
            input_variable: None,
            output_variable: Some(meta::VariableStreamField {
                field_name: "output".to_string(),
                stream_type: meta::StreamInterface::ReadWrite,
                min_count: 0,
                max_count: u16::MAX,
            }),
        }),
        handlers: vec![],
    }
}

pub struct TeeModuleStream {
    pub fd_0: OwnedFd,
    pub output: Vec<Box<dyn std::io::Write + Send + Sync>>,
}

pub struct TeeModule {
    source: Source,
    state: abort_handler::RunState<abort_handler::FdIn>,
}

impl TeeModule {
    pub fn new(source: Source) -> Self {
        TeeModule {
            state: abort_handler::RunState::new(),
            source,
        }
    }

    pub fn exec(
        &self,
        context: Box<dyn job::JobRunnerContext>,
        mut streams: TeeModuleStream,
    ) -> Result<job::ExitCode, String> {
        let mut inp;
        let mut active = VecDeque::with_capacity(streams.output.len());
        {
            let (fd_in, reader) = abort_handler::FdIn::new(streams.fd_0);
            for fd in streams.output {
                active.push_back(fd);
            }
            self.state.start(fd_in);
            inp = reader;
        }

        let mut buf = [0u8; BUFFER_SIZE];

        // Read blocks of input until the input is closed.
        // Part of this functionality is to act as /dev/null, which means it must keep running even if there are no outputs.
        loop {
            let size = match inp.read(&mut buf) {
                Ok(0) => {
                    // EOF reached, stop processing.
                    let _ = self.stop();
                    return Ok(0);
                }
                Ok(size) => size,
                Err(e)
                    if e.kind() == std::io::ErrorKind::Interrupted
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    // Just loop again.
                    continue;
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // EOF means cannot continue reading, so stop immediately.
                    let _ = self.stop();
                    return Ok(0);
                }
                Err(e) => {
                    let _ = event_bus::send_error_event(
                        &context,
                        &self.source,
                        format!("Failed to read from input stream: {}", e),
                    );
                    let _ = self.stop();
                    return Ok(1);
                }
            };

            let mut valid = VecDeque::with_capacity(active.len());
            for mut out in active {
                // Write the data to each output stream.
                loop {
                    match out.write_all(&buf[..size]) {
                        Ok(_) => {
                            // If we successfully wrote to the output stream, keep it in the valid list.
                            valid.push_back(out);
                            break;
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                            // Retry.
                            continue;
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // Not a good solution.  However, other solutions mean dropping data.
                            // This puts back pressure on the input stream.
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            continue;
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                            // EOF on the output stream, but the rest of the output streams are still valid.
                            break;
                        }
                        Err(e) => {
                            let _ = event_bus::send_error_event(
                                &context,
                                &self.source,
                                format!("Failed to write to output stream: {}", e),
                            );
                            let _ = self.stop();
                            return Ok(1);
                        }
                    }
                }
            }

            active = valid;
        }
    }

    /// Required module abort handler.
    pub fn abort(&self) -> bool {
        let _ = self.stop();
        true
    }

    /// Stop the stream reading.
    fn stop(&self) -> Result<(), String> {
        self.state.on_stop(|e| e.stop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_lib::helpers::fd::{VecWriter, make_fd_reader};
    use std::cell::RefCell;

    struct EventBus {
        name: &'static str,
        msgs: RefCell<Vec<(String, job::EventPayload)>>,
    }
    impl job::JobRunnerContext for EventBus {
        fn send_event(
            &self,
            event_ref: &job::EventRef,
            payload: job::EventPayload,
        ) -> Result<(), String> {
            println!("{} {}: {}", self.name, event_ref, payload);
            self.msgs.borrow_mut().push((event_ref.clone(), payload));
            Ok(())
        }
    }

    #[test]
    fn test_zero_outputs() {
        let bus = EventBus {
            name: "test_zero_inputs",
            msgs: RefCell::new(vec![]),
        };
        let module = TeeModule::new(Source::default());
        let fd_0 = make_fd_reader(b"hello\nworld\n");
        let streams = TeeModuleStream {
            fd_0,
            output: vec![],
        };
        let code = module.exec(Box::new(bus), streams).unwrap();
        assert_eq!(code, 0);
    }

    #[test]
    fn test_single_output() {
        let bus = EventBus {
            name: "test_single_input_default_separator",
            msgs: RefCell::new(vec![]),
        };
        let module = TeeModule::new(Source::default());
        let (ov1, out1) = VecWriter::new_pair();
        let fd_0 = make_fd_reader(b"hello\nworld\n");
        let streams = TeeModuleStream {
            fd_0,
            output: vec![ov1],
        };
        let code = module.exec(Box::new(bus), streams).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*out1.read().unwrap(), b"hello\nworld\n");
    }

    #[test]
    fn test_two_outputs() {
        let bus = EventBus {
            name: "test_single_input_default_separator",
            msgs: RefCell::new(vec![]),
        };
        let module = TeeModule::new(Source::default());
        let data = b"a1\na2\na3";
        let fd_0 = make_fd_reader(data);
        let (ov1, out1) = VecWriter::new_pair();
        let (ov2, out2) = VecWriter::new_pair();
        let streams = TeeModuleStream {
            fd_0,
            output: vec![ov1, ov2],
        };
        let code = module.exec(Box::new(bus), streams).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*out1.read().unwrap(), data);
        assert_eq!(*out2.read().unwrap(), data);
    }

    #[test]
    fn test_two_outputs_three_buffer_reads() {
        let bus = EventBus {
            name: "test_two_inputs_separator_two_chars",
            msgs: RefCell::new(vec![]),
        };
        let module = TeeModule::new(Source::default());
        let mut data = [0u8; BUFFER_SIZE * 2 + 3];
        for i in 0..BUFFER_SIZE * 2 + 3 {
            data[i] = (i % 256) as u8;
        }
        let fd_0 = make_fd_reader(&data);
        let (ov1, out1) = VecWriter::new_pair();
        let (ov2, out2) = VecWriter::new_pair();
        let streams = TeeModuleStream {
            fd_0,
            output: vec![ov1, ov2],
        };
        let code = module.exec(Box::new(bus), streams).unwrap();
        assert_eq!(code, 0);
        assert_eq!(*out1.read().unwrap(), data);
        assert_eq!(*out2.read().unwrap(), data);
    }

    // TODO add a test for mid-read abort.
}
