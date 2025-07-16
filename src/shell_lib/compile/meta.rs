//! Information that describes the module.
//! 
//! Eventually, this should move into macros for automatically constructing
//! this from the code.

/// Allowed types for module input parameters and output states.
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
    Enum(Vec<String>),
}

/// A field / parameter definition for a module.
/// 
/// If `optional` is true, then the user does not need to provide a value for this field,
/// and the code representation will wrap the type in an `Option<ValueType>`.
pub struct NamedValue {
    pub name: String,
    // pub description: String,  // use this?
    pub value_type: ValueType,
    pub optional: bool,   
}

/// The code that evaluates a parameter value at runtime.
pub type ParameterValueCode = String;

#[derive(Clone, Debug)]
pub enum StreamDeclaration {
    /// Associated to the module's declared fixed stream name.
    Name(String),

    /// Associated to the module's declared fixed stream file descriptor index.
    /// The first item is the file descriptor index as declared in the module's definition.
    /// The second is the name of the stream item in the parameter structure.
    FdIndex(u16, String),

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

pub enum StreamType {
    /// The module reads from the stream.
    Input(StreamInterface),

    /// The module writes to the stream.
    Output(StreamInterface),
}

pub enum StreamInterface {
    /// Either a std::io::Read or a std::io::Write trait.
    /// These will be passed inside a Box.
    ReadWrite,

    /// A file descriptor.
    Fd,
}

pub struct FixedStreamDef {
    /// The name of the stream.
    pub name: Option<String>,
    // pub description: String,  // use this?

    /// The file descriptor index for the stream.
    /// The actual file descriptor passed to the module's code can be any number, but it will assign it to this index.
    /// A stream must have at least one of the name or index; both indicates the name acts as an alias.
    /// The fd_index will be referenced in the stream struct as `fd_(index)`; if the name is also
    /// provided, then only the `fd_(index)` should exist.
    /// If the StreamType is `Input(Fd)` or `Output(Fd)`, then the field must be a `std::os::fd::OwnedFd`.
    /// If the StreamType is `Input(ReadWrite)`, then the field must be a `Box<dyn std::io::Read>`.
    /// If the StreamType is `Output(ReadWrite)`, then the field must be a `Box<dyn std::io::Write>`.
    pub fd_index: Option<usize>,

    /// The type of the stream.
    pub stream_type: StreamType,

    /// Whether the script must provide the stream.
    pub required: bool,
}

/// Defines a structure owned by the module that the script will use to interact with the module.
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

/// Defines a variable stream field for the module's stream structure.
/// If the StreamInterface is `Fd`, then the field must be a `Vec<std::os::fd::OwnedFd>`.
/// If the StreamInterface is `ReadWrite` and an input stream, then the field must be a `Vec<Box<dyn std::io::Read>>`.
/// If the StreamInterface is `ReadWrite` and an output stream, then the field must be a `Vec<Box<dyn std::io::Write>>`.
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
pub struct ModuleStreamStructure {
    pub name: String,
    pub fixed_streams: Vec<FixedStreamDef>,
    pub input_variable: Option<VariableStreamField>,
    pub output_variable: Option<VariableStreamField>,
}

/// The module metadata definition.
/// 
/// This describes basic information about the module itself, as well as
/// information used to generate Rust code to interact with the module.
/// 
/// Each module must have these items defined:
/// 
/// * `mod_name`: Used to construct the module's Rust mod name.
///     It will be joined with `::` to form the full path.
/// * `instance_struct`: The name of the module's instance `struct`.
///     It must implement the `new()`, `exec()`, `state()` and actions.
///     * `new() -> Self`: The constructor for the module.
///         Parameter order:
///           * `params: #[compile_param_struct.name]`: The compile-time parameters.
///             Only passed if the compile_param_struct is Some.
///     * `exec(&self) -> Result<i16, String>`: The function that executes the module.
///         Parameter order:
///           * `context: Box<dyn JobRunnerContext>`: Allows for limited interaction with the engine.
///           * `params: #[runtime_param_struct.name]`: The runtime parameters.
///             Only passed if the runtime_param_struct is Some.
///           * `mut streams: #[stream_struct.name]`: The stream structure.
///             Only passed if the stream_struct is Some.
///         This function returns the exit code of the module, or an error if the
///         parameter setup was wrong and the module could not start.
///         The function is required to clean up its state on exit, including closing all
///         streams passed to it.
///     * `state(&self) -> #[state_struct.name]`: The function that returns the module's state.
///         Only needed if the `state_struct` is Some.
///         The state returns the result of the most recent execution of the module instance.
///     * `abort(&self) -> bool`: The function that aborts the module.
///         An implicit action all modules must implement.  It should attempt to stop the module from running.
///         The script engine will only call this if the module is running, but if the abort is registered
///         through an event group, then it may be called before or after it runs.
///     * `#[handler name](&self, Box<dyn JobRunnerContext>, #[handler_params]) -> Result<i16, String>`: The handler functions.
/// * `state_struct`: The name of the module's state `type strut`.
///     It's returned by the module's `get_state()` method.
/// * `state_fields`: A list of states the module reports, for use by the compiled code to get.
///     These must be public fields in the state structure.
/// * `compile_param_struct`: The module's parameter structure, passed into the struct's `new()` function.
///     These will be compile-time parameters, so script authors will have little flexibility
///     in using them.  Therefore, use them sparingly.
/// * `runtime_param_struct`: The module's runtime value structure, passed into the struct's `exec()` function.
///     This is in addition to the standard environment values provided by the shell.
/// * `stream_struct`: The name of the structure that contains the stream instance information.
/// * `handlers`: A list of available handlers and their parameters.
///     The first element is the action's method name which matches with the name of the action available to the script author,
///     the second is the list of parameters the script author passes to the action.
/// * `dependencies`: A list of the Cargo.toml `[dependencies]` lines this module depends on.
/// * `os_dependencies`: A list of the Cargo.toml OS dependencies this module requires, where the first item is the
///     `[target.'cfg(target_os = "NAME")'.dependencies]` NAME value, and the second is the dependency line in that section.
/// 
/// It's the responsibility of the module to close all streams passed to it.
/// 
/// Separate from this is the "main" module.  Each AST must have exactly one node named "main", which follows the
/// rules of a main module.  The main module's provided streams should include the standard input, output, and error streams,
/// but it doesn't need to; the builder will make these available to other nodes.  It can also provide handlers to perform
/// special functions.  Uniquely, though, the main module does not provide an `exec()` function, but rather a `run()`
/// function that takes the `Box<dyn JobRunnerContext>` and returns `Result<(String, Sender<Vec<Option<job::ExitCode>>>), String>`,
/// where the `Sender` is a channel that the module that alerts the run function that the script has ended, and the String value
/// contains the name of the event that starts the process.  This allows
/// the main module to monitor system signals and other events.  Because of the nature of the `run()` function, it does not
/// use a runtime parameter structure.
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

    pub dependencies: Vec<String>,
    pub os_dependencies: Vec<(String, String)>,

    /// The module's mod name, divided along paths.
    pub mod_name: Vec<String>,

    /// The module's instance struct name.
    pub instance_struct: String,

    /// The module's state structure definition.
    pub state_struct: Option<ModuleStructure>,

    /// The compile-time parameter structure accepted by the module's `new` call.
    /// Leave as None to indicate that the module does not accept a parameter.
    pub compile_param_struct: Option<ModuleStructure>,

    /// The runtime parameter value structure name accepted by the module's `exec` call.
    pub runtime_param_struct: Option<ModuleStructure>,

    /// The name of the structure that contains the stream instance information.
    pub stream_struct: Option<ModuleStreamStructure>,

    /// The list of available handlers and their parameters.
    pub handlers: Vec<(String, Vec<NamedValue>)>,
}
