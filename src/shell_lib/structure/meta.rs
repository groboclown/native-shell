//! Information that describes the module.
//!
//! Eventually, this should move into macros for automatically constructing
//! this from the code.

use crate::shell_lib::structure::event::EventKind;

/// Allowed types for module input parameters and output states.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueType {
    String,
    Float,
    Boolean,
    StringList,
    FloatList,
    BooleanList,
    StringMap,
    FloatMap,
    BooleanMap,
    StringListMap, // HashMap<String, Vec<String>>
    StringMapList, // Vec<HashMap<String, String>>
    Enum(Vec<String>),
}

/// A field / parameter definition for a module.
///
/// If `optional` is true, then the user does not need to provide a value for this field,
/// and the code representation will wrap the type in an `Option<ValueType>`.
/// Note that the values settable by a user do not include 'null' like values, so 'optional'
/// only means that the user did not set the value.
#[derive(Clone, Debug)]
pub struct NamedValue {
    pub name: String,
    // pub description: String,  // use this?
    pub value_type: ValueType,
    pub optional: bool,
}

/// The Rust code that evaluates a parameter value at runtime.
pub type ParameterValueCode = String;

#[derive(Clone, Debug)]
pub enum StreamDeclaration {
    /// Associated to the module's declared fixed stream name.
    Name(String),

    /// Associated to the module's declared fixed stream file descriptor index.
    FdIndex(u16),

    /// Associated to the module's declared variable stream.
    /// The string represents the name in the parameter structure.
    VariableStream(String, StreamDirection),
}

#[derive(Clone, Debug)]
pub enum StreamDirection {
    /// The stream is an input stream.
    Input,

    /// The stream is an output stream.
    Output,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamType {
    /// The module reads from the stream.
    Input(StreamInterface),

    /// The module writes to the stream.
    Output(StreamInterface),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamInterface {
    /// Either a std::io::Read or a std::io::Write trait.
    /// These will be passed inside a Box.
    ReadWrite,

    /// A file descriptor.
    FD,
}

/// Defines a structure owned by the module that the script will use to interact with the module.
#[derive(Clone, Debug)]
pub struct ModuleStructure {
    /// The name of the structure.
    /// It must share the same mod name as the module.
    pub name: String,

    /// The list of public fields in the structure.
    pub fields: Vec<NamedValue>,

    /// The 'new' function that creates the structure.
    /// Modules must provide this if the structure contains non-public fields.
    pub new: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FixedStreamDef {
    /// The name of the stream.
    pub name: Option<String>,

    /// The file descriptor index for the stream.
    /// The actual file descriptor passed to the module's code can be any number, but it will assign it to this index.
    /// A stream must have at least one of the name or index; both indicates the name acts as an alias.
    /// The fd_index will be referenced in the stream struct as `fd_(index)`; if the name is also
    /// provided, then only the `fd_(index)` should exist.
    /// If the StreamType is `Input(Fd)` or `Output(Fd)`, then the field must be a `std::os::fd::OwnedFd`.
    /// If the StreamType is `Input(ReadWrite)`, then the field must be a `Box<dyn std::io::Read + Send + Sync>`.
    /// If the StreamType is `Output(ReadWrite)`, then the field must be a `Box<dyn std::io::Write + Send + Sync>`.
    pub fd_index: Option<usize>,

    /// The type of the stream.
    pub stream_type: StreamType,

    /// Whether the script must provide the stream.
    pub required: bool,
}

/// Defines a variable stream field for the module's stream structure.
/// If the StreamInterface is `Fd`, then the field must be a `Vec<std::os::fd::OwnedFd>`.
/// If the StreamInterface is `ReadWrite` and an input stream, then the field must be a
///    `Vec<Box<dyn std::io::Read + Send + Sync>>`.
/// If the StreamInterface is `ReadWrite` and an output stream, then the field must be a
///    `Vec<Box<dyn std::io::Write + Send + Sync>>`.
#[derive(Clone, Debug)]
pub struct VariableStreamField {
    pub field_name: String,
    pub stream_type: StreamInterface,
    pub min_count: u16,
    pub max_count: u16,
}

/// Defines the module's stream structure.
///
/// If the structure does not use a variable stream type, then it should use
/// `Fd` as the stream type, and set the min and max counts to 0.
///
/// The structure must define the stream's fixed fields are either the name
/// (if only the name is given), or `fd_(index)` where `index` is the
/// file descriptor index.
#[derive(Clone, Debug)]
pub struct ModuleStreamStructure {
    pub name: String,
    pub fixed_streams: Vec<FixedStreamDef>,
    pub input_variable: Option<VariableStreamField>,
    pub output_variable: Option<VariableStreamField>,
}

/// A version bit for a crate dependency.
/// The first value is the separator between the previous value and the next (e.g. '.' or '-' or '\0' for none).,
#[derive(Clone, Debug)]
pub enum VerBit {
    S((char, String)),
    N((char, u32)),
}

#[derive(Clone, Debug)]
pub struct CrateDependency {
    pub name: String,
    pub version: Vec<VerBit>,
    pub features: Vec<String>,
}

/// Meta-information about a function that receives events.
#[derive(Clone, Debug)]
pub struct EventFunc {
    pub func_name: String,
    pub event_name: String,
    pub kind: EventKind,
}

impl EventFunc {
    pub fn sig(name: &str) -> Self {
        Self {
            func_name: name.to_string(),
            event_name: name.to_string(),
            kind: EventKind::Signal,
        }
    }
    pub fn msg(name: &str) -> Self {
        Self {
            func_name: name.to_string(),
            event_name: name.to_string(),
            kind: EventKind::Message,
        }
    }
}

/// The module metadata definition.
///
/// This describes basic information about the module itself, as well as
/// information used to generate Rust code to interact with the module.
///
pub struct ModuleMeta {
    /// The human readable module name.
    pub name: String,
    /// The description of the module.
    pub description: String,
    /// The version of the module.
    pub version: String,
    /// The authors of the module.
    pub authors: Vec<String>,

    // Below here are Rust reflection of the module's source.
    /// A list of the Cargo.toml `[dependencies]` lines this module depends on.
    pub dependencies: Vec<CrateDependency>,

    /// A list of the Cargo.toml OS dependencies this module requires, where the first item is the
    /// `[target.'cfg(target_os = "NAME")'.dependencies]` NAME value, and the second is the
    /// dependency line in that section.
    pub os_dependencies: Vec<(String, String)>,

    /// The module's Rust mod name, divided along paths.
    /// The builder uses this to construct the module's Rust mod name.
    /// The builder will join it with `::` to form the full path.
    pub mod_name: Vec<String>,

    /// If the module works in the Job role, then it adds this structure.
    pub job: Option<JobModuleStruct>,

    /// If the module works in the Command role, then it adds this structure.
    pub command: Option<CommandModuleStruct>,
}

/// Modules that take on the Job role define this meta-structure.
#[derive(Clone)]
pub struct JobModuleStruct {
    /// The module's instance struct name.
    ///
    /// ### The 'new' function
    ///
    /// The struct's impl must include a `new(#[parameter list]) -> Self` function, whose
    /// arguments depend upon the contents of this structure:
    ///
    /// * if `compile_param_struct` is None, then the signature will look like:
    ///     ```rust
    ///     pub fn new(
    ///       source: shell_lib::structure::Source,
    ///       ctx: &mut dyn shell_lib::structure::InitCtx,
    ///     ) -> Self
    ///     ```
    /// * if `compile_param_struct` is Some, then the signature will look like:
    ///   ```rust
    ///     pub fn new(
    ///       source: shell_lib::structure::Source,
    ///       ctx: &mut dyn shell_lib::structure::InitCtx,
    ///       params: #[compile_param_struct.name],
    ///     ) -> Self
    ///   ```
    /// where:
    ///
    /// * `source`: the location in the source script that defined this job.  This allows
    ///   the module to enhance its debugging.
    /// * `ctx`: context for initializing the module with the larger system.
    /// * `params`: parameters provided by the user script.
    ///
    /// The `new` function does not allow for error returning.
    /// Errors should come during the execution of the job - if execution ends up not running
    /// the job, then setup errors should not stop the script from running.
    /// `panic` should happen only in cases of issues arising from the builder or other
    /// critical script usage problems.
    ///
    /// ### The 'exec' function
    ///
    /// The struct's impl must include a method `exec` that runs the job's behavior.  It
    /// signature depends upon the contents of this structure:
    ///
    /// * if `runtime_param_struct` and `stream_struct` are None, then it must look like:
    ///     ```rust
    ///     pub fn exec(
    ///       &self,
    ///       ctx: &mut dyn shell_lib::structure::ExecInit,
    ///     ) -> Result<shell_lib::structure::ScriptExit, shell_lib::structure::ScriptExit>
    ///     ```
    /// * if `runtime_param_struct` is Some and `stream_struct` is None, then it must look like:
    ///     ```rust
    ///     pub fn exec(
    ///       &self,
    ///       ctx: &mut dyn shell_lib::structure::ExecInit,
    ///       runtime: #[runtime_param_struct.name],
    ///     ) -> Result<shell_lib::structure::ScriptExit, shell_lib::structure::ScriptExit>
    ///     ```
    /// * if `runtime_param_struct` is None and `stream_struct` is Some, then it must look like:
    ///     ```rust
    ///     pub fn exec(
    ///       &self,
    ///       ctx: &mut dyn shell_lib::structure::ExecInit,
    ///       mut streams: #[stream_struct.name],
    ///     ) -> Result<shell_lib::structure::ScriptExit, shell_lib::structure::ScriptExit>
    ///     ```
    ///     It's the responsibility of the `exec` function to close all streams passed to it.
    /// * if `runtime_param_struct` and `stream_struct` are Some, then it must look like:
    ///     ```rust
    ///     pub fn exec(
    ///       &self,
    ///       ctx: &mut dyn shell_lib::structure::ExecInit,
    ///       runtime: #[runtime_param_struct.name],
    ///       streams: #[stream_struct.name],
    ///     ) -> Result<shell_lib::structure::ScriptExit, shell_lib::structure::ScriptExit>
    ///     ```
    ///     It's the responsibility of the `exec` function to close all streams passed to it.
    ///
    /// The method returns a result with the same type for both the ok and err result.
    /// This allows for easier use of `?` for early exit scenarios.
    ///
    /// ### The `state` function
    ///
    /// If the job includes Some for its `state_struct`, then the struct's impl must also
    /// include the `state` method with the signature:
    ///
    ///     ```rust
    ///     pub fn state(&self) -> #[state_struct.name]
    ///     ```
    ///
    /// This returns a copy of the internal state of the job, which may change as the
    /// `exec` runs.  The struct must take caution to make the method thread safe.
    pub instance_struct: String,

    /// The module's state structure definition.
    /// Leave as `None` to note that the module does not maintain state outside of job runs.
    /// If Some, it must exist in the module's `mod_name` module.
    pub state_struct: Option<ModuleStructure>,

    /// The compile-time parameter structure accepted by the module's `new` call.
    /// Leave as None to indicate that the module does not accept a parameter.
    /// If Some, it must exist in the module's `mod_name` module.
    pub compile_param_struct: Option<ModuleStructure>,

    /// The runtime parameter value structure name accepted by the module's `exec` call.
    /// If Some, it must exist in the module's `mod_name` module.
    pub runtime_param_struct: Option<ModuleStructure>,

    /// The name of the structure that contains the stream instance information.
    /// Other jobs build the contents of the stream.  The `exec` method will take this
    /// as a parameter.
    /// If Some, it must exist in the module's `mod_name` module.
    pub stream_struct: Option<ModuleStreamStructure>,

    /// List of event handlers to automatically bind.
    /// Without this, the `new` function must explicitly bind handlers.
    /// When a handler is given, the 'func_name' must exist as a function in the
    /// module structure, and, depending on the 'mutable' and 'kind' values, one of:
    ///   ```rust
    ///   pub fn #[event.func_name](&self, &mut dyn ExecCtx, EventRef, &String) -> Result<(), ScriptExit>
    ///   pub fn #[event.func_name](&self, &mut dyn ExecCtx, EventRef, SignalCode) -> Result<(), ScriptExit>
    ///   ```
    pub handlers: Vec<EventFunc>,
}

/// Modules that take on the command role define this meta-structure.
///
/// * The module may include `argv` parameter, which will be populated with the script's command line arguments.  If included,
///   it must be of type StringList.
/// * The module may include `environ` parameter, which will be populated with the script's environment variables.
///   If included, it must be of type StringMap.
/// * The module does not use the `exec()` function like a normal module.  Instead, it must
///   implement the trait `CommandImpl`.
/// * In the final compiled version, the module's streams will create:
///   * writer stream fd 0 (stdin),
///   * reader stream fd 1 (stdout),
///   * reader stream fd 2 (stderr).
///   (the read/write are reversed so that jobs that use them will have the correct write/read direction).
/// * If the struct has a Some state_struct, then the function must provide a
///   `pub fn state(&self) -> &#[state_struct.name]` function.
pub struct CommandModuleStruct {
    /// The module's instance struct name.
    /// It must implement the `CommandImpl` trait and exist in the module's `mod_name` module.
    pub instance_struct: String,

    /// The module's state structure definition.
    pub state_struct: Option<ModuleStructure>,

    /// The compile-time parameter structure accepted by the module's `new` call.
    /// Leave as None to indicate that the module does not accept a parameter.
    pub compile_param_struct: Option<ModuleStructure>,

    /// List of event handlers to automatically bind.
    /// Without this, the `new` function must explicitly bind handlers.
    /// When a handler is given, the 'func_name' must exist as a function in the
    /// module structure, and, depending on the 'mutable' and 'kind' values, one of:
    ///   ```rust
    ///   pub fn #[event.func_name](&self, &mut dyn ExecCtx, EventRef, &String) -> Result<(), ScriptExit>
    ///   pub fn #[event.func_name](&self, &mut dyn ExecCtx, EventRef, SignalCode) -> Result<(), ScriptExit>
    ///   pub fn #[event.func_name](&mut self, &mut dyn ExecCtx, EventRef, &String) -> Result<(), ScriptExit>
    ///   pub fn #[event.func_name](&mut self, &mut dyn ExecCtx, EventRef, SignalCode) -> Result<(), ScriptExit>
    ///   ```
    pub handlers: Vec<EventFunc>,
}

impl Into<JobModuleStruct> for &CommandModuleStruct {
    fn into(self) -> JobModuleStruct {
        JobModuleStruct {
            instance_struct: self.instance_struct.clone(),
            state_struct: self.state_struct.clone(),
            compile_param_struct: self.compile_param_struct.clone(),
            runtime_param_struct: None,
            stream_struct: None,
            handlers: self.handlers.clone(),
        }
    }
}

/// Helper function to create a crate dependency that uses the latest version.
pub fn as_latest_crate_dependency<'a>(name: &'a str) -> CrateDependency {
    CrateDependency {
        name: name.to_string(),
        version: vec![],
        features: vec![],
    }
}
