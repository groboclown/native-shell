#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Perform some operation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action\","]
#[doc = "  \"description\": \"Perform some operation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"fd\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionFdSet\""]
#[doc = "    },"]
#[doc = "    \"on\": {"]
#[doc = "      \"$ref\": \"#/$defs/MapActionList\""]
#[doc = "    },"]
#[doc = "    \"on-error\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"on-success\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"run\": {"]
#[doc = "      \"title\": \"Run a module.\","]
#[doc = "      \"description\": \"Run an internal module.  For example, 'spawn', 'if', 'exec'.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"run-step\": {"]
#[doc = "      \"title\": \"Run Step\","]
#[doc = "      \"description\": \"Run the named step in the step list.  Does not take any other parameters.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStderr\""]
#[doc = "    },"]
#[doc = "    \"stdin\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdin\""]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdout\""]
#[doc = "    },"]
#[doc = "    \"with\": {"]
#[doc = "      \"title\": \"With Parameters\","]
#[doc = "      \"description\": \"Run the action with the given named parameters.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/ParameterValue\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fd: ::std::option::Option<ActionFdSet>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub on: ::std::option::Option<MapActionList>,
    #[serde(
        rename = "on-error",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_error: ::std::option::Option<ActionList>,
    #[serde(
        rename = "on-success",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_success: ::std::option::Option<ActionList>,
    #[doc = "Run an internal module.  For example, 'spawn', 'if', 'exec'."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub run: ::std::option::Option<::std::string::String>,
    #[doc = "Run the named step in the step list.  Does not take any other parameters."]
    #[serde(
        rename = "run-step",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub run_step: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<ActionStderr>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdin: ::std::option::Option<ActionStdin>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<ActionStdout>,
    #[doc = "Run the action with the given named parameters."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub with: ::std::collections::HashMap<ActionWithKey, ParameterValue>,
}
impl ::std::convert::From<&Action> for Action {
    fn from(value: &Action) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Action {
    fn default() -> Self {
        Self {
            description: Default::default(),
            fd: Default::default(),
            on: Default::default(),
            on_error: Default::default(),
            on_success: Default::default(),
            run: Default::default(),
            run_step: Default::default(),
            stderr: Default::default(),
            stdin: Default::default(),
            stdout: Default::default(),
            with: Default::default(),
        }
    }
}
impl Action {
    pub fn builder() -> builder::Action {
        Default::default()
    }
}
#[doc = "Describes what to do with the file descriptors for the shell.  By default, only 0-2 are allowed; additional ones must be declared."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"File Descriptor\","]
#[doc = "  \"description\": \"Describes what to do with the file descriptors for the shell.  By default, only 0-2 are allowed; additional ones must be declared.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"title\": \"File Descriptor\","]
#[doc = "    \"description\": \"A file descriptor definition.\","]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"required\": ["]
#[doc = "      \"id\","]
#[doc = "      \"type\""]
#[doc = "    ],"]
#[doc = "    \"properties\": {"]
#[doc = "      \"close-on-start\": {"]
#[doc = "        \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "        \"default\": false,"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      },"]
#[doc = "      \"description\": {"]
#[doc = "        \"$ref\": \"#/$defs/Description\""]
#[doc = "      },"]
#[doc = "      \"file\": {"]
#[doc = "        \"description\": \"If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"id\": {"]
#[doc = "        \"description\": \"The file descriptor ID established by the owning process that launched the shell.\","]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"minimum\": 0.0"]
#[doc = "      },"]
#[doc = "      \"log-level\": {"]
#[doc = "        \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.  Only applies to out streams.\","]
#[doc = "        \"default\": \"info\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"debug\","]
#[doc = "          \"info\","]
#[doc = "          \"warn\","]
#[doc = "          \"error\","]
#[doc = "          \"fatal\","]
#[doc = "          \"ignore\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"to-shell\": {"]
#[doc = "        \"description\": \"If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type.\","]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"type\": {"]
#[doc = "        \"description\": \"The type of the file descriptor.\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"in\","]
#[doc = "          \"out\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"additionalProperties\": false"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ActionFdSet(pub ::std::vec::Vec<FileDescriptor>);
impl ::std::ops::Deref for ActionFdSet {
    type Target = ::std::vec::Vec<FileDescriptor>;
    fn deref(&self) -> &::std::vec::Vec<FileDescriptor> {
        &self.0
    }
}
impl ::std::convert::From<ActionFdSet> for ::std::vec::Vec<FileDescriptor> {
    fn from(value: ActionFdSet) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActionFdSet> for ActionFdSet {
    fn from(value: &ActionFdSet) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<FileDescriptor>> for ActionFdSet {
    fn from(value: ::std::vec::Vec<FileDescriptor>) -> Self {
        Self(value)
    }
}
#[doc = "An ordered list of actions to take when an event happens."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"On Event List\","]
#[doc = "  \"description\": \"An ordered list of actions to take when an event happens.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/Action\""]
#[doc = "  },"]
#[doc = "  \"minItems\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ActionList(pub ::std::vec::Vec<Action>);
impl ::std::ops::Deref for ActionList {
    type Target = ::std::vec::Vec<Action>;
    fn deref(&self) -> &::std::vec::Vec<Action> {
        &self.0
    }
}
impl ::std::convert::From<ActionList> for ::std::vec::Vec<Action> {
    fn from(value: ActionList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActionList> for ActionList {
    fn from(value: &ActionList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Action>> for ActionList {
    fn from(value: ::std::vec::Vec<Action>) -> Self {
        Self(value)
    }
}
#[doc = "Standard error for the action."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Standard Error\","]
#[doc = "  \"description\": \"Standard error for the action.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"close-on-start\": {"]
#[doc = "      \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"log-level\": {"]
#[doc = "      \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.\","]
#[doc = "      \"default\": \"info\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"debug\","]
#[doc = "        \"info\","]
#[doc = "        \"warn\","]
#[doc = "        \"error\","]
#[doc = "        \"fatal\","]
#[doc = "        \"ignore\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"to-shell\": {"]
#[doc = "      \"description\": \"If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionStderr {
    #[doc = "Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(rename = "close-on-start", default)]
    pub close_on_start: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<::std::string::String>,
    #[doc = "When the script runs in different log levels, it can control whether this stream is processed or not."]
    #[serde(rename = "log-level", default = "defaults::action_stderr_log_level")]
    pub log_level: ActionStderrLogLevel,
    #[doc = "If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type."]
    #[serde(
        rename = "to-shell",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_shell: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ActionStderr> for ActionStderr {
    fn from(value: &ActionStderr) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ActionStderr {
    fn default() -> Self {
        Self {
            close_on_start: Default::default(),
            description: Default::default(),
            file: Default::default(),
            log_level: defaults::action_stderr_log_level(),
            to_shell: Default::default(),
        }
    }
}
impl ActionStderr {
    pub fn builder() -> builder::ActionStderr {
        Default::default()
    }
}
#[doc = "When the script runs in different log levels, it can control whether this stream is processed or not."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.\","]
#[doc = "  \"default\": \"info\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"debug\","]
#[doc = "    \"info\","]
#[doc = "    \"warn\","]
#[doc = "    \"error\","]
#[doc = "    \"fatal\","]
#[doc = "    \"ignore\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ActionStderrLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "fatal")]
    Fatal,
    #[serde(rename = "ignore")]
    Ignore,
}
impl ::std::convert::From<&Self> for ActionStderrLogLevel {
    fn from(value: &ActionStderrLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ActionStderrLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}
impl ::std::str::FromStr for ActionStderrLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            "ignore" => Ok(Self::Ignore),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActionStderrLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActionStderrLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActionStderrLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ActionStderrLogLevel {
    fn default() -> Self {
        ActionStderrLogLevel::Info
    }
}
#[doc = "Standard input behavior for the action."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Standard Input\","]
#[doc = "  \"description\": \"Standard input behavior for the action.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"close-on-start\": {"]
#[doc = "      \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"to-shell\": {"]
#[doc = "      \"description\": \"If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionStdin {
    #[doc = "Whether to close standard input when the shell starts."]
    #[serde(rename = "close-on-start", default)]
    pub close_on_start: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<::std::string::String>,
    #[doc = "If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type."]
    #[serde(
        rename = "to-shell",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_shell: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ActionStdin> for ActionStdin {
    fn from(value: &ActionStdin) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ActionStdin {
    fn default() -> Self {
        Self {
            close_on_start: Default::default(),
            description: Default::default(),
            file: Default::default(),
            to_shell: Default::default(),
        }
    }
}
impl ActionStdin {
    pub fn builder() -> builder::ActionStdin {
        Default::default()
    }
}
#[doc = "Standard output for the action."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Standard Output\","]
#[doc = "  \"description\": \"Standard output for the action.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"close-on-start\": {"]
#[doc = "      \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"log-level\": {"]
#[doc = "      \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.\","]
#[doc = "      \"default\": \"info\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"debug\","]
#[doc = "        \"info\","]
#[doc = "        \"warn\","]
#[doc = "        \"error\","]
#[doc = "        \"fatal\","]
#[doc = "        \"ignore\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"to-shell\": {"]
#[doc = "      \"description\": \"If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionStdout {
    #[doc = "Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(rename = "close-on-start", default)]
    pub close_on_start: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<::std::string::String>,
    #[doc = "When the script runs in different log levels, it can control whether this stream is processed or not."]
    #[serde(rename = "log-level", default = "defaults::action_stdout_log_level")]
    pub log_level: ActionStdoutLogLevel,
    #[doc = "If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type."]
    #[serde(
        rename = "to-shell",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_shell: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ActionStdout> for ActionStdout {
    fn from(value: &ActionStdout) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ActionStdout {
    fn default() -> Self {
        Self {
            close_on_start: Default::default(),
            description: Default::default(),
            file: Default::default(),
            log_level: defaults::action_stdout_log_level(),
            to_shell: Default::default(),
        }
    }
}
impl ActionStdout {
    pub fn builder() -> builder::ActionStdout {
        Default::default()
    }
}
#[doc = "When the script runs in different log levels, it can control whether this stream is processed or not."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.\","]
#[doc = "  \"default\": \"info\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"debug\","]
#[doc = "    \"info\","]
#[doc = "    \"warn\","]
#[doc = "    \"error\","]
#[doc = "    \"fatal\","]
#[doc = "    \"ignore\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ActionStdoutLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "fatal")]
    Fatal,
    #[serde(rename = "ignore")]
    Ignore,
}
impl ::std::convert::From<&Self> for ActionStdoutLogLevel {
    fn from(value: &ActionStdoutLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ActionStdoutLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}
impl ::std::str::FromStr for ActionStdoutLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            "ignore" => Ok(Self::Ignore),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActionStdoutLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActionStdoutLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActionStdoutLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ActionStdoutLogLevel {
    fn default() -> Self {
        ActionStdoutLogLevel::Info
    }
}
#[doc = "`ActionWithKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActionWithKey(::std::string::String);
impl ::std::ops::Deref for ActionWithKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActionWithKey> for ::std::string::String {
    fn from(value: ActionWithKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActionWithKey> for ActionWithKey {
    fn from(value: &ActionWithKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ActionWithKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActionWithKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActionWithKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActionWithKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActionWithKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The value of the argument.  Can be a string, number, boolean, or null.  The shell syntax of '${VALUE[@]}' and '${VALUE[*]}' can be used here for injecting array values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Argument Value\","]
#[doc = "  \"description\": \"The value of the argument.  Can be a string, number, boolean, or null.  The shell syntax of '${VALUE[@]}' and '${VALUE[*]}' can be used here for injecting array values.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"boolean\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ArgumentValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
}
impl ::std::convert::From<&Self> for ArgumentValue {
    fn from(value: &ArgumentValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for ArgumentValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for ArgumentValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "A command set definition.  Provides default values for a process or step."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Command Set\","]
#[doc = "  \"description\": \"A command set definition.  Provides default values for a process or step.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"$ref\": \"#/$defs/IdRef\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"title\": \"Environment Variables\","]
#[doc = "      \"description\": \"Environment variables to require for the shell process, or the default values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Environment Variable\","]
#[doc = "          \"description\": \"An environment variable definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"default\": {"]
#[doc = "              \"description\": \"The default value for the environment variable.  If not set, the variable must be provided by the parent process.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"required\": {"]
#[doc = "              \"description\": \"Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"unset\": {"]
#[doc = "              \"description\": \"If true, then unset the environment variable in the process.  This is useful for removing inherited environment variables.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"Overrides any parent value.  Cannot be used with 'default'.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fd\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionFdSet\""]
#[doc = "    },"]
#[doc = "    \"inherit-env\": {"]
#[doc = "      \"description\": \"Whether to inherit the environment variables from the parent process.  If false, then the environment variables must be explicitly set.\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"on\": {"]
#[doc = "      \"$ref\": \"#/$defs/MapActionList\""]
#[doc = "    },"]
#[doc = "    \"on-error\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"on-success\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStderr\""]
#[doc = "    },"]
#[doc = "    \"stdin\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdin\""]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdout\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommandSet {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<IdRef>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "Environment variables to require for the shell process, or the default values."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<CommandSetEnvKey, EnvironmentVariable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fd: ::std::option::Option<ActionFdSet>,
    #[doc = "Whether to inherit the environment variables from the parent process.  If false, then the environment variables must be explicitly set."]
    #[serde(rename = "inherit-env", default = "defaults::default_bool::<true>")]
    pub inherit_env: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub on: ::std::option::Option<MapActionList>,
    #[serde(
        rename = "on-error",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_error: ::std::option::Option<ActionList>,
    #[serde(
        rename = "on-success",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_success: ::std::option::Option<ActionList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<ActionStderr>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdin: ::std::option::Option<ActionStdin>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<ActionStdout>,
}
impl ::std::convert::From<&CommandSet> for CommandSet {
    fn from(value: &CommandSet) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CommandSet {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            description: Default::default(),
            env: Default::default(),
            fd: Default::default(),
            inherit_env: defaults::default_bool::<true>(),
            on: Default::default(),
            on_error: Default::default(),
            on_success: Default::default(),
            stderr: Default::default(),
            stdin: Default::default(),
            stdout: Default::default(),
        }
    }
}
impl CommandSet {
    pub fn builder() -> builder::CommandSet {
        Default::default()
    }
}
#[doc = "`CommandSetEnvKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CommandSetEnvKey(::std::string::String);
impl ::std::ops::Deref for CommandSetEnvKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CommandSetEnvKey> for ::std::string::String {
    fn from(value: CommandSetEnvKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CommandSetEnvKey> for CommandSetEnvKey {
    fn from(value: &CommandSetEnvKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CommandSetEnvKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CommandSetEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommandSetEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommandSetEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CommandSetEnvKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A constructed value definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constructed Value\","]
#[doc = "  \"description\": \"A constructed value definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"array-split\": {"]
#[doc = "      \"title\": \"Array Split Character\","]
#[doc = "      \"description\": \"If present, then split the environment variable value by this character to create an array.  Default are space characters.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"description\": \"A format string to apply to the value.  If not present, then the value is used as-is.  Only valid for boolean, integer and numeric types.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"is-array\": {"]
#[doc = "      \"description\": \"If true, then treat the value as an array.  If false, then the value is treated as a single string.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParameterValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstructedValue {
    #[doc = "If present, then split the environment variable value by this character to create an array.  Default are space characters."]
    #[serde(
        rename = "array-split",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub array_split: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "A format string to apply to the value.  If not present, then the value is used as-is.  Only valid for boolean, integer and numeric types."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::string::String>,
    #[doc = "If true, then treat the value as an array.  If false, then the value is treated as a single string."]
    #[serde(rename = "is-array", default)]
    pub is_array: bool,
    pub value: ParameterValue,
}
impl ::std::convert::From<&ConstructedValue> for ConstructedValue {
    fn from(value: &ConstructedValue) -> Self {
        value.clone()
    }
}
impl ConstructedValue {
    pub fn builder() -> builder::ConstructedValue {
        Default::default()
    }
}
#[doc = "A description of the item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Description\","]
#[doc = "  \"description\": \"A description of the item.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 2000,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Description(::std::string::String);
impl ::std::ops::Deref for Description {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Description> for ::std::string::String {
    fn from(value: Description) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Description> for Description {
    fn from(value: &Description) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Description {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Description {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Description {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Description {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Description {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "An environment variable definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Environment Variable\","]
#[doc = "  \"description\": \"An environment variable definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"The default value for the environment variable.  If not set, the variable must be provided by the parent process.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"unset\": {"]
#[doc = "      \"description\": \"If true, then unset the environment variable in the process.  This is useful for removing inherited environment variables.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"Overrides any parent value.  Cannot be used with 'default'.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentVariable {
    #[doc = "The default value for the environment variable.  If not set, the variable must be provided by the parent process."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied."]
    #[serde(default)]
    pub required: bool,
    #[doc = "If true, then unset the environment variable in the process.  This is useful for removing inherited environment variables."]
    #[serde(default)]
    pub unset: bool,
    #[doc = "Overrides any parent value.  Cannot be used with 'default'."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&EnvironmentVariable> for EnvironmentVariable {
    fn from(value: &EnvironmentVariable) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EnvironmentVariable {
    fn default() -> Self {
        Self {
            default: Default::default(),
            description: Default::default(),
            required: Default::default(),
            unset: Default::default(),
            value: Default::default(),
        }
    }
}
impl EnvironmentVariable {
    pub fn builder() -> builder::EnvironmentVariable {
        Default::default()
    }
}
#[doc = "A file descriptor definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"File Descriptor\","]
#[doc = "  \"description\": \"A file descriptor definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"close-on-start\": {"]
#[doc = "      \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The file descriptor ID established by the owning process that launched the shell.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"log-level\": {"]
#[doc = "      \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.  Only applies to out streams.\","]
#[doc = "      \"default\": \"info\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"debug\","]
#[doc = "        \"info\","]
#[doc = "        \"warn\","]
#[doc = "        \"error\","]
#[doc = "        \"fatal\","]
#[doc = "        \"ignore\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"to-shell\": {"]
#[doc = "      \"description\": \"If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the file descriptor.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"in\","]
#[doc = "        \"out\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FileDescriptor {
    #[doc = "Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(rename = "close-on-start", default)]
    pub close_on_start: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "If set, then the file descriptor is opened from this file.  This is useful for file descriptors that are not needed by the shell."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<::std::string::String>,
    #[doc = "The file descriptor ID established by the owning process that launched the shell."]
    pub id: u64,
    #[doc = "When the script runs in different log levels, it can control whether this stream is processed or not.  Only applies to out streams."]
    #[serde(rename = "log-level", default = "defaults::file_descriptor_log_level")]
    pub log_level: FileDescriptorLogLevel,
    #[doc = "If set, then connect this FD to the shell's corresponding file descriptor stream.  It must match the shell's corresponding type."]
    #[serde(
        rename = "to-shell",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_shell: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the file descriptor."]
    #[serde(rename = "type")]
    pub type_: FileDescriptorType,
}
impl ::std::convert::From<&FileDescriptor> for FileDescriptor {
    fn from(value: &FileDescriptor) -> Self {
        value.clone()
    }
}
impl FileDescriptor {
    pub fn builder() -> builder::FileDescriptor {
        Default::default()
    }
}
#[doc = "When the script runs in different log levels, it can control whether this stream is processed or not.  Only applies to out streams."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When the script runs in different log levels, it can control whether this stream is processed or not.  Only applies to out streams.\","]
#[doc = "  \"default\": \"info\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"debug\","]
#[doc = "    \"info\","]
#[doc = "    \"warn\","]
#[doc = "    \"error\","]
#[doc = "    \"fatal\","]
#[doc = "    \"ignore\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileDescriptorLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "fatal")]
    Fatal,
    #[serde(rename = "ignore")]
    Ignore,
}
impl ::std::convert::From<&Self> for FileDescriptorLogLevel {
    fn from(value: &FileDescriptorLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FileDescriptorLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}
impl ::std::str::FromStr for FileDescriptorLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            "ignore" => Ok(Self::Ignore),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileDescriptorLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FileDescriptorLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileDescriptorLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for FileDescriptorLogLevel {
    fn default() -> Self {
        FileDescriptorLogLevel::Info
    }
}
#[doc = "How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "  \"default\": \"line\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"line\","]
#[doc = "    \"byte\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileDescriptorMerge {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "byte")]
    Byte,
}
impl ::std::convert::From<&Self> for FileDescriptorMerge {
    fn from(value: &FileDescriptorMerge) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FileDescriptorMerge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Line => write!(f, "line"),
            Self::Byte => write!(f, "byte"),
        }
    }
}
impl ::std::str::FromStr for FileDescriptorMerge {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "line" => Ok(Self::Line),
            "byte" => Ok(Self::Byte),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileDescriptorMerge {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FileDescriptorMerge {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileDescriptorMerge {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for FileDescriptorMerge {
    fn default() -> Self {
        FileDescriptorMerge::Line
    }
}
#[doc = "The type of the file descriptor."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of the file descriptor.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"in\","]
#[doc = "    \"out\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileDescriptorType {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}
impl ::std::convert::From<&Self> for FileDescriptorType {
    fn from(value: &FileDescriptorType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FileDescriptorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::In => write!(f, "in"),
            Self::Out => write!(f, "out"),
        }
    }
}
impl ::std::str::FromStr for FileDescriptorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in" => Ok(Self::In),
            "out" => Ok(Self::Out),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileDescriptorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FileDescriptorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileDescriptorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An identifier."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ID Reference\","]
#[doc = "  \"description\": \"An identifier.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z0-9_.$ -]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IdRef(::std::string::String);
impl ::std::ops::Deref for IdRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IdRef> for ::std::string::String {
    fn from(value: IdRef) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IdRef> for IdRef {
    fn from(value: &IdRef) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for IdRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-zA-Z0-9_.$ -]+$").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_.$ -]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IdRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IdRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IdRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IdRef {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A map of event names to actions to take when the event happens."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Map of On Event Lists\","]
#[doc = "  \"description\": \"A map of event names to actions to take when the event happens.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \".*\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapActionList(pub ::std::collections::HashMap<MapActionListKey, ActionList>);
impl ::std::ops::Deref for MapActionList {
    type Target = ::std::collections::HashMap<MapActionListKey, ActionList>;
    fn deref(&self) -> &::std::collections::HashMap<MapActionListKey, ActionList> {
        &self.0
    }
}
impl ::std::convert::From<MapActionList>
    for ::std::collections::HashMap<MapActionListKey, ActionList>
{
    fn from(value: MapActionList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapActionList> for MapActionList {
    fn from(value: &MapActionList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<MapActionListKey, ActionList>>
    for MapActionList
{
    fn from(value: ::std::collections::HashMap<MapActionListKey, ActionList>) -> Self {
        Self(value)
    }
}
#[doc = "`MapActionListKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MapActionListKey(::std::string::String);
impl ::std::ops::Deref for MapActionListKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MapActionListKey> for ::std::string::String {
    fn from(value: MapActionListKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapActionListKey> for MapActionListKey {
    fn from(value: &MapActionListKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for MapActionListKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MapActionListKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MapActionListKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapActionListKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MapActionListKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A json-like format conversion from the script 2 layout.  Based on 'try 2' of the script language experiments."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Native Shell Script 2 Schema\","]
#[doc = "  \"description\": \"A json-like format conversion from the script 2 layout.  Based on 'try 2' of the script language experiments.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"processes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command-sets\": {"]
#[doc = "      \"title\": \"Command Sets\","]
#[doc = "      \"description\": \"Named command sets that define a runtime environment for processes.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/CommandSet\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"processes\": {"]
#[doc = "      \"title\": \"Processes\","]
#[doc = "      \"description\": \"A map of named executables that can be managed by the script.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/Process\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"shell\": {"]
#[doc = "      \"title\": \"Shell\","]
#[doc = "      \"description\": \"The shell's interaction with the parent environment.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"env\": {"]
#[doc = "          \"title\": \"Environment Variables\","]
#[doc = "          \"description\": \"Environment variables to require for the shell process, or the default values.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"title\": \"Environment Variable\","]
#[doc = "              \"description\": \"An environment variable definition.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"default\": {"]
#[doc = "                  \"description\": \"The default value for the environment variable.  If not set, the variable must be provided by the parent process.\","]
#[doc = "                  \"type\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"null\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"description\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Description\""]
#[doc = "                },"]
#[doc = "                \"required\": {"]
#[doc = "                  \"description\": \"Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied.\","]
#[doc = "                  \"default\": false,"]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"fd\": {"]
#[doc = "          \"title\": \"File Descriptor\","]
#[doc = "          \"description\": \"Describes what to do with the file descriptors for the shell.  By default, only 0-2 are allowed; additional ones must be declared.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"File Descriptor\","]
#[doc = "            \"description\": \"A file descriptor definition.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\","]
#[doc = "              \"type\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"close-on-start\": {"]
#[doc = "                \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "                \"default\": false,"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"description\": {"]
#[doc = "                \"$ref\": \"#/$defs/Description\""]
#[doc = "              },"]
#[doc = "              \"id\": {"]
#[doc = "                \"description\": \"The file descriptor ID established by the owning process that launched the shell.\","]
#[doc = "                \"type\": \"integer\","]
#[doc = "                \"minimum\": 0.0"]
#[doc = "              },"]
#[doc = "              \"merge\": {"]
#[doc = "                \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "                \"default\": \"line\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"line\","]
#[doc = "                  \"byte\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"type\": {"]
#[doc = "                \"description\": \"The type of the file descriptor.\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"in\","]
#[doc = "                  \"out\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"signals\": {"]
#[doc = "          \"title\": \"OS Signal Handler\","]
#[doc = "          \"description\": \"How to handle OS signals received by the shell process.  The shell can either send events to the event broker system, or send a signal to all running processes.  The object key is the signal name or number, and the value is the event name or, if prefixed with '+', the signal to send to all processes.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"title\": \"Signal Handler\","]
#[doc = "              \"description\": \"The name of the event to send to the event broker system, or a signal to send to all processes.  If prefixed with '+', it is a signal to send to all processes.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"stderr\": {"]
#[doc = "          \"title\": \"Standard Input\","]
#[doc = "          \"description\": \"Standard input for the shell.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"close-on-start\": {"]
#[doc = "              \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"merge\": {"]
#[doc = "              \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "              \"default\": \"byte\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"line\","]
#[doc = "                \"byte\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"stdin\": {"]
#[doc = "          \"title\": \"Standard Input\","]
#[doc = "          \"description\": \"Standard input for the shell.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"close-on-start\": {"]
#[doc = "              \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"stdout\": {"]
#[doc = "          \"title\": \"Standard Input\","]
#[doc = "          \"description\": \"Standard input for the shell.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"close-on-start\": {"]
#[doc = "              \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"merge\": {"]
#[doc = "              \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "              \"default\": \"line\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"line\","]
#[doc = "                \"byte\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"title\": \"Constructed Values\","]
#[doc = "          \"description\": \"Constructed values to use in the script.  Unlike environment variables, these aren't passed to child processes.  They can also have an 'array' structure, to split the value into an array, for use in injecting its value into array parameters.  Also, these are scope sensitive, meaning they are evaluated at time of use.  If associated to a process, then it reads from the environment variables passed to the process.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"title\": \"Constructed Value\","]
#[doc = "              \"description\": \"A constructed value definition.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"array-split\": {"]
#[doc = "                  \"title\": \"Array Split Character\","]
#[doc = "                  \"description\": \"If present, then split the environment variable value by this character to create an array.  Default are space characters.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"description\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Description\""]
#[doc = "                },"]
#[doc = "                \"format\": {"]
#[doc = "                  \"description\": \"A format string to apply to the value.  If not present, then the value is used as-is.  Only valid for boolean, integer and numeric types.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"is-array\": {"]
#[doc = "                  \"description\": \"If true, then treat the value as an array.  If false, then the value is treated as a single string.\","]
#[doc = "                  \"default\": false,"]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"$ref\": \"#/$defs/ParameterValue\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"steps\": {"]
#[doc = "      \"title\": \"Steps\","]
#[doc = "      \"description\": \"A map of named steps available to run in the script.  Each step can be a process, a module, or a control flow statement.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/Action\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$comment\": \"The patternProperties here uses '.*', when it should really use '^[a-zA-Z0-9_.$ -]+$'.  This comes from a known bug in quicktype (https://github.com/glideapps/quicktype/issues/1464).\","]
#[doc = "  \"$spdx-license\": \"MIT\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NativeShellScript2Schema {
    #[doc = "Named command sets that define a runtime environment for processes."]
    #[serde(
        rename = "command-sets",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub command_sets:
        ::std::collections::HashMap<NativeShellScript2SchemaCommandSetsKey, CommandSet>,
    #[doc = "A map of named executables that can be managed by the script."]
    pub processes: ::std::collections::HashMap<NativeShellScript2SchemaProcessesKey, Process>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shell: ::std::option::Option<Shell>,
    #[doc = "A map of named steps available to run in the script.  Each step can be a process, a module, or a control flow statement."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub steps: ::std::collections::HashMap<NativeShellScript2SchemaStepsKey, Action>,
}
impl ::std::convert::From<&NativeShellScript2Schema> for NativeShellScript2Schema {
    fn from(value: &NativeShellScript2Schema) -> Self {
        value.clone()
    }
}
impl NativeShellScript2Schema {
    pub fn builder() -> builder::NativeShellScript2Schema {
        Default::default()
    }
}
#[doc = "`NativeShellScript2SchemaCommandSetsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NativeShellScript2SchemaCommandSetsKey(::std::string::String);
impl ::std::ops::Deref for NativeShellScript2SchemaCommandSetsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellScript2SchemaCommandSetsKey> for ::std::string::String {
    fn from(value: NativeShellScript2SchemaCommandSetsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellScript2SchemaCommandSetsKey>
    for NativeShellScript2SchemaCommandSetsKey
{
    fn from(value: &NativeShellScript2SchemaCommandSetsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellScript2SchemaCommandSetsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellScript2SchemaCommandSetsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellScript2SchemaCommandSetsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellScript2SchemaCommandSetsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellScript2SchemaCommandSetsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`NativeShellScript2SchemaProcessesKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NativeShellScript2SchemaProcessesKey(::std::string::String);
impl ::std::ops::Deref for NativeShellScript2SchemaProcessesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellScript2SchemaProcessesKey> for ::std::string::String {
    fn from(value: NativeShellScript2SchemaProcessesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellScript2SchemaProcessesKey>
    for NativeShellScript2SchemaProcessesKey
{
    fn from(value: &NativeShellScript2SchemaProcessesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellScript2SchemaProcessesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellScript2SchemaProcessesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellScript2SchemaProcessesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellScript2SchemaProcessesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellScript2SchemaProcessesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`NativeShellScript2SchemaStepsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NativeShellScript2SchemaStepsKey(::std::string::String);
impl ::std::ops::Deref for NativeShellScript2SchemaStepsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellScript2SchemaStepsKey> for ::std::string::String {
    fn from(value: NativeShellScript2SchemaStepsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellScript2SchemaStepsKey> for NativeShellScript2SchemaStepsKey {
    fn from(value: &NativeShellScript2SchemaStepsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellScript2SchemaStepsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellScript2SchemaStepsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellScript2SchemaStepsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellScript2SchemaStepsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellScript2SchemaStepsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The value of a parameter.  Can be a string, number, or boolean.  The shell syntax of '${VALUE[@]}' and '${VALUE[*]}' can be used here for injecting array values.  Blown out to allow for easier code generation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Parameter Value\","]
#[doc = "  \"description\": \"The value of a parameter.  Can be a string, number, or boolean.  The shell syntax of '${VALUE[@]}' and '${VALUE[*]}' can be used here for injecting array values.  Blown out to allow for easier code generation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"array\": {"]
#[doc = "      \"title\": \"Array Value\","]
#[doc = "      \"description\": \"An array value.  For the purposes of this schema, only string items are allowed.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"boolean\": {"]
#[doc = "      \"description\": \"A boolean value.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"number\": {"]
#[doc = "      \"description\": \"A number value.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"string\": {"]
#[doc = "      \"description\": \"A string value.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParameterValue {
    #[doc = "An array value.  For the purposes of this schema, only string items are allowed."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub array: ::std::vec::Vec<::std::string::String>,
    #[doc = "A boolean value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boolean: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number: ::std::option::Option<f64>,
    #[doc = "A string value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub string: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ParameterValue> for ParameterValue {
    fn from(value: &ParameterValue) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ParameterValue {
    fn default() -> Self {
        Self {
            array: Default::default(),
            boolean: Default::default(),
            number: Default::default(),
            string: Default::default(),
        }
    }
}
impl ParameterValue {
    pub fn builder() -> builder::ParameterValue {
        Default::default()
    }
}
#[doc = "A process definition.  A process is a named executable that can be run in the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Process\","]
#[doc = "  \"description\": \"A process definition.  A process is a named executable that can be run in the script.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"title\": \"Arguments\","]
#[doc = "      \"description\": \"Arguments to pass to the command when it is run.\","]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Argument Value\","]
#[doc = "        \"description\": \"The value of the argument.  Can be a string, number, boolean, or null.  The shell syntax of '${VALUE[@]}' and '${VALUE[*]}' can be used here for injecting array values.\","]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"number\","]
#[doc = "          \"boolean\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"command\": {"]
#[doc = "      \"title\": \"Command\","]
#[doc = "      \"description\": \"The command to run for this process.  This can be a simple command (which means reading the location from the context's PATH env) or a full path to an executable.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"command-set\": {"]
#[doc = "      \"$ref\": \"#/$defs/IdRef\""]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Whether this process runs by default with the script.  If true, then this process will be run when the script is executed without any arguments.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"title\": \"Environment Variables\","]
#[doc = "      \"description\": \"Environment variables to require for the shell process, or the default values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Environment Variable\","]
#[doc = "          \"description\": \"An environment variable definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"default\": {"]
#[doc = "              \"description\": \"The default value for the environment variable.  If not set, the variable must be provided by the parent process.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"required\": {"]
#[doc = "              \"description\": \"Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"unset\": {"]
#[doc = "              \"description\": \"If true, then unset the environment variable in the process.  This is useful for removing inherited environment variables.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"Overrides any parent value.  Cannot be used with 'default'.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fd\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionFdSet\""]
#[doc = "    },"]
#[doc = "    \"inherit-env\": {"]
#[doc = "      \"description\": \"Whether to inherit the environment variables from the parent process.  If false, then the environment variables must be explicitly set.\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"on\": {"]
#[doc = "      \"$ref\": \"#/$defs/MapActionList\""]
#[doc = "    },"]
#[doc = "    \"on-error\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"on-success\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionList\""]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStderr\""]
#[doc = "    },"]
#[doc = "    \"stdin\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdin\""]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"$ref\": \"#/$defs/ActionStdout\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Process {
    #[doc = "Arguments to pass to the command when it is run."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub args: ::std::vec::Vec<ArgumentValue>,
    #[doc = "The command to run for this process.  This can be a simple command (which means reading the location from the context's PATH env) or a full path to an executable."]
    pub command: ::std::string::String,
    #[serde(
        rename = "command-set",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub command_set: ::std::option::Option<IdRef>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[doc = "Whether this process runs by default with the script.  If true, then this process will be run when the script is executed without any arguments."]
    #[serde(default)]
    pub default: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "Environment variables to require for the shell process, or the default values."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<ProcessEnvKey, EnvironmentVariable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fd: ::std::option::Option<ActionFdSet>,
    #[doc = "Whether to inherit the environment variables from the parent process.  If false, then the environment variables must be explicitly set."]
    #[serde(rename = "inherit-env", default = "defaults::default_bool::<true>")]
    pub inherit_env: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub on: ::std::option::Option<MapActionList>,
    #[serde(
        rename = "on-error",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_error: ::std::option::Option<ActionList>,
    #[serde(
        rename = "on-success",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_success: ::std::option::Option<ActionList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<ActionStderr>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdin: ::std::option::Option<ActionStdin>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<ActionStdout>,
}
impl ::std::convert::From<&Process> for Process {
    fn from(value: &Process) -> Self {
        value.clone()
    }
}
impl Process {
    pub fn builder() -> builder::Process {
        Default::default()
    }
}
#[doc = "`ProcessEnvKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProcessEnvKey(::std::string::String);
impl ::std::ops::Deref for ProcessEnvKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProcessEnvKey> for ::std::string::String {
    fn from(value: ProcessEnvKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessEnvKey> for ProcessEnvKey {
    fn from(value: &ProcessEnvKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ProcessEnvKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProcessEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProcessEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProcessEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProcessEnvKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The shell's interaction with the parent environment."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Shell\","]
#[doc = "  \"description\": \"The shell's interaction with the parent environment.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"env\": {"]
#[doc = "      \"title\": \"Environment Variables\","]
#[doc = "      \"description\": \"Environment variables to require for the shell process, or the default values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Environment Variable\","]
#[doc = "          \"description\": \"An environment variable definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"default\": {"]
#[doc = "              \"description\": \"The default value for the environment variable.  If not set, the variable must be provided by the parent process.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"required\": {"]
#[doc = "              \"description\": \"Whether this environment variable is required.  If true, the shell will fail to start if this variable is not set.  If false, then the 'default' value must be supplied.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fd\": {"]
#[doc = "      \"title\": \"File Descriptor\","]
#[doc = "      \"description\": \"Describes what to do with the file descriptors for the shell.  By default, only 0-2 are allowed; additional ones must be declared.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"File Descriptor\","]
#[doc = "        \"description\": \"A file descriptor definition.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"close-on-start\": {"]
#[doc = "            \"description\": \"Whether to close this file descriptor when the shell starts.  This is useful for file descriptors that are not needed by the shell.\","]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"description\": {"]
#[doc = "            \"$ref\": \"#/$defs/Description\""]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"description\": \"The file descriptor ID established by the owning process that launched the shell.\","]
#[doc = "            \"type\": \"integer\","]
#[doc = "            \"minimum\": 0.0"]
#[doc = "          },"]
#[doc = "          \"merge\": {"]
#[doc = "            \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "            \"default\": \"line\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"line\","]
#[doc = "              \"byte\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"description\": \"The type of the file descriptor.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"in\","]
#[doc = "              \"out\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"signals\": {"]
#[doc = "      \"title\": \"OS Signal Handler\","]
#[doc = "      \"description\": \"How to handle OS signals received by the shell process.  The shell can either send events to the event broker system, or send a signal to all running processes.  The object key is the signal name or number, and the value is the event name or, if prefixed with '+', the signal to send to all processes.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Signal Handler\","]
#[doc = "          \"description\": \"The name of the event to send to the event broker system, or a signal to send to all processes.  If prefixed with '+', it is a signal to send to all processes.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"title\": \"Standard Input\","]
#[doc = "      \"description\": \"Standard input for the shell.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"close-on-start\": {"]
#[doc = "          \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"$ref\": \"#/$defs/Description\""]
#[doc = "        },"]
#[doc = "        \"merge\": {"]
#[doc = "          \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "          \"default\": \"byte\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"line\","]
#[doc = "            \"byte\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"stdin\": {"]
#[doc = "      \"title\": \"Standard Input\","]
#[doc = "      \"description\": \"Standard input for the shell.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"close-on-start\": {"]
#[doc = "          \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"$ref\": \"#/$defs/Description\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"title\": \"Standard Input\","]
#[doc = "      \"description\": \"Standard input for the shell.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"close-on-start\": {"]
#[doc = "          \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"$ref\": \"#/$defs/Description\""]
#[doc = "        },"]
#[doc = "        \"merge\": {"]
#[doc = "          \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "          \"default\": \"line\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"line\","]
#[doc = "            \"byte\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"title\": \"Constructed Values\","]
#[doc = "      \"description\": \"Constructed values to use in the script.  Unlike environment variables, these aren't passed to child processes.  They can also have an 'array' structure, to split the value into an array, for use in injecting its value into array parameters.  Also, these are scope sensitive, meaning they are evaluated at time of use.  If associated to a process, then it reads from the environment variables passed to the process.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Constructed Value\","]
#[doc = "          \"description\": \"A constructed value definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"array-split\": {"]
#[doc = "              \"title\": \"Array Split Character\","]
#[doc = "              \"description\": \"If present, then split the environment variable value by this character to create an array.  Default are space characters.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$ref\": \"#/$defs/Description\""]
#[doc = "            },"]
#[doc = "            \"format\": {"]
#[doc = "              \"description\": \"A format string to apply to the value.  If not present, then the value is used as-is.  Only valid for boolean, integer and numeric types.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"is-array\": {"]
#[doc = "              \"description\": \"If true, then treat the value as an array.  If false, then the value is treated as a single string.\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"$ref\": \"#/$defs/ParameterValue\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Shell {
    #[doc = "Environment variables to require for the shell process, or the default values."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<ShellEnvKey, EnvironmentVariable>,
    #[doc = "Describes what to do with the file descriptors for the shell.  By default, only 0-2 are allowed; additional ones must be declared."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fd: ::std::vec::Vec<FileDescriptor>,
    #[doc = "How to handle OS signals received by the shell process.  The shell can either send events to the event broker system, or send a signal to all running processes.  The object key is the signal name or number, and the value is the event name or, if prefixed with '+', the signal to send to all processes."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub signals: ::std::collections::HashMap<ShellSignalsKey, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<StandardInput>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdin: ::std::option::Option<StandardInput>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<StandardInput>,
    #[doc = "Constructed values to use in the script.  Unlike environment variables, these aren't passed to child processes.  They can also have an 'array' structure, to split the value into an array, for use in injecting its value into array parameters.  Also, these are scope sensitive, meaning they are evaluated at time of use.  If associated to a process, then it reads from the environment variables passed to the process."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub values: ::std::collections::HashMap<ShellValuesKey, ConstructedValue>,
}
impl ::std::convert::From<&Shell> for Shell {
    fn from(value: &Shell) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Shell {
    fn default() -> Self {
        Self {
            env: Default::default(),
            fd: Default::default(),
            signals: Default::default(),
            stderr: Default::default(),
            stdin: Default::default(),
            stdout: Default::default(),
            values: Default::default(),
        }
    }
}
impl Shell {
    pub fn builder() -> builder::Shell {
        Default::default()
    }
}
#[doc = "`ShellEnvKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ShellEnvKey(::std::string::String);
impl ::std::ops::Deref for ShellEnvKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ShellEnvKey> for ::std::string::String {
    fn from(value: ShellEnvKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShellEnvKey> for ShellEnvKey {
    fn from(value: &ShellEnvKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ShellEnvKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ShellEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ShellEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ShellEnvKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ShellEnvKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ShellSignalsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ShellSignalsKey(::std::string::String);
impl ::std::ops::Deref for ShellSignalsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ShellSignalsKey> for ::std::string::String {
    fn from(value: ShellSignalsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShellSignalsKey> for ShellSignalsKey {
    fn from(value: &ShellSignalsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ShellSignalsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ShellSignalsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ShellSignalsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ShellSignalsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ShellSignalsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ShellValuesKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ShellValuesKey(::std::string::String);
impl ::std::ops::Deref for ShellValuesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ShellValuesKey> for ::std::string::String {
    fn from(value: ShellValuesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShellValuesKey> for ShellValuesKey {
    fn from(value: &ShellValuesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ShellValuesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ShellValuesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ShellValuesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ShellValuesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ShellValuesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Standard input for the shell."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Standard Input\","]
#[doc = "  \"description\": \"Standard input for the shell.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"close-on-start\": {"]
#[doc = "      \"description\": \"Whether to close standard input when the shell starts.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/Description\""]
#[doc = "    },"]
#[doc = "    \"merge\": {"]
#[doc = "      \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "      \"default\": \"byte\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"line\","]
#[doc = "        \"byte\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StandardInput {
    #[doc = "Whether to close standard input when the shell starts."]
    #[serde(rename = "close-on-start", default)]
    pub close_on_start: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Description>,
    #[doc = "How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors."]
    #[serde(default = "defaults::standard_input_merge")]
    pub merge: StandardInputMerge,
}
impl ::std::convert::From<&StandardInput> for StandardInput {
    fn from(value: &StandardInput) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for StandardInput {
    fn default() -> Self {
        Self {
            close_on_start: Default::default(),
            description: Default::default(),
            merge: defaults::standard_input_merge(),
        }
    }
}
impl StandardInput {
    pub fn builder() -> builder::StandardInput {
        Default::default()
    }
}
#[doc = "How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"How to merge the output of this file descriptor.  'line' means interleaving complete lines.  'byte' means writing directly to the stream without any buffering.  Only applicable to 'out' type file descriptors.\","]
#[doc = "  \"default\": \"byte\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"line\","]
#[doc = "    \"byte\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StandardInputMerge {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "byte")]
    Byte,
}
impl ::std::convert::From<&Self> for StandardInputMerge {
    fn from(value: &StandardInputMerge) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StandardInputMerge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Line => write!(f, "line"),
            Self::Byte => write!(f, "byte"),
        }
    }
}
impl ::std::str::FromStr for StandardInputMerge {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "line" => Ok(Self::Line),
            "byte" => Ok(Self::Byte),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StandardInputMerge {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StandardInputMerge {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StandardInputMerge {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StandardInputMerge {
    fn default() -> Self {
        StandardInputMerge::Byte
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Action {
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        fd: ::std::result::Result<::std::option::Option<super::ActionFdSet>, ::std::string::String>,
        on: ::std::result::Result<
            ::std::option::Option<super::MapActionList>,
            ::std::string::String,
        >,
        on_error:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        on_success:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        run: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        run_step: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stderr: ::std::result::Result<
            ::std::option::Option<super::ActionStderr>,
            ::std::string::String,
        >,
        stdin:
            ::std::result::Result<::std::option::Option<super::ActionStdin>, ::std::string::String>,
        stdout: ::std::result::Result<
            ::std::option::Option<super::ActionStdout>,
            ::std::string::String,
        >,
        with: ::std::result::Result<
            ::std::collections::HashMap<super::ActionWithKey, super::ParameterValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Action {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                fd: Ok(Default::default()),
                on: Ok(Default::default()),
                on_error: Ok(Default::default()),
                on_success: Ok(Default::default()),
                run: Ok(Default::default()),
                run_step: Ok(Default::default()),
                stderr: Ok(Default::default()),
                stdin: Ok(Default::default()),
                stdout: Ok(Default::default()),
                with: Ok(Default::default()),
            }
        }
    }
    impl Action {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionFdSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fd: {}", e));
            self
        }
        pub fn on<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MapActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on: {}", e));
            self
        }
        pub fn on_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_error: {}", e));
            self
        }
        pub fn on_success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_success = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_success: {}", e));
            self
        }
        pub fn run<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.run = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run: {}", e));
            self
        }
        pub fn run_step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.run_step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_step: {}", e));
            self
        }
        pub fn stderr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStderr>>,
            T::Error: ::std::fmt::Display,
        {
            self.stderr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stderr: {}", e));
            self
        }
        pub fn stdin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdin>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdin: {}", e));
            self
        }
        pub fn stdout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdout>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdout: {}", e));
            self
        }
        pub fn with<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ActionWithKey, super::ParameterValue>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.with = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for with: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Action> for super::Action {
        type Error = super::error::ConversionError;
        fn try_from(value: Action) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                fd: value.fd?,
                on: value.on?,
                on_error: value.on_error?,
                on_success: value.on_success?,
                run: value.run?,
                run_step: value.run_step?,
                stderr: value.stderr?,
                stdin: value.stdin?,
                stdout: value.stdout?,
                with: value.with?,
            })
        }
    }
    impl ::std::convert::From<super::Action> for Action {
        fn from(value: super::Action) -> Self {
            Self {
                description: Ok(value.description),
                fd: Ok(value.fd),
                on: Ok(value.on),
                on_error: Ok(value.on_error),
                on_success: Ok(value.on_success),
                run: Ok(value.run),
                run_step: Ok(value.run_step),
                stderr: Ok(value.stderr),
                stdin: Ok(value.stdin),
                stdout: Ok(value.stdout),
                with: Ok(value.with),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionStderr {
        close_on_start: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        log_level: ::std::result::Result<super::ActionStderrLogLevel, ::std::string::String>,
        to_shell: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionStderr {
        fn default() -> Self {
            Self {
                close_on_start: Ok(Default::default()),
                description: Ok(Default::default()),
                file: Ok(Default::default()),
                log_level: Ok(super::defaults::action_stderr_log_level()),
                to_shell: Ok(Default::default()),
            }
        }
    }
    impl ActionStderr {
        pub fn close_on_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.close_on_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close_on_start: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionStderrLogLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
        pub fn to_shell<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.to_shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_shell: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionStderr> for super::ActionStderr {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionStderr,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close_on_start: value.close_on_start?,
                description: value.description?,
                file: value.file?,
                log_level: value.log_level?,
                to_shell: value.to_shell?,
            })
        }
    }
    impl ::std::convert::From<super::ActionStderr> for ActionStderr {
        fn from(value: super::ActionStderr) -> Self {
            Self {
                close_on_start: Ok(value.close_on_start),
                description: Ok(value.description),
                file: Ok(value.file),
                log_level: Ok(value.log_level),
                to_shell: Ok(value.to_shell),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionStdin {
        close_on_start: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        to_shell: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionStdin {
        fn default() -> Self {
            Self {
                close_on_start: Ok(Default::default()),
                description: Ok(Default::default()),
                file: Ok(Default::default()),
                to_shell: Ok(Default::default()),
            }
        }
    }
    impl ActionStdin {
        pub fn close_on_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.close_on_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close_on_start: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn to_shell<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.to_shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_shell: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionStdin> for super::ActionStdin {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionStdin,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close_on_start: value.close_on_start?,
                description: value.description?,
                file: value.file?,
                to_shell: value.to_shell?,
            })
        }
    }
    impl ::std::convert::From<super::ActionStdin> for ActionStdin {
        fn from(value: super::ActionStdin) -> Self {
            Self {
                close_on_start: Ok(value.close_on_start),
                description: Ok(value.description),
                file: Ok(value.file),
                to_shell: Ok(value.to_shell),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionStdout {
        close_on_start: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        log_level: ::std::result::Result<super::ActionStdoutLogLevel, ::std::string::String>,
        to_shell: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionStdout {
        fn default() -> Self {
            Self {
                close_on_start: Ok(Default::default()),
                description: Ok(Default::default()),
                file: Ok(Default::default()),
                log_level: Ok(super::defaults::action_stdout_log_level()),
                to_shell: Ok(Default::default()),
            }
        }
    }
    impl ActionStdout {
        pub fn close_on_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.close_on_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close_on_start: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionStdoutLogLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
        pub fn to_shell<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.to_shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_shell: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionStdout> for super::ActionStdout {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionStdout,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close_on_start: value.close_on_start?,
                description: value.description?,
                file: value.file?,
                log_level: value.log_level?,
                to_shell: value.to_shell?,
            })
        }
    }
    impl ::std::convert::From<super::ActionStdout> for ActionStdout {
        fn from(value: super::ActionStdout) -> Self {
            Self {
                close_on_start: Ok(value.close_on_start),
                description: Ok(value.description),
                file: Ok(value.file),
                log_level: Ok(value.log_level),
                to_shell: Ok(value.to_shell),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommandSet {
        cwd: ::std::result::Result<::std::option::Option<super::IdRef>, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        env: ::std::result::Result<
            ::std::collections::HashMap<super::CommandSetEnvKey, super::EnvironmentVariable>,
            ::std::string::String,
        >,
        fd: ::std::result::Result<::std::option::Option<super::ActionFdSet>, ::std::string::String>,
        inherit_env: ::std::result::Result<bool, ::std::string::String>,
        on: ::std::result::Result<
            ::std::option::Option<super::MapActionList>,
            ::std::string::String,
        >,
        on_error:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        on_success:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        stderr: ::std::result::Result<
            ::std::option::Option<super::ActionStderr>,
            ::std::string::String,
        >,
        stdin:
            ::std::result::Result<::std::option::Option<super::ActionStdin>, ::std::string::String>,
        stdout: ::std::result::Result<
            ::std::option::Option<super::ActionStdout>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommandSet {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                description: Ok(Default::default()),
                env: Ok(Default::default()),
                fd: Ok(Default::default()),
                inherit_env: Ok(super::defaults::default_bool::<true>()),
                on: Ok(Default::default()),
                on_error: Ok(Default::default()),
                on_success: Ok(Default::default()),
                stderr: Ok(Default::default()),
                stdin: Ok(Default::default()),
                stdout: Ok(Default::default()),
            }
        }
    }
    impl CommandSet {
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IdRef>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::CommandSetEnvKey, super::EnvironmentVariable>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {}", e));
            self
        }
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionFdSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fd: {}", e));
            self
        }
        pub fn inherit_env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.inherit_env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inherit_env: {}", e));
            self
        }
        pub fn on<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MapActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on: {}", e));
            self
        }
        pub fn on_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_error: {}", e));
            self
        }
        pub fn on_success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_success = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_success: {}", e));
            self
        }
        pub fn stderr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStderr>>,
            T::Error: ::std::fmt::Display,
        {
            self.stderr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stderr: {}", e));
            self
        }
        pub fn stdin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdin>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdin: {}", e));
            self
        }
        pub fn stdout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdout>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdout: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CommandSet> for super::CommandSet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommandSet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                description: value.description?,
                env: value.env?,
                fd: value.fd?,
                inherit_env: value.inherit_env?,
                on: value.on?,
                on_error: value.on_error?,
                on_success: value.on_success?,
                stderr: value.stderr?,
                stdin: value.stdin?,
                stdout: value.stdout?,
            })
        }
    }
    impl ::std::convert::From<super::CommandSet> for CommandSet {
        fn from(value: super::CommandSet) -> Self {
            Self {
                cwd: Ok(value.cwd),
                description: Ok(value.description),
                env: Ok(value.env),
                fd: Ok(value.fd),
                inherit_env: Ok(value.inherit_env),
                on: Ok(value.on),
                on_error: Ok(value.on_error),
                on_success: Ok(value.on_success),
                stderr: Ok(value.stderr),
                stdin: Ok(value.stdin),
                stdout: Ok(value.stdout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstructedValue {
        array_split: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        format: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_array: ::std::result::Result<bool, ::std::string::String>,
        value: ::std::result::Result<super::ParameterValue, ::std::string::String>,
    }
    impl ::std::default::Default for ConstructedValue {
        fn default() -> Self {
            Self {
                array_split: Ok(Default::default()),
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                is_array: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstructedValue {
        pub fn array_split<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.array_split = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for array_split: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn is_array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_array = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_array: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParameterValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstructedValue> for super::ConstructedValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstructedValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                array_split: value.array_split?,
                description: value.description?,
                format: value.format?,
                is_array: value.is_array?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstructedValue> for ConstructedValue {
        fn from(value: super::ConstructedValue) -> Self {
            Self {
                array_split: Ok(value.array_split),
                description: Ok(value.description),
                format: Ok(value.format),
                is_array: Ok(value.is_array),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnvironmentVariable {
        default: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        required: ::std::result::Result<bool, ::std::string::String>,
        unset: ::std::result::Result<bool, ::std::string::String>,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EnvironmentVariable {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                required: Ok(Default::default()),
                unset: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl EnvironmentVariable {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn unset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.unset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unset: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EnvironmentVariable> for super::EnvironmentVariable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnvironmentVariable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                required: value.required?,
                unset: value.unset?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::EnvironmentVariable> for EnvironmentVariable {
        fn from(value: super::EnvironmentVariable) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                required: Ok(value.required),
                unset: Ok(value.unset),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileDescriptor {
        close_on_start: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<u64, ::std::string::String>,
        log_level: ::std::result::Result<super::FileDescriptorLogLevel, ::std::string::String>,
        to_shell: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FileDescriptorType, ::std::string::String>,
    }
    impl ::std::default::Default for FileDescriptor {
        fn default() -> Self {
            Self {
                close_on_start: Ok(Default::default()),
                description: Ok(Default::default()),
                file: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                log_level: Ok(super::defaults::file_descriptor_log_level()),
                to_shell: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FileDescriptor {
        pub fn close_on_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.close_on_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close_on_start: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileDescriptorLogLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
        pub fn to_shell<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.to_shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_shell: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileDescriptorType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FileDescriptor> for super::FileDescriptor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileDescriptor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close_on_start: value.close_on_start?,
                description: value.description?,
                file: value.file?,
                id: value.id?,
                log_level: value.log_level?,
                to_shell: value.to_shell?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FileDescriptor> for FileDescriptor {
        fn from(value: super::FileDescriptor) -> Self {
            Self {
                close_on_start: Ok(value.close_on_start),
                description: Ok(value.description),
                file: Ok(value.file),
                id: Ok(value.id),
                log_level: Ok(value.log_level),
                to_shell: Ok(value.to_shell),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NativeShellScript2Schema {
        command_sets: ::std::result::Result<
            ::std::collections::HashMap<
                super::NativeShellScript2SchemaCommandSetsKey,
                super::CommandSet,
            >,
            ::std::string::String,
        >,
        processes: ::std::result::Result<
            ::std::collections::HashMap<
                super::NativeShellScript2SchemaProcessesKey,
                super::Process,
            >,
            ::std::string::String,
        >,
        shell: ::std::result::Result<::std::option::Option<super::Shell>, ::std::string::String>,
        steps: ::std::result::Result<
            ::std::collections::HashMap<super::NativeShellScript2SchemaStepsKey, super::Action>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NativeShellScript2Schema {
        fn default() -> Self {
            Self {
                command_sets: Ok(Default::default()),
                processes: Err("no value supplied for processes".to_string()),
                shell: Ok(Default::default()),
                steps: Ok(Default::default()),
            }
        }
    }
    impl NativeShellScript2Schema {
        pub fn command_sets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::NativeShellScript2SchemaCommandSetsKey,
                    super::CommandSet,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.command_sets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command_sets: {}", e));
            self
        }
        pub fn processes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::NativeShellScript2SchemaProcessesKey,
                    super::Process,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.processes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for processes: {}", e));
            self
        }
        pub fn shell<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Shell>>,
            T::Error: ::std::fmt::Display,
        {
            self.shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shell: {}", e));
            self
        }
        pub fn steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::NativeShellScript2SchemaStepsKey, super::Action>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for steps: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NativeShellScript2Schema> for super::NativeShellScript2Schema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NativeShellScript2Schema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_sets: value.command_sets?,
                processes: value.processes?,
                shell: value.shell?,
                steps: value.steps?,
            })
        }
    }
    impl ::std::convert::From<super::NativeShellScript2Schema> for NativeShellScript2Schema {
        fn from(value: super::NativeShellScript2Schema) -> Self {
            Self {
                command_sets: Ok(value.command_sets),
                processes: Ok(value.processes),
                shell: Ok(value.shell),
                steps: Ok(value.steps),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ParameterValue {
        array: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        boolean: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        number: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        string: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ParameterValue {
        fn default() -> Self {
            Self {
                array: Ok(Default::default()),
                boolean: Ok(Default::default()),
                number: Ok(Default::default()),
                string: Ok(Default::default()),
            }
        }
    }
    impl ParameterValue {
        pub fn array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.array = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for array: {}", e));
            self
        }
        pub fn boolean<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.boolean = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boolean: {}", e));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {}", e));
            self
        }
        pub fn string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for string: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ParameterValue> for super::ParameterValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ParameterValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                array: value.array?,
                boolean: value.boolean?,
                number: value.number?,
                string: value.string?,
            })
        }
    }
    impl ::std::convert::From<super::ParameterValue> for ParameterValue {
        fn from(value: super::ParameterValue) -> Self {
            Self {
                array: Ok(value.array),
                boolean: Ok(value.boolean),
                number: Ok(value.number),
                string: Ok(value.string),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Process {
        args: ::std::result::Result<::std::vec::Vec<super::ArgumentValue>, ::std::string::String>,
        command: ::std::result::Result<::std::string::String, ::std::string::String>,
        command_set:
            ::std::result::Result<::std::option::Option<super::IdRef>, ::std::string::String>,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        default: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        env: ::std::result::Result<
            ::std::collections::HashMap<super::ProcessEnvKey, super::EnvironmentVariable>,
            ::std::string::String,
        >,
        fd: ::std::result::Result<::std::option::Option<super::ActionFdSet>, ::std::string::String>,
        inherit_env: ::std::result::Result<bool, ::std::string::String>,
        on: ::std::result::Result<
            ::std::option::Option<super::MapActionList>,
            ::std::string::String,
        >,
        on_error:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        on_success:
            ::std::result::Result<::std::option::Option<super::ActionList>, ::std::string::String>,
        stderr: ::std::result::Result<
            ::std::option::Option<super::ActionStderr>,
            ::std::string::String,
        >,
        stdin:
            ::std::result::Result<::std::option::Option<super::ActionStdin>, ::std::string::String>,
        stdout: ::std::result::Result<
            ::std::option::Option<super::ActionStdout>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Process {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                command: Err("no value supplied for command".to_string()),
                command_set: Ok(Default::default()),
                cwd: Ok(Default::default()),
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                env: Ok(Default::default()),
                fd: Ok(Default::default()),
                inherit_env: Ok(super::defaults::default_bool::<true>()),
                on: Ok(Default::default()),
                on_error: Ok(Default::default()),
                on_success: Ok(Default::default()),
                stderr: Ok(Default::default()),
                stdin: Ok(Default::default()),
                stdout: Ok(Default::default()),
            }
        }
    }
    impl Process {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ArgumentValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {}", e));
            self
        }
        pub fn command_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IdRef>>,
            T::Error: ::std::fmt::Display,
        {
            self.command_set = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command_set: {}", e));
            self
        }
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {}", e));
            self
        }
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ProcessEnvKey, super::EnvironmentVariable>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {}", e));
            self
        }
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionFdSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fd: {}", e));
            self
        }
        pub fn inherit_env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.inherit_env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inherit_env: {}", e));
            self
        }
        pub fn on<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MapActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on: {}", e));
            self
        }
        pub fn on_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_error: {}", e));
            self
        }
        pub fn on_success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionList>>,
            T::Error: ::std::fmt::Display,
        {
            self.on_success = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_success: {}", e));
            self
        }
        pub fn stderr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStderr>>,
            T::Error: ::std::fmt::Display,
        {
            self.stderr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stderr: {}", e));
            self
        }
        pub fn stdin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdin>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdin: {}", e));
            self
        }
        pub fn stdout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActionStdout>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdout: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Process> for super::Process {
        type Error = super::error::ConversionError;
        fn try_from(value: Process) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                command: value.command?,
                command_set: value.command_set?,
                cwd: value.cwd?,
                default: value.default?,
                description: value.description?,
                env: value.env?,
                fd: value.fd?,
                inherit_env: value.inherit_env?,
                on: value.on?,
                on_error: value.on_error?,
                on_success: value.on_success?,
                stderr: value.stderr?,
                stdin: value.stdin?,
                stdout: value.stdout?,
            })
        }
    }
    impl ::std::convert::From<super::Process> for Process {
        fn from(value: super::Process) -> Self {
            Self {
                args: Ok(value.args),
                command: Ok(value.command),
                command_set: Ok(value.command_set),
                cwd: Ok(value.cwd),
                default: Ok(value.default),
                description: Ok(value.description),
                env: Ok(value.env),
                fd: Ok(value.fd),
                inherit_env: Ok(value.inherit_env),
                on: Ok(value.on),
                on_error: Ok(value.on_error),
                on_success: Ok(value.on_success),
                stderr: Ok(value.stderr),
                stdin: Ok(value.stdin),
                stdout: Ok(value.stdout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Shell {
        env: ::std::result::Result<
            ::std::collections::HashMap<super::ShellEnvKey, super::EnvironmentVariable>,
            ::std::string::String,
        >,
        fd: ::std::result::Result<::std::vec::Vec<super::FileDescriptor>, ::std::string::String>,
        signals: ::std::result::Result<
            ::std::collections::HashMap<super::ShellSignalsKey, ::std::string::String>,
            ::std::string::String,
        >,
        stderr: ::std::result::Result<
            ::std::option::Option<super::StandardInput>,
            ::std::string::String,
        >,
        stdin: ::std::result::Result<
            ::std::option::Option<super::StandardInput>,
            ::std::string::String,
        >,
        stdout: ::std::result::Result<
            ::std::option::Option<super::StandardInput>,
            ::std::string::String,
        >,
        values: ::std::result::Result<
            ::std::collections::HashMap<super::ShellValuesKey, super::ConstructedValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Shell {
        fn default() -> Self {
            Self {
                env: Ok(Default::default()),
                fd: Ok(Default::default()),
                signals: Ok(Default::default()),
                stderr: Ok(Default::default()),
                stdin: Ok(Default::default()),
                stdout: Ok(Default::default()),
                values: Ok(Default::default()),
            }
        }
    }
    impl Shell {
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ShellEnvKey, super::EnvironmentVariable>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {}", e));
            self
        }
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FileDescriptor>>,
            T::Error: ::std::fmt::Display,
        {
            self.fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fd: {}", e));
            self
        }
        pub fn signals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ShellSignalsKey, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.signals = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signals: {}", e));
            self
        }
        pub fn stderr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StandardInput>>,
            T::Error: ::std::fmt::Display,
        {
            self.stderr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stderr: {}", e));
            self
        }
        pub fn stdin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StandardInput>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdin: {}", e));
            self
        }
        pub fn stdout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StandardInput>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdout: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ShellValuesKey, super::ConstructedValue>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Shell> for super::Shell {
        type Error = super::error::ConversionError;
        fn try_from(value: Shell) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                env: value.env?,
                fd: value.fd?,
                signals: value.signals?,
                stderr: value.stderr?,
                stdin: value.stdin?,
                stdout: value.stdout?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::Shell> for Shell {
        fn from(value: super::Shell) -> Self {
            Self {
                env: Ok(value.env),
                fd: Ok(value.fd),
                signals: Ok(value.signals),
                stderr: Ok(value.stderr),
                stdin: Ok(value.stdin),
                stdout: Ok(value.stdout),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StandardInput {
        close_on_start: ::std::result::Result<bool, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::Description>, ::std::string::String>,
        merge: ::std::result::Result<super::StandardInputMerge, ::std::string::String>,
    }
    impl ::std::default::Default for StandardInput {
        fn default() -> Self {
            Self {
                close_on_start: Ok(Default::default()),
                description: Ok(Default::default()),
                merge: Ok(super::defaults::standard_input_merge()),
            }
        }
    }
    impl StandardInput {
        pub fn close_on_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.close_on_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close_on_start: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Description>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn merge<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StandardInputMerge>,
            T::Error: ::std::fmt::Display,
        {
            self.merge = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for merge: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StandardInput> for super::StandardInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StandardInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close_on_start: value.close_on_start?,
                description: value.description?,
                merge: value.merge?,
            })
        }
    }
    impl ::std::convert::From<super::StandardInput> for StandardInput {
        fn from(value: super::StandardInput) -> Self {
            Self {
                close_on_start: Ok(value.close_on_start),
                description: Ok(value.description),
                merge: Ok(value.merge),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn action_stderr_log_level() -> super::ActionStderrLogLevel {
        super::ActionStderrLogLevel::Info
    }
    pub(super) fn action_stdout_log_level() -> super::ActionStdoutLogLevel {
        super::ActionStdoutLogLevel::Info
    }
    pub(super) fn file_descriptor_log_level() -> super::FileDescriptorLogLevel {
        super::FileDescriptorLogLevel::Info
    }
    pub(super) fn standard_input_merge() -> super::StandardInputMerge {
        super::StandardInputMerge::Byte
    }
}
