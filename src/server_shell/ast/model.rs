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
#[doc = "An action that can be triggered by a node.  This is a named group of event brokers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action\","]
#[doc = "  \"description\": \"An action that can be triggered by a node.  This is a named group of event brokers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"run\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the action.  Used only for debugging.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"run\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"title\": \"Run Node Handler\","]
#[doc = "          \"description\": \"Execute a handler on the current node.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"handler\","]
#[doc = "            \"parameters\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"handler\": {"]
#[doc = "              \"description\": \"The name of the handler to run on the current node.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"parameters\": {"]
#[doc = "              \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"run-node-handler\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Run Node\","]
#[doc = "          \"description\": \"Execute a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"node\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"for-exit\": {"]
#[doc = "              \"title\": \"For Exit\","]
#[doc = "              \"description\": \"Behavior for the next action in the list depending on the executed node's exit code.\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"title\": \"Exit Behavior\","]
#[doc = "                \"description\": \"Behavior based on the node's exit code.\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"behavior\","]
#[doc = "                  \"source\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"behavior\": {"]
#[doc = "                    \"description\": \"The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script.\","]
#[doc = "                    \"type\": \"string\","]
#[doc = "                    \"enum\": ["]
#[doc = "                      \"run\","]
#[doc = "                      \"skip-next\","]
#[doc = "                      \"skip-all\","]
#[doc = "                      \"abort-script\""]
#[doc = "                    ]"]
#[doc = "                  },"]
#[doc = "                  \"code-end\": {"]
#[doc = "                    \"description\": \"Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound.\","]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  },"]
#[doc = "                  \"code-start\": {"]
#[doc = "                    \"description\": \"Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound.\","]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  },"]
#[doc = "                  \"source\": {"]
#[doc = "                    \"$ref\": \"#/$defs/Source\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"additionalProperties\": false"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"node\": {"]
#[doc = "              \"description\": \"The node ID to run.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"run-node\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Send Event\","]
#[doc = "          \"description\": \"Send an event to the event broker.  It has either a string message, or an integer code, or both, or neither.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"group\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"code\": {"]
#[doc = "              \"description\": \"The integer code to send with the event.  Associated with a signal or exit code or something else.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"integer\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"message\": {"]
#[doc = "              \"description\": \"The message to send with the event.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"name\": {"]
#[doc = "              \"description\": \"The event index to send.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"send-event\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Abort Script\","]
#[doc = "          \"description\": \"Abort the script's execution.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"exit-code\","]
#[doc = "            \"message\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"exit-code\": {"]
#[doc = "              \"description\": \"The exit code to return from the script engine.\","]
#[doc = "              \"type\": \"integer\","]
#[doc = "              \"maximum\": 127.0,"]
#[doc = "              \"minimum\": -128.0"]
#[doc = "            },"]
#[doc = "            \"message\": {"]
#[doc = "              \"description\": \"Message to report to the user.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"abort\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[doc = "The name of the action.  Used only for debugging."]
    pub name: ::std::string::String,
    pub run: ActionRun,
    pub source: Source,
}
impl ::std::convert::From<&Action> for Action {
    fn from(value: &Action) -> Self {
        value.clone()
    }
}
impl Action {
    pub fn builder() -> builder::Action {
        Default::default()
    }
}
#[doc = "A parameter for the action. Must provide exactly one of a value or a value-array."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action Parameter\","]
#[doc = "  \"description\": \"A parameter for the action. Must provide exactly one of a value or a value-array.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionParameter {
    #[doc = "The name of the parameter."]
    pub name: ::std::string::String,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<ComputedValue>,
}
impl ::std::convert::From<&ActionParameter> for ActionParameter {
    fn from(value: &ActionParameter) -> Self {
        value.clone()
    }
}
impl ActionParameter {
    pub fn builder() -> builder::ActionParameter {
        Default::default()
    }
}
#[doc = "`ActionRun`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Run Node Handler\","]
#[doc = "      \"description\": \"Execute a handler on the current node.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"handler\","]
#[doc = "        \"parameters\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"handler\": {"]
#[doc = "          \"description\": \"The name of the handler to run on the current node.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"parameters\": {"]
#[doc = "          \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"run-node-handler\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Run Node\","]
#[doc = "      \"description\": \"Execute a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"node\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"for-exit\": {"]
#[doc = "          \"title\": \"For Exit\","]
#[doc = "          \"description\": \"Behavior for the next action in the list depending on the executed node's exit code.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"Exit Behavior\","]
#[doc = "            \"description\": \"Behavior based on the node's exit code.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"behavior\","]
#[doc = "              \"source\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"behavior\": {"]
#[doc = "                \"description\": \"The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script.\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"run\","]
#[doc = "                  \"skip-next\","]
#[doc = "                  \"skip-all\","]
#[doc = "                  \"abort-script\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"code-end\": {"]
#[doc = "                \"description\": \"Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound.\","]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"code-start\": {"]
#[doc = "                \"description\": \"Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound.\","]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"source\": {"]
#[doc = "                \"$ref\": \"#/$defs/Source\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"node\": {"]
#[doc = "          \"description\": \"The node ID to run.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"run-node\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Send Event\","]
#[doc = "      \"description\": \"Send an event to the event broker.  It has either a string message, or an integer code, or both, or neither.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"group\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"code\": {"]
#[doc = "          \"description\": \"The integer code to send with the event.  Associated with a signal or exit code or something else.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"The message to send with the event.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"The event index to send.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"send-event\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Abort Script\","]
#[doc = "      \"description\": \"Abort the script's execution.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"exit-code\","]
#[doc = "        \"message\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"exit-code\": {"]
#[doc = "          \"description\": \"The exit code to return from the script engine.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 127.0,"]
#[doc = "          \"minimum\": -128.0"]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"Message to report to the user.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"abort\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum ActionRun {
    #[doc = "Run Node Handler\n\nExecute a handler on the current node."]
    #[serde(rename = "run-node-handler")]
    RunNodeHandler {
        #[doc = "The name of the handler to run on the current node."]
        handler: ::std::string::String,
        parameters: NamedParameters,
        source: Source,
    },
    #[doc = "Run Node\n\nExecute a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code."]
    #[serde(rename = "run-node")]
    RunNode {
        #[doc = "Behavior for the next action in the list depending on the executed node's exit code."]
        #[serde(
            rename = "for-exit",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        for_exit: ::std::vec::Vec<ExitBehavior>,
        #[doc = "The node ID to run."]
        node: ::std::string::String,
        source: Source,
    },
    #[doc = "Send Event\n\nSend an event to the event broker.  It has either a string message, or an integer code, or both, or neither."]
    #[serde(rename = "send-event")]
    SendEvent {
        #[doc = "The integer code to send with the event.  Associated with a signal or exit code or something else."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        code: ::std::option::Option<i64>,
        group: ::serde_json::Value,
        #[doc = "The message to send with the event."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        message: ::std::option::Option<::std::string::String>,
        #[doc = "The event index to send."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        name: ::std::option::Option<i64>,
        source: Source,
    },
    #[doc = "Abort Script\n\nAbort the script's execution."]
    #[serde(rename = "abort")]
    Abort {
        #[doc = "The exit code to return from the script engine."]
        #[serde(rename = "exit-code")]
        exit_code: i8,
        #[doc = "Message to report to the user."]
        message: ::std::string::String,
        source: Source,
    },
}
impl ::std::convert::From<&Self> for ActionRun {
    fn from(value: &ActionRun) -> Self {
        value.clone()
    }
}
#[doc = "A number value that is the result of an arithmetic operation on two or more number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Arithmetic Value\","]
#[doc = "  \"description\": \"A number value that is the result of an arithmetic operation on two or more number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"operation\","]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"operation\": {"]
#[doc = "      \"description\": \"The arithmetic operation to perform.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"add\","]
#[doc = "        \"subtract\","]
#[doc = "        \"multiply\","]
#[doc = "        \"divide\","]
#[doc = "        \"modulus\","]
#[doc = "        \"power\","]
#[doc = "        \"round\","]
#[doc = "        \"floor\","]
#[doc = "        \"ceil\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"arithmetic\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ArithmeticValue {
    #[doc = "The arithmetic operation to perform."]
    pub operation: ArithmeticValueOperation,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&ArithmeticValue> for ArithmeticValue {
    fn from(value: &ArithmeticValue) -> Self {
        value.clone()
    }
}
impl ArithmeticValue {
    pub fn builder() -> builder::ArithmeticValue {
        Default::default()
    }
}
#[doc = "The arithmetic operation to perform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The arithmetic operation to perform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"add\","]
#[doc = "    \"subtract\","]
#[doc = "    \"multiply\","]
#[doc = "    \"divide\","]
#[doc = "    \"modulus\","]
#[doc = "    \"power\","]
#[doc = "    \"round\","]
#[doc = "    \"floor\","]
#[doc = "    \"ceil\""]
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
pub enum ArithmeticValueOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "subtract")]
    Subtract,
    #[serde(rename = "multiply")]
    Multiply,
    #[serde(rename = "divide")]
    Divide,
    #[serde(rename = "modulus")]
    Modulus,
    #[serde(rename = "power")]
    Power,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "ceil")]
    Ceil,
}
impl ::std::convert::From<&Self> for ArithmeticValueOperation {
    fn from(value: &ArithmeticValueOperation) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ArithmeticValueOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "subtract"),
            Self::Multiply => write!(f, "multiply"),
            Self::Divide => write!(f, "divide"),
            Self::Modulus => write!(f, "modulus"),
            Self::Power => write!(f, "power"),
            Self::Round => write!(f, "round"),
            Self::Floor => write!(f, "floor"),
            Self::Ceil => write!(f, "ceil"),
        }
    }
}
impl ::std::str::FromStr for ArithmeticValueOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "multiply" => Ok(Self::Multiply),
            "divide" => Ok(Self::Divide),
            "modulus" => Ok(Self::Modulus),
            "power" => Ok(Self::Power),
            "round" => Ok(Self::Round),
            "floor" => Ok(Self::Floor),
            "ceil" => Ok(Self::Ceil),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ArithmeticValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ArithmeticValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ArithmeticValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A string value that is the string representation of a boolean."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Boolean to String Value\","]
#[doc = "  \"description\": \"A string value that is the string representation of a boolean.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"false-string\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"true-string\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"boolean-to-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BooleanToStringValue {
    #[serde(
        rename = "false-string",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub false_string: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub source: Source,
    #[serde(
        rename = "true-string",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub true_string: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedBooleanValue,
}
impl ::std::convert::From<&BooleanToStringValue> for BooleanToStringValue {
    fn from(value: &BooleanToStringValue) -> Self {
        value.clone()
    }
}
impl BooleanToStringValue {
    pub fn builder() -> builder::BooleanToStringValue {
        Default::default()
    }
}
#[doc = "A computed list value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed List Value\","]
#[doc = "  \"description\": \"A computed list value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupBooleanListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantBooleanListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedBooleanListValue {
    LookupBooleanListValue(LookupBooleanListValue),
    ConstantBooleanListValue(ConstantBooleanListValue),
}
impl ::std::convert::From<&Self> for ComputedBooleanListValue {
    fn from(value: &ComputedBooleanListValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupBooleanListValue> for ComputedBooleanListValue {
    fn from(value: LookupBooleanListValue) -> Self {
        Self::LookupBooleanListValue(value)
    }
}
impl ::std::convert::From<ConstantBooleanListValue> for ComputedBooleanListValue {
    fn from(value: ConstantBooleanListValue) -> Self {
        Self::ConstantBooleanListValue(value)
    }
}
#[doc = "A computed map value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Map Value\","]
#[doc = "  \"description\": \"A computed map value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupBooleanMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/UnionBooleanMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantBooleanMapValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedBooleanMapValue {
    LookupBooleanMapValue(LookupBooleanMapValue),
    UnionBooleanMapValue(UnionBooleanMapValue),
    ConstantBooleanMapValue(ConstantBooleanMapValue),
}
impl ::std::convert::From<&Self> for ComputedBooleanMapValue {
    fn from(value: &ComputedBooleanMapValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupBooleanMapValue> for ComputedBooleanMapValue {
    fn from(value: LookupBooleanMapValue) -> Self {
        Self::LookupBooleanMapValue(value)
    }
}
impl ::std::convert::From<UnionBooleanMapValue> for ComputedBooleanMapValue {
    fn from(value: UnionBooleanMapValue) -> Self {
        Self::UnionBooleanMapValue(value)
    }
}
impl ::std::convert::From<ConstantBooleanMapValue> for ComputedBooleanMapValue {
    fn from(value: ConstantBooleanMapValue) -> Self {
        Self::ConstantBooleanMapValue(value)
    }
}
#[doc = "A computed boolean value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Boolean Value\","]
#[doc = "  \"description\": \"A computed boolean value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LogicalBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantBooleanValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedBooleanValue {
    LookupBooleanValue(LookupBooleanValue),
    LogicalBooleanValue(LogicalBooleanValue),
    ConstantBooleanValue(ConstantBooleanValue),
}
impl ::std::convert::From<&Self> for ComputedBooleanValue {
    fn from(value: &ComputedBooleanValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupBooleanValue> for ComputedBooleanValue {
    fn from(value: LookupBooleanValue) -> Self {
        Self::LookupBooleanValue(value)
    }
}
impl ::std::convert::From<LogicalBooleanValue> for ComputedBooleanValue {
    fn from(value: LogicalBooleanValue) -> Self {
        Self::LogicalBooleanValue(value)
    }
}
impl ::std::convert::From<ConstantBooleanValue> for ComputedBooleanValue {
    fn from(value: ConstantBooleanValue) -> Self {
        Self::ConstantBooleanValue(value)
    }
}
#[doc = "A computed list value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed List Value\","]
#[doc = "  \"description\": \"A computed list value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantNumberListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedNumberListValue {
    LookupNumberListValue(LookupNumberListValue),
    ConstantNumberListValue(ConstantNumberListValue),
}
impl ::std::convert::From<&Self> for ComputedNumberListValue {
    fn from(value: &ComputedNumberListValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupNumberListValue> for ComputedNumberListValue {
    fn from(value: LookupNumberListValue) -> Self {
        Self::LookupNumberListValue(value)
    }
}
impl ::std::convert::From<ConstantNumberListValue> for ComputedNumberListValue {
    fn from(value: ConstantNumberListValue) -> Self {
        Self::ConstantNumberListValue(value)
    }
}
#[doc = "A computed map value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Map Value\","]
#[doc = "  \"description\": \"A computed map value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupNumberMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/UnionNumberMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantNumberMapValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedNumberMapValue {
    LookupNumberMapValue(LookupNumberMapValue),
    UnionNumberMapValue(UnionNumberMapValue),
    ConstantNumberMapValue(ConstantNumberMapValue),
}
impl ::std::convert::From<&Self> for ComputedNumberMapValue {
    fn from(value: &ComputedNumberMapValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupNumberMapValue> for ComputedNumberMapValue {
    fn from(value: LookupNumberMapValue) -> Self {
        Self::LookupNumberMapValue(value)
    }
}
impl ::std::convert::From<UnionNumberMapValue> for ComputedNumberMapValue {
    fn from(value: UnionNumberMapValue) -> Self {
        Self::UnionNumberMapValue(value)
    }
}
impl ::std::convert::From<ConstantNumberMapValue> for ComputedNumberMapValue {
    fn from(value: ConstantNumberMapValue) -> Self {
        Self::ConstantNumberMapValue(value)
    }
}
#[doc = "A computed number value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Number Value\","]
#[doc = "  \"description\": \"A computed number value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupNumberValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ArithmeticValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantNumberValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedNumberValue {
    LookupNumberValue(LookupNumberValue),
    ArithmeticValue(::std::boxed::Box<ArithmeticValue>),
    ConstantNumberValue(ConstantNumberValue),
}
impl ::std::convert::From<&Self> for ComputedNumberValue {
    fn from(value: &ComputedNumberValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupNumberValue> for ComputedNumberValue {
    fn from(value: LookupNumberValue) -> Self {
        Self::LookupNumberValue(value)
    }
}
impl ::std::convert::From<::std::boxed::Box<ArithmeticValue>> for ComputedNumberValue {
    fn from(value: ::std::boxed::Box<ArithmeticValue>) -> Self {
        Self::ArithmeticValue(value)
    }
}
impl ::std::convert::From<ConstantNumberValue> for ComputedNumberValue {
    fn from(value: ConstantNumberValue) -> Self {
        Self::ConstantNumberValue(value)
    }
}
#[doc = "A computed list value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed List Value\","]
#[doc = "  \"description\": \"A computed list value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupStringListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantStringListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedStringListValue {
    LookupStringListValue(LookupStringListValue),
    ConstantStringListValue(ConstantStringListValue),
}
impl ::std::convert::From<&Self> for ComputedStringListValue {
    fn from(value: &ComputedStringListValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupStringListValue> for ComputedStringListValue {
    fn from(value: LookupStringListValue) -> Self {
        Self::LookupStringListValue(value)
    }
}
impl ::std::convert::From<ConstantStringListValue> for ComputedStringListValue {
    fn from(value: ConstantStringListValue) -> Self {
        Self::ConstantStringListValue(value)
    }
}
#[doc = "A computed map value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Map Value\","]
#[doc = "  \"description\": \"A computed map value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupStringMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/UnionStringMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantStringMapValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedStringMapValue {
    LookupStringMapValue(LookupStringMapValue),
    UnionStringMapValue(UnionStringMapValue),
    ConstantStringMapValue(ConstantStringMapValue),
}
impl ::std::convert::From<&Self> for ComputedStringMapValue {
    fn from(value: &ComputedStringMapValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupStringMapValue> for ComputedStringMapValue {
    fn from(value: LookupStringMapValue) -> Self {
        Self::LookupStringMapValue(value)
    }
}
impl ::std::convert::From<UnionStringMapValue> for ComputedStringMapValue {
    fn from(value: UnionStringMapValue) -> Self {
        Self::UnionStringMapValue(value)
    }
}
impl ::std::convert::From<ConstantStringMapValue> for ComputedStringMapValue {
    fn from(value: ConstantStringMapValue) -> Self {
        Self::ConstantStringMapValue(value)
    }
}
#[doc = "A computed string value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed String Value\","]
#[doc = "  \"description\": \"A computed string value.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupStateStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LookupEnvStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConcatenatedStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/NumberToStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/BooleanToStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ListToStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapToStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ConstantStringValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedStringValue {
    LookupStateStringValue(LookupStateStringValue),
    LookupEnvStringValue(LookupEnvStringValue),
    ConcatenatedStringValue(ConcatenatedStringValue),
    NumberToStringValue(NumberToStringValue),
    BooleanToStringValue(BooleanToStringValue),
    ListToStringValue(ListToStringValue),
    MapToStringValue(MapToStringValue),
    ConstantStringValue(ConstantStringValue),
}
impl ::std::convert::From<&Self> for ComputedStringValue {
    fn from(value: &ComputedStringValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LookupStateStringValue> for ComputedStringValue {
    fn from(value: LookupStateStringValue) -> Self {
        Self::LookupStateStringValue(value)
    }
}
impl ::std::convert::From<LookupEnvStringValue> for ComputedStringValue {
    fn from(value: LookupEnvStringValue) -> Self {
        Self::LookupEnvStringValue(value)
    }
}
impl ::std::convert::From<ConcatenatedStringValue> for ComputedStringValue {
    fn from(value: ConcatenatedStringValue) -> Self {
        Self::ConcatenatedStringValue(value)
    }
}
impl ::std::convert::From<NumberToStringValue> for ComputedStringValue {
    fn from(value: NumberToStringValue) -> Self {
        Self::NumberToStringValue(value)
    }
}
impl ::std::convert::From<BooleanToStringValue> for ComputedStringValue {
    fn from(value: BooleanToStringValue) -> Self {
        Self::BooleanToStringValue(value)
    }
}
impl ::std::convert::From<ListToStringValue> for ComputedStringValue {
    fn from(value: ListToStringValue) -> Self {
        Self::ListToStringValue(value)
    }
}
impl ::std::convert::From<MapToStringValue> for ComputedStringValue {
    fn from(value: MapToStringValue) -> Self {
        Self::MapToStringValue(value)
    }
}
impl ::std::convert::From<ConstantStringValue> for ComputedStringValue {
    fn from(value: ConstantStringValue) -> Self {
        Self::ConstantStringValue(value)
    }
}
#[doc = "A value that is computed at runtime.  This can be a constant value, a lookup from a node's state, or an array of values, or an operation on a list of values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Value\","]
#[doc = "  \"description\": \"A value that is computed at runtime.  This can be a constant value, a lookup from a node's state, or an array of values, or an operation on a list of values.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberMapValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanMapValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComputedValue {
    StringValue(ComputedStringValue),
    NumberValue(ComputedNumberValue),
    BooleanValue(ComputedBooleanValue),
    StringListValue(ComputedStringListValue),
    NumberListValue(ComputedNumberListValue),
    BooleanListValue(ComputedBooleanListValue),
    StringMapValue(ComputedStringMapValue),
    NumberMapValue(ComputedNumberMapValue),
    BooleanMapValue(ComputedBooleanMapValue),
}
impl ::std::convert::From<&Self> for ComputedValue {
    fn from(value: &ComputedValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ComputedStringValue> for ComputedValue {
    fn from(value: ComputedStringValue) -> Self {
        Self::StringValue(value)
    }
}
impl ::std::convert::From<ComputedNumberValue> for ComputedValue {
    fn from(value: ComputedNumberValue) -> Self {
        Self::NumberValue(value)
    }
}
impl ::std::convert::From<ComputedBooleanValue> for ComputedValue {
    fn from(value: ComputedBooleanValue) -> Self {
        Self::BooleanValue(value)
    }
}
impl ::std::convert::From<ComputedStringListValue> for ComputedValue {
    fn from(value: ComputedStringListValue) -> Self {
        Self::StringListValue(value)
    }
}
impl ::std::convert::From<ComputedNumberListValue> for ComputedValue {
    fn from(value: ComputedNumberListValue) -> Self {
        Self::NumberListValue(value)
    }
}
impl ::std::convert::From<ComputedBooleanListValue> for ComputedValue {
    fn from(value: ComputedBooleanListValue) -> Self {
        Self::BooleanListValue(value)
    }
}
impl ::std::convert::From<ComputedStringMapValue> for ComputedValue {
    fn from(value: ComputedStringMapValue) -> Self {
        Self::StringMapValue(value)
    }
}
impl ::std::convert::From<ComputedNumberMapValue> for ComputedValue {
    fn from(value: ComputedNumberMapValue) -> Self {
        Self::NumberMapValue(value)
    }
}
impl ::std::convert::From<ComputedBooleanMapValue> for ComputedValue {
    fn from(value: ComputedBooleanMapValue) -> Self {
        Self::BooleanMapValue(value)
    }
}
#[doc = "A string value that is the concatenation of multiple string values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Concatenated String Value\","]
#[doc = "  \"description\": \"A string value that is the concatenation of multiple string values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"concatenated-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConcatenatedStringValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedStringListValue,
}
impl ::std::convert::From<&ConcatenatedStringValue> for ConcatenatedStringValue {
    fn from(value: &ConcatenatedStringValue) -> Self {
        value.clone()
    }
}
impl ConcatenatedStringValue {
    pub fn builder() -> builder::ConcatenatedStringValue {
        Default::default()
    }
}
#[doc = "A constant boolean list value.  Can include expanding a sub-list within the list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Boolean List Value\","]
#[doc = "  \"description\": \"A constant boolean list value.  Can include expanding a sub-list within the list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-boolean-list\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant boolean list value.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantBooleanListValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant boolean list value."]
    pub value: ::std::vec::Vec<ConstantBooleanListValueValueItem>,
}
impl ::std::convert::From<&ConstantBooleanListValue> for ConstantBooleanListValue {
    fn from(value: &ConstantBooleanListValue) -> Self {
        value.clone()
    }
}
impl ConstantBooleanListValue {
    pub fn builder() -> builder::ConstantBooleanListValue {
        Default::default()
    }
}
#[doc = "`ConstantBooleanListValueValueItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConstantBooleanListValueValueItem {
    Value(ComputedBooleanValue),
    ListValue(ComputedBooleanListValue),
}
impl ::std::convert::From<&Self> for ConstantBooleanListValueValueItem {
    fn from(value: &ConstantBooleanListValueValueItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ComputedBooleanValue> for ConstantBooleanListValueValueItem {
    fn from(value: ComputedBooleanValue) -> Self {
        Self::Value(value)
    }
}
impl ::std::convert::From<ComputedBooleanListValue> for ConstantBooleanListValueValueItem {
    fn from(value: ComputedBooleanListValue) -> Self {
        Self::ListValue(value)
    }
}
#[doc = "A constant boolean map value.  Can include expanding a sub-map within the map."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Boolean Map Value\","]
#[doc = "  \"description\": \"A constant boolean map value.  Can include expanding a sub-map within the map.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-boolean-map\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant boolean map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct ConstantBooleanMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant boolean map value."]
    pub value: ::std::collections::HashMap<ConstantBooleanMapValueValueKey, ComputedBooleanValue>,
}
impl ::std::convert::From<&ConstantBooleanMapValue> for ConstantBooleanMapValue {
    fn from(value: &ConstantBooleanMapValue) -> Self {
        value.clone()
    }
}
impl ConstantBooleanMapValue {
    pub fn builder() -> builder::ConstantBooleanMapValue {
        Default::default()
    }
}
#[doc = "`ConstantBooleanMapValueValueKey`"]
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
pub struct ConstantBooleanMapValueValueKey(::std::string::String);
impl ::std::ops::Deref for ConstantBooleanMapValueValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConstantBooleanMapValueValueKey> for ::std::string::String {
    fn from(value: ConstantBooleanMapValueValueKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConstantBooleanMapValueValueKey> for ConstantBooleanMapValueValueKey {
    fn from(value: &ConstantBooleanMapValueValueKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ConstantBooleanMapValueValueKey {
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
impl ::std::convert::TryFrom<&str> for ConstantBooleanMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConstantBooleanMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConstantBooleanMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConstantBooleanMapValueValueKey {
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
#[doc = "A constant boolean value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Boolean Value\","]
#[doc = "  \"description\": \"A constant boolean value.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-boolean\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant boolean value.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantBooleanValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant boolean value."]
    pub value: bool,
}
impl ::std::convert::From<&ConstantBooleanValue> for ConstantBooleanValue {
    fn from(value: &ConstantBooleanValue) -> Self {
        value.clone()
    }
}
impl ConstantBooleanValue {
    pub fn builder() -> builder::ConstantBooleanValue {
        Default::default()
    }
}
#[doc = "A constant number list value.  Can include expanding a sub-list within the list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Number List Value\","]
#[doc = "  \"description\": \"A constant number list value.  Can include expanding a sub-list within the list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-number-list\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant number list value.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantNumberListValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant number list value."]
    pub value: ::std::vec::Vec<ConstantNumberListValueValueItem>,
}
impl ::std::convert::From<&ConstantNumberListValue> for ConstantNumberListValue {
    fn from(value: &ConstantNumberListValue) -> Self {
        value.clone()
    }
}
impl ConstantNumberListValue {
    pub fn builder() -> builder::ConstantNumberListValue {
        Default::default()
    }
}
#[doc = "`ConstantNumberListValueValueItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConstantNumberListValueValueItem {
    Value(ComputedNumberValue),
    ListValue(ComputedNumberListValue),
}
impl ::std::convert::From<&Self> for ConstantNumberListValueValueItem {
    fn from(value: &ConstantNumberListValueValueItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ComputedNumberValue> for ConstantNumberListValueValueItem {
    fn from(value: ComputedNumberValue) -> Self {
        Self::Value(value)
    }
}
impl ::std::convert::From<ComputedNumberListValue> for ConstantNumberListValueValueItem {
    fn from(value: ComputedNumberListValue) -> Self {
        Self::ListValue(value)
    }
}
#[doc = "A constant number map value.  Can include expanding a sub-map within the map."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Number Map Value\","]
#[doc = "  \"description\": \"A constant number map value.  Can include expanding a sub-map within the map.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-number-map\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant number map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct ConstantNumberMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant number map value."]
    pub value: ::std::collections::HashMap<ConstantNumberMapValueValueKey, ComputedNumberValue>,
}
impl ::std::convert::From<&ConstantNumberMapValue> for ConstantNumberMapValue {
    fn from(value: &ConstantNumberMapValue) -> Self {
        value.clone()
    }
}
impl ConstantNumberMapValue {
    pub fn builder() -> builder::ConstantNumberMapValue {
        Default::default()
    }
}
#[doc = "`ConstantNumberMapValueValueKey`"]
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
pub struct ConstantNumberMapValueValueKey(::std::string::String);
impl ::std::ops::Deref for ConstantNumberMapValueValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConstantNumberMapValueValueKey> for ::std::string::String {
    fn from(value: ConstantNumberMapValueValueKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConstantNumberMapValueValueKey> for ConstantNumberMapValueValueKey {
    fn from(value: &ConstantNumberMapValueValueKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ConstantNumberMapValueValueKey {
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
impl ::std::convert::TryFrom<&str> for ConstantNumberMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConstantNumberMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConstantNumberMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConstantNumberMapValueValueKey {
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
#[doc = "A constant number value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Number Value\","]
#[doc = "  \"description\": \"A constant number value.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-number\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant number value.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantNumberValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: f64,
}
impl ::std::convert::From<&ConstantNumberValue> for ConstantNumberValue {
    fn from(value: &ConstantNumberValue) -> Self {
        value.clone()
    }
}
impl ConstantNumberValue {
    pub fn builder() -> builder::ConstantNumberValue {
        Default::default()
    }
}
#[doc = "A constant string list value.  Can include expanding a sub-list within the list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String List Value\","]
#[doc = "  \"description\": \"A constant string list value.  Can include expanding a sub-list within the list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-string-list\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant string list value.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantStringListValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant string list value."]
    pub value: ::std::vec::Vec<ConstantStringListValueValueItem>,
}
impl ::std::convert::From<&ConstantStringListValue> for ConstantStringListValue {
    fn from(value: &ConstantStringListValue) -> Self {
        value.clone()
    }
}
impl ConstantStringListValue {
    pub fn builder() -> builder::ConstantStringListValue {
        Default::default()
    }
}
#[doc = "`ConstantStringListValueValueItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConstantStringListValueValueItem {
    Value(ComputedStringValue),
    ListValue(ComputedStringListValue),
}
impl ::std::convert::From<&Self> for ConstantStringListValueValueItem {
    fn from(value: &ConstantStringListValueValueItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ComputedStringValue> for ConstantStringListValueValueItem {
    fn from(value: ComputedStringValue) -> Self {
        Self::Value(value)
    }
}
impl ::std::convert::From<ComputedStringListValue> for ConstantStringListValueValueItem {
    fn from(value: ComputedStringListValue) -> Self {
        Self::ListValue(value)
    }
}
#[doc = "A constant string map value.  Can include expanding a sub-map within the map."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String Map Value\","]
#[doc = "  \"description\": \"A constant string map value.  Can include expanding a sub-map within the map.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-string-map\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant string map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/ComputedStringValue\""]
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
pub struct ConstantStringMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant string map value."]
    pub value: ::std::collections::HashMap<ConstantStringMapValueValueKey, ComputedStringValue>,
}
impl ::std::convert::From<&ConstantStringMapValue> for ConstantStringMapValue {
    fn from(value: &ConstantStringMapValue) -> Self {
        value.clone()
    }
}
impl ConstantStringMapValue {
    pub fn builder() -> builder::ConstantStringMapValue {
        Default::default()
    }
}
#[doc = "`ConstantStringMapValueValueKey`"]
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
pub struct ConstantStringMapValueValueKey(::std::string::String);
impl ::std::ops::Deref for ConstantStringMapValueValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConstantStringMapValueValueKey> for ::std::string::String {
    fn from(value: ConstantStringMapValueValueKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConstantStringMapValueValueKey> for ConstantStringMapValueValueKey {
    fn from(value: &ConstantStringMapValueValueKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ConstantStringMapValueValueKey {
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
impl ::std::convert::TryFrom<&str> for ConstantStringMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConstantStringMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConstantStringMapValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConstantStringMapValueValueKey {
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
#[doc = "A constant string value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String Value\","]
#[doc = "  \"description\": \"A constant string value.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"constant-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant string value.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstantStringValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The constant string value."]
    pub value: ::std::string::String,
}
impl ::std::convert::From<&ConstantStringValue> for ConstantStringValue {
    fn from(value: &ConstantStringValue) -> Self {
        value.clone()
    }
}
impl ConstantStringValue {
    pub fn builder() -> builder::ConstantStringValue {
        Default::default()
    }
}
#[doc = "An environment variable for the node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Environment Variable\","]
#[doc = "  \"description\": \"An environment variable for the node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EnvironmentVariable {
    pub name: ComputedValue,
    pub source: Source,
    pub value: ComputedValue,
}
impl ::std::convert::From<&EnvironmentVariable> for EnvironmentVariable {
    fn from(value: &EnvironmentVariable) -> Self {
        value.clone()
    }
}
impl EnvironmentVariable {
    pub fn builder() -> builder::EnvironmentVariable {
        Default::default()
    }
}
#[doc = "An event listener that is triggered by the node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Listener\","]
#[doc = "  \"description\": \"An event listener that is triggered by the node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"$ref\": \"#/$defs/OrderedActions\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The event index that triggers this action list.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventListener {
    pub actions: OrderedActions,
    #[doc = "The event index that triggers this action list."]
    pub name: i64,
    pub source: Source,
}
impl ::std::convert::From<&EventListener> for EventListener {
    fn from(value: &EventListener) -> Self {
        value.clone()
    }
}
impl EventListener {
    pub fn builder() -> builder::EventListener {
        Default::default()
    }
}
#[doc = "An event name that can be triggered by a node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Name\","]
#[doc = "  \"description\": \"An event name that can be triggered by a node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"source\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The event index that triggers this action list.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text description of the event.  This is used for debugging and logging.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventName {
    #[doc = "The event index that triggers this action list."]
    pub name: i64,
    pub source: Source,
    #[doc = "The text description of the event.  This is used for debugging and logging."]
    pub text: ::std::string::String,
}
impl ::std::convert::From<&EventName> for EventName {
    fn from(value: &EventName) -> Self {
        value.clone()
    }
}
impl EventName {
    pub fn builder() -> builder::EventName {
        Default::default()
    }
}
#[doc = "An action that is triggered when the node exits."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Exit Action\","]
#[doc = "  \"description\": \"An action that is triggered when the node exits.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"$ref\": \"#/$defs/OrderedActions\""]
#[doc = "    },"]
#[doc = "    \"code-end\": {"]
#[doc = "      \"description\": \"Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"code-start\": {"]
#[doc = "      \"description\": \"Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExitAction {
    pub actions: OrderedActions,
    #[doc = "Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound."]
    #[serde(
        rename = "code-end",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_end: ::std::option::Option<i64>,
    #[doc = "Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound."]
    #[serde(
        rename = "code-start",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_start: ::std::option::Option<i64>,
    pub source: Source,
}
impl ::std::convert::From<&ExitAction> for ExitAction {
    fn from(value: &ExitAction) -> Self {
        value.clone()
    }
}
impl ExitAction {
    pub fn builder() -> builder::ExitAction {
        Default::default()
    }
}
#[doc = "Behavior based on the node's exit code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Exit Behavior\","]
#[doc = "  \"description\": \"Behavior based on the node's exit code.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"behavior\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"behavior\": {"]
#[doc = "      \"description\": \"The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"run\","]
#[doc = "        \"skip-next\","]
#[doc = "        \"skip-all\","]
#[doc = "        \"abort-script\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"code-end\": {"]
#[doc = "      \"description\": \"Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"code-start\": {"]
#[doc = "      \"description\": \"Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExitBehavior {
    #[doc = "The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script."]
    pub behavior: ExitBehaviorBehavior,
    #[doc = "Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound."]
    #[serde(
        rename = "code-end",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_end: ::std::option::Option<i64>,
    #[doc = "Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound."]
    #[serde(
        rename = "code-start",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_start: ::std::option::Option<i64>,
    pub source: Source,
}
impl ::std::convert::From<&ExitBehavior> for ExitBehavior {
    fn from(value: &ExitBehavior) -> Self {
        value.clone()
    }
}
impl ExitBehavior {
    pub fn builder() -> builder::ExitBehavior {
        Default::default()
    }
}
#[doc = "The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The behavior to take based on the exit code of the node.  'run' means run the next action, 'skip-next' means skip the next action, 'skip-all' means skip all remaining actions, and 'abort-script' means terminate the script.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"run\","]
#[doc = "    \"skip-next\","]
#[doc = "    \"skip-all\","]
#[doc = "    \"abort-script\""]
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
pub enum ExitBehaviorBehavior {
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "skip-next")]
    SkipNext,
    #[serde(rename = "skip-all")]
    SkipAll,
    #[serde(rename = "abort-script")]
    AbortScript,
}
impl ::std::convert::From<&Self> for ExitBehaviorBehavior {
    fn from(value: &ExitBehaviorBehavior) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExitBehaviorBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Run => write!(f, "run"),
            Self::SkipNext => write!(f, "skip-next"),
            Self::SkipAll => write!(f, "skip-all"),
            Self::AbortScript => write!(f, "abort-script"),
        }
    }
}
impl ::std::str::FromStr for ExitBehaviorBehavior {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "run" => Ok(Self::Run),
            "skip-next" => Ok(Self::SkipNext),
            "skip-all" => Ok(Self::SkipAll),
            "abort-script" => Ok(Self::AbortScript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExitBehaviorBehavior {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExitBehaviorBehavior {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExitBehaviorBehavior {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A string value that is the string representation of a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List to String Value\","]
#[doc = "  \"description\": \"A string value that is the string representation of a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"list-to-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListToStringValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub separator: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedStringListValue,
}
impl ::std::convert::From<&ListToStringValue> for ListToStringValue {
    fn from(value: &ListToStringValue) -> Self {
        value.clone()
    }
}
impl ListToStringValue {
    pub fn builder() -> builder::ListToStringValue {
        Default::default()
    }
}
#[doc = "A boolean value that is the result of a logical operation on two or more boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Logical Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical operation on two or more boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"operation\","]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"operation\": {"]
#[doc = "      \"description\": \"The logical operation to perform.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"and\","]
#[doc = "        \"or\","]
#[doc = "        \"not\","]
#[doc = "        \"xor\","]
#[doc = "        \"nand\","]
#[doc = "        \"nor\","]
#[doc = "        \"xnor\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"logical\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogicalBooleanValue {
    #[doc = "The logical operation to perform."]
    pub operation: LogicalBooleanValueOperation,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedBooleanListValue,
}
impl ::std::convert::From<&LogicalBooleanValue> for LogicalBooleanValue {
    fn from(value: &LogicalBooleanValue) -> Self {
        value.clone()
    }
}
impl LogicalBooleanValue {
    pub fn builder() -> builder::LogicalBooleanValue {
        Default::default()
    }
}
#[doc = "The logical operation to perform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The logical operation to perform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"and\","]
#[doc = "    \"or\","]
#[doc = "    \"not\","]
#[doc = "    \"xor\","]
#[doc = "    \"nand\","]
#[doc = "    \"nor\","]
#[doc = "    \"xnor\""]
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
pub enum LogicalBooleanValueOperation {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "xor")]
    Xor,
    #[serde(rename = "nand")]
    Nand,
    #[serde(rename = "nor")]
    Nor,
    #[serde(rename = "xnor")]
    Xnor,
}
impl ::std::convert::From<&Self> for LogicalBooleanValueOperation {
    fn from(value: &LogicalBooleanValueOperation) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LogicalBooleanValueOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
            Self::Xor => write!(f, "xor"),
            Self::Nand => write!(f, "nand"),
            Self::Nor => write!(f, "nor"),
            Self::Xnor => write!(f, "xnor"),
        }
    }
}
impl ::std::str::FromStr for LogicalBooleanValueOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            "xor" => Ok(Self::Xor),
            "nand" => Ok(Self::Nand),
            "nor" => Ok(Self::Nor),
            "xnor" => Ok(Self::Xnor),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LogicalBooleanValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LogicalBooleanValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LogicalBooleanValueOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A boolean list value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Boolean List Value\","]
#[doc = "  \"description\": \"A boolean list value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-boolean-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupBooleanListValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupBooleanListValue> for LookupBooleanListValue {
    fn from(value: &LookupBooleanListValue) -> Self {
        value.clone()
    }
}
impl LookupBooleanListValue {
    pub fn builder() -> builder::LookupBooleanListValue {
        Default::default()
    }
}
#[doc = "A boolean map value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Boolean Map Value\","]
#[doc = "  \"description\": \"A boolean map value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-boolean-map\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupBooleanMapValue {
    pub name: ComputedStringValue,
    pub node: ComputedStringValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupBooleanMapValue> for LookupBooleanMapValue {
    fn from(value: &LookupBooleanMapValue) -> Self {
        value.clone()
    }
}
impl LookupBooleanMapValue {
    pub fn builder() -> builder::LookupBooleanMapValue {
        Default::default()
    }
}
#[doc = "A boolean value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupBooleanValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupBooleanValue> for LookupBooleanValue {
    fn from(value: &LookupBooleanValue) -> Self {
        value.clone()
    }
}
impl LookupBooleanValue {
    pub fn builder() -> builder::LookupBooleanValue {
        Default::default()
    }
}
#[doc = "A string value that is looked up from a node's environment."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String Value\","]
#[doc = "  \"description\": \"A string value that is looked up from a node's environment.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-env-string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupEnvStringValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupEnvStringValue> for LookupEnvStringValue {
    fn from(value: &LookupEnvStringValue) -> Self {
        value.clone()
    }
}
impl LookupEnvStringValue {
    pub fn builder() -> builder::LookupEnvStringValue {
        Default::default()
    }
}
#[doc = "A number list value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Number List Value\","]
#[doc = "  \"description\": \"A number list value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-number-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupNumberListValue {
    pub name: ComputedStringValue,
    pub node: ComputedStringValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupNumberListValue> for LookupNumberListValue {
    fn from(value: &LookupNumberListValue) -> Self {
        value.clone()
    }
}
impl LookupNumberListValue {
    pub fn builder() -> builder::LookupNumberListValue {
        Default::default()
    }
}
#[doc = "A number map value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Number Map Value\","]
#[doc = "  \"description\": \"A number map value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-number-map\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupNumberMapValue {
    pub name: ComputedStringValue,
    pub node: ComputedStringValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupNumberMapValue> for LookupNumberMapValue {
    fn from(value: &LookupNumberMapValue) -> Self {
        value.clone()
    }
}
impl LookupNumberMapValue {
    pub fn builder() -> builder::LookupNumberMapValue {
        Default::default()
    }
}
#[doc = "A number value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Number Value\","]
#[doc = "  \"description\": \"A number value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupNumberValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupNumberValue> for LookupNumberValue {
    fn from(value: &LookupNumberValue) -> Self {
        value.clone()
    }
}
impl LookupNumberValue {
    pub fn builder() -> builder::LookupNumberValue {
        Default::default()
    }
}
#[doc = "A string value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String Value\","]
#[doc = "  \"description\": \"A string value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupStateStringValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupStateStringValue> for LookupStateStringValue {
    fn from(value: &LookupStateStringValue) -> Self {
        value.clone()
    }
}
impl LookupStateStringValue {
    pub fn builder() -> builder::LookupStateStringValue {
        Default::default()
    }
}
#[doc = "A string list value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String List Value\","]
#[doc = "  \"description\": \"A string list value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-string-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupStringListValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupStringListValue> for LookupStringListValue {
    fn from(value: &LookupStringListValue) -> Self {
        value.clone()
    }
}
impl LookupStringListValue {
    pub fn builder() -> builder::LookupStringListValue {
        Default::default()
    }
}
#[doc = "A string map value that is looked up from a node's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String Map Value\","]
#[doc = "  \"description\": \"A string map value that is looked up from a node's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"node\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"lookup-state-string-map\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupStringMapValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupStringMapValue> for LookupStringMapValue {
    fn from(value: &LookupStringMapValue) -> Self {
        value.clone()
    }
}
impl LookupStringMapValue {
    pub fn builder() -> builder::LookupStringMapValue {
        Default::default()
    }
}
#[doc = "A string value that is the string representation of a map."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Map to String Value\","]
#[doc = "  \"description\": \"A string value that is the string representation of a map.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item-separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"key-separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"map-to-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringMapValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapToStringValue {
    #[serde(
        rename = "item-separator",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub item_separator: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    #[serde(
        rename = "key-separator",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub key_separator: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedStringMapValue,
}
impl ::std::convert::From<&MapToStringValue> for MapToStringValue {
    fn from(value: &MapToStringValue) -> Self {
        value.clone()
    }
}
impl MapToStringValue {
    pub fn builder() -> builder::MapToStringValue {
        Default::default()
    }
}
#[doc = "The parameters for the action."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The parameters for the action.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"title\": \"Action Parameter\","]
#[doc = "    \"description\": \"A parameter for the action. Must provide exactly one of a value or a value-array.\","]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"required\": ["]
#[doc = "      \"name\","]
#[doc = "      \"source\""]
#[doc = "    ],"]
#[doc = "    \"properties\": {"]
#[doc = "      \"name\": {"]
#[doc = "        \"description\": \"The name of the parameter.\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"source\": {"]
#[doc = "        \"$ref\": \"#/$defs/Source\""]
#[doc = "      },"]
#[doc = "      \"value\": {"]
#[doc = "        \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"additionalProperties\": false"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NamedParameters(pub ::std::vec::Vec<ActionParameter>);
impl ::std::ops::Deref for NamedParameters {
    type Target = ::std::vec::Vec<ActionParameter>;
    fn deref(&self) -> &::std::vec::Vec<ActionParameter> {
        &self.0
    }
}
impl ::std::convert::From<NamedParameters> for ::std::vec::Vec<ActionParameter> {
    fn from(value: NamedParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NamedParameters> for NamedParameters {
    fn from(value: &NamedParameters) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<ActionParameter>> for NamedParameters {
    fn from(value: ::std::vec::Vec<ActionParameter>) -> Self {
        Self(value)
    }
}
#[doc = "An abstract syntax tree (AST) schema for a Native Shell."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Native Shell AST Schema\","]
#[doc = "  \"description\": \"An abstract syntax tree (AST) schema for a Native Shell.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default-start\","]
#[doc = "    \"events\","]
#[doc = "    \"nodes\","]
#[doc = "    \"source\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default-start\": {"]
#[doc = "      \"description\": \"The default start nodes for the script.  These are the nodes that is executed when the script starts.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"The name of the node to start.\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"events\": {"]
#[doc = "      \"title\": \"Event Name List\","]
#[doc = "      \"description\": \"The event names that nodes can trigger.  These construct a named group of event brokers, which are referenced by the nodes through the index.  Eventually, this may have a parameter list, but for the moment, events can carry a message and/or a code.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Event Name\","]
#[doc = "        \"description\": \"An event name that can be triggered by a node.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"name\","]
#[doc = "          \"source\","]
#[doc = "          \"text\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"The event index that triggers this action list.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          },"]
#[doc = "          \"text\": {"]
#[doc = "            \"description\": \"The text description of the event.  This is used for debugging and logging.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"title\": \"Node List\","]
#[doc = "      \"description\": \"The nodes in the AST.  Each node is a module with a backing builder.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Node\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"The version of the AST schema.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$spdx-license\": \"MIT\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NativeShellAstSchema {
    #[doc = "The default start nodes for the script.  These are the nodes that is executed when the script starts."]
    #[serde(rename = "default-start")]
    pub default_start: ::std::vec::Vec<::std::string::String>,
    #[doc = "The event names that nodes can trigger.  These construct a named group of event brokers, which are referenced by the nodes through the index.  Eventually, this may have a parameter list, but for the moment, events can carry a message and/or a code."]
    pub events: ::std::vec::Vec<EventName>,
    #[doc = "The nodes in the AST.  Each node is a module with a backing builder."]
    pub nodes: ::std::vec::Vec<Node>,
    pub source: Source,
    #[doc = "The version of the AST schema."]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&NativeShellAstSchema> for NativeShellAstSchema {
    fn from(value: &NativeShellAstSchema) -> Self {
        value.clone()
    }
}
impl NativeShellAstSchema {
    pub fn builder() -> builder::NativeShellAstSchema {
        Default::default()
    }
}
#[doc = "A node in the AST.  Each node has a name, a type, and a list of actions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Node\","]
#[doc = "  \"description\": \"A node in the AST.  Each node has a name, a type, and a list of actions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"environment\","]
#[doc = "    \"event-listeners\","]
#[doc = "    \"exit-actions\","]
#[doc = "    \"initial-parameters\","]
#[doc = "    \"module\","]
#[doc = "    \"name\","]
#[doc = "    \"runtime-parameters\","]
#[doc = "    \"source\","]
#[doc = "    \"streams\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"environment\": {"]
#[doc = "      \"description\": \"The environment for the node.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "        },"]
#[doc = "        \"environment-variables\": {"]
#[doc = "          \"description\": \"The environment variables for the node.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"Environment Variable\","]
#[doc = "            \"description\": \"An environment variable for the node.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"name\","]
#[doc = "              \"source\","]
#[doc = "              \"value\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"name\": {"]
#[doc = "                \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "              },"]
#[doc = "              \"source\": {"]
#[doc = "                \"$ref\": \"#/$defs/Source\""]
#[doc = "              },"]
#[doc = "              \"value\": {"]
#[doc = "                \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"parent\": {"]
#[doc = "          \"description\": \"The parent node of this node.  This is used to construct inherited environments.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"umode\": {"]
#[doc = "          \"description\": \"The user mode for the node.  This is used to set the user and group for the node.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"event-listeners\": {"]
#[doc = "      \"title\": \"Event Listeners\","]
#[doc = "      \"description\": \"A list of event listeners that are triggered by the node.  This is a named group of event brokers.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Event Listener\","]
#[doc = "        \"description\": \"An event listener that is triggered by the node.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"actions\","]
#[doc = "          \"name\","]
#[doc = "          \"source\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"actions\": {"]
#[doc = "            \"$ref\": \"#/$defs/OrderedActions\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"The event index that triggers this action list.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exit-actions\": {"]
#[doc = "      \"title\": \"Exit Actions\","]
#[doc = "      \"description\": \"A list of actions that are triggered when the node exits.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Exit Action\","]
#[doc = "        \"description\": \"An action that is triggered when the node exits.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"actions\","]
#[doc = "          \"source\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"actions\": {"]
#[doc = "            \"$ref\": \"#/$defs/OrderedActions\""]
#[doc = "          },"]
#[doc = "          \"code-end\": {"]
#[doc = "            \"description\": \"Exit code range end (inclusive) for triggering this action list.  Do not set for no upper bound.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"code-start\": {"]
#[doc = "            \"description\": \"Exit code range start (inclusive) for triggering this action list.  Do not set for no lower bound.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initial-parameters\": {"]
#[doc = "      \"title\": \"Initial Parameters\","]
#[doc = "      \"description\": \"The compile-time parameters for the node.  These parameters are static and help initialize the module.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-zA-Z_][a-zA-Z0-9_]*$\": {"]
#[doc = "          \"title\": \"Parameter\","]
#[doc = "          \"description\": \"A compile-time parameter for the node.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"number\","]
#[doc = "                \"boolean\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"module\": {"]
#[doc = "      \"description\": \"The build implementation for the node.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the node.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"runtime-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"streams\": {"]
#[doc = "      \"description\": \"The streams for the node.  This shows how to link this node to other nodes.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Stream\","]
#[doc = "        \"description\": \"A stream that connects this node to another node.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"fd\","]
#[doc = "          \"mode\","]
#[doc = "          \"source\","]
#[doc = "          \"to-fd\","]
#[doc = "          \"to-node\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"fd\": {"]
#[doc = "            \"description\": \"The FD of the stream.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"mode\": {"]
#[doc = "            \"description\": \"The type of the stream.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"input\","]
#[doc = "              \"output\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"The name of the stream.  Used only for debugging.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          },"]
#[doc = "          \"to-fd\": {"]
#[doc = "            \"description\": \"The FD of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"to-node\": {"]
#[doc = "            \"description\": \"The name of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Node {
    pub environment: NodeEnvironment,
    #[doc = "A list of event listeners that are triggered by the node.  This is a named group of event brokers."]
    #[serde(rename = "event-listeners")]
    pub event_listeners: ::std::vec::Vec<EventListener>,
    #[doc = "A list of actions that are triggered when the node exits."]
    #[serde(rename = "exit-actions")]
    pub exit_actions: ::std::vec::Vec<ExitAction>,
    #[doc = "The compile-time parameters for the node.  These parameters are static and help initialize the module."]
    #[serde(rename = "initial-parameters")]
    pub initial_parameters: ::std::collections::HashMap<NodeInitialParametersKey, Parameter>,
    #[doc = "The build implementation for the node."]
    pub module: ::std::string::String,
    #[doc = "The name of the node."]
    pub name: ::std::string::String,
    #[serde(rename = "runtime-parameters")]
    pub runtime_parameters: NamedParameters,
    pub source: Source,
    #[doc = "The streams for the node.  This shows how to link this node to other nodes."]
    pub streams: ::std::vec::Vec<Stream>,
}
impl ::std::convert::From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}
impl Node {
    pub fn builder() -> builder::Node {
        Default::default()
    }
}
#[doc = "The environment for the node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The environment for the node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "    },"]
#[doc = "    \"environment-variables\": {"]
#[doc = "      \"description\": \"The environment variables for the node.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Environment Variable\","]
#[doc = "        \"description\": \"An environment variable for the node.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"name\","]
#[doc = "          \"source\","]
#[doc = "          \"value\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          },"]
#[doc = "          \"value\": {"]
#[doc = "            \"$ref\": \"#/$defs/ComputedValue\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"description\": \"The parent node of this node.  This is used to construct inherited environments.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"umode\": {"]
#[doc = "      \"description\": \"The user mode for the node.  This is used to set the user and group for the node.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NodeEnvironment {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<ComputedValue>,
    #[doc = "The environment variables for the node."]
    #[serde(
        rename = "environment-variables",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub environment_variables: ::std::vec::Vec<EnvironmentVariable>,
    #[doc = "The parent node of this node.  This is used to construct inherited environments."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<::std::string::String>,
    pub source: Source,
    #[doc = "The user mode for the node.  This is used to set the user and group for the node."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub umode: ::std::option::Option<i64>,
}
impl ::std::convert::From<&NodeEnvironment> for NodeEnvironment {
    fn from(value: &NodeEnvironment) -> Self {
        value.clone()
    }
}
impl NodeEnvironment {
    pub fn builder() -> builder::NodeEnvironment {
        Default::default()
    }
}
#[doc = "`NodeInitialParametersKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_][a-zA-Z0-9_]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NodeInitialParametersKey(::std::string::String);
impl ::std::ops::Deref for NodeInitialParametersKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NodeInitialParametersKey> for ::std::string::String {
    fn from(value: NodeInitialParametersKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NodeInitialParametersKey> for NodeInitialParametersKey {
    fn from(value: &NodeInitialParametersKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NodeInitialParametersKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap()
            });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z_][a-zA-Z0-9_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NodeInitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NodeInitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NodeInitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NodeInitialParametersKey {
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
#[doc = "A string value that is the string representation of a number."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Number to String Value\","]
#[doc = "  \"description\": \"A string value that is the string representation of a number.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"number-to-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$comment\": \"We may want to expand the format into an explicit definition so it does not require additional parsing.  At the moment, this is the printf format string.\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NumberToStringValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ComputedNumberValue,
}
impl ::std::convert::From<&NumberToStringValue> for NumberToStringValue {
    fn from(value: &NumberToStringValue) -> Self {
        value.clone()
    }
}
impl NumberToStringValue {
    pub fn builder() -> builder::NumberToStringValue {
        Default::default()
    }
}
#[doc = "A list of actions that can be triggered by a node.  This is a named group of event brokers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ordered Actions\","]
#[doc = "  \"description\": \"A list of actions that can be triggered by a node.  This is a named group of event brokers.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/Action\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OrderedActions(pub ::std::vec::Vec<Action>);
impl ::std::ops::Deref for OrderedActions {
    type Target = ::std::vec::Vec<Action>;
    fn deref(&self) -> &::std::vec::Vec<Action> {
        &self.0
    }
}
impl ::std::convert::From<OrderedActions> for ::std::vec::Vec<Action> {
    fn from(value: OrderedActions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OrderedActions> for OrderedActions {
    fn from(value: &OrderedActions) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Action>> for OrderedActions {
    fn from(value: ::std::vec::Vec<Action>) -> Self {
        Self(value)
    }
}
#[doc = "A compile-time parameter for the node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Parameter\","]
#[doc = "  \"description\": \"A compile-time parameter for the node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"boolean\","]
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
pub struct Parameter {
    pub source: Source,
    pub value: ParameterValue,
}
impl ::std::convert::From<&Parameter> for Parameter {
    fn from(value: &Parameter) -> Self {
        value.clone()
    }
}
impl Parameter {
    pub fn builder() -> builder::Parameter {
        Default::default()
    }
}
#[doc = "`ParameterValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
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
pub enum ParameterValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
}
impl ::std::convert::From<&Self> for ParameterValue {
    fn from(value: &ParameterValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for ParameterValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for ParameterValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "The source from the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Source\","]
#[doc = "  \"description\": \"The source from the script.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"column\","]
#[doc = "    \"file\","]
#[doc = "    \"line\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"column\": {"]
#[doc = "      \"description\": \"The column number where the node is defined.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"The file where the node is defined.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"line\": {"]
#[doc = "      \"description\": \"The line number where the node is defined.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Source {
    #[doc = "The column number where the node is defined."]
    pub column: i64,
    #[doc = "The file where the node is defined."]
    pub file: ::std::string::String,
    #[doc = "The line number where the node is defined."]
    pub line: i64,
}
impl ::std::convert::From<&Source> for Source {
    fn from(value: &Source) -> Self {
        value.clone()
    }
}
impl Source {
    pub fn builder() -> builder::Source {
        Default::default()
    }
}
#[doc = "A stream that connects this node to another node."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Stream\","]
#[doc = "  \"description\": \"A stream that connects this node to another node.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fd\","]
#[doc = "    \"mode\","]
#[doc = "    \"source\","]
#[doc = "    \"to-fd\","]
#[doc = "    \"to-node\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fd\": {"]
#[doc = "      \"description\": \"The FD of the stream.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mode\": {"]
#[doc = "      \"description\": \"The type of the stream.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"input\","]
#[doc = "        \"output\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the stream.  Used only for debugging.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"to-fd\": {"]
#[doc = "      \"description\": \"The FD of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"to-node\": {"]
#[doc = "      \"description\": \"The name of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    #[doc = "The FD of the stream."]
    pub fd: i64,
    #[doc = "The type of the stream."]
    pub mode: StreamMode,
    #[doc = "The name of the stream.  Used only for debugging."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub source: Source,
    #[doc = "The FD of the node that this stream connects to.  This is used to link the node to other nodes."]
    #[serde(rename = "to-fd")]
    pub to_fd: i64,
    #[doc = "The name of the node that this stream connects to.  This is used to link the node to other nodes."]
    #[serde(rename = "to-node")]
    pub to_node: ::std::string::String,
}
impl ::std::convert::From<&Stream> for Stream {
    fn from(value: &Stream) -> Self {
        value.clone()
    }
}
impl Stream {
    pub fn builder() -> builder::Stream {
        Default::default()
    }
}
#[doc = "The type of the stream."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of the stream.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"input\","]
#[doc = "    \"output\""]
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
pub enum StreamMode {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
}
impl ::std::convert::From<&Self> for StreamMode {
    fn from(value: &StreamMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Input => write!(f, "input"),
            Self::Output => write!(f, "output"),
        }
    }
}
impl ::std::str::FromStr for StreamMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input" => Ok(Self::Input),
            "output" => Ok(Self::Output),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A boolean map value that is the union of multiple boolean map values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Union Boolean Map Value\","]
#[doc = "  \"description\": \"A boolean map value that is the union of multiple boolean map values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"union-boolean-map\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"The values to union.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/ComputedBooleanMapValue\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnionBooleanMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The values to union."]
    pub values: ::std::vec::Vec<ComputedBooleanMapValue>,
}
impl ::std::convert::From<&UnionBooleanMapValue> for UnionBooleanMapValue {
    fn from(value: &UnionBooleanMapValue) -> Self {
        value.clone()
    }
}
impl UnionBooleanMapValue {
    pub fn builder() -> builder::UnionBooleanMapValue {
        Default::default()
    }
}
#[doc = "A number map value that is the union of multiple number map values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Union Number Map Value\","]
#[doc = "  \"description\": \"A number map value that is the union of multiple number map values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"union-number-map\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"The values to union.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/ComputedNumberMapValue\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnionNumberMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The values to union."]
    pub values: ::std::vec::Vec<ComputedNumberMapValue>,
}
impl ::std::convert::From<&UnionNumberMapValue> for UnionNumberMapValue {
    fn from(value: &UnionNumberMapValue) -> Self {
        value.clone()
    }
}
impl UnionNumberMapValue {
    pub fn builder() -> builder::UnionNumberMapValue {
        Default::default()
    }
}
#[doc = "A string map value that is the union of multiple string map values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Union String Map Value\","]
#[doc = "  \"description\": \"A string map value that is the union of multiple string map values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"union-string-map\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"The values to union.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/ComputedStringMapValue\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnionStringMapValue {
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    #[doc = "The values to union."]
    pub values: ::std::vec::Vec<ComputedStringMapValue>,
}
impl ::std::convert::From<&UnionStringMapValue> for UnionStringMapValue {
    fn from(value: &UnionStringMapValue) -> Self {
        value.clone()
    }
}
impl UnionStringMapValue {
    pub fn builder() -> builder::UnionStringMapValue {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Action {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        run: ::std::result::Result<super::ActionRun, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for Action {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                run: Err("no value supplied for run".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl Action {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn run<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionRun>,
            T::Error: ::std::fmt::Display,
        {
            self.run = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Action> for super::Action {
        type Error = super::error::ConversionError;
        fn try_from(value: Action) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                run: value.run?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::Action> for Action {
        fn from(value: super::Action) -> Self {
            Self {
                name: Ok(value.name),
                run: Ok(value.run),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionParameter {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::option::Option<super::ComputedValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionParameter {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Ok(Default::default()),
            }
        }
    }
    impl ActionParameter {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ComputedValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionParameter> for super::ActionParameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionParameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ActionParameter> for ActionParameter {
        fn from(value: super::ActionParameter) -> Self {
            Self {
                name: Ok(value.name),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ArithmeticValue {
        operation: ::std::result::Result<super::ArithmeticValueOperation, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for ArithmeticValue {
        fn default() -> Self {
            Self {
                operation: Err("no value supplied for operation".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ArithmeticValue {
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ArithmeticValueOperation>,
            T::Error: ::std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ArithmeticValue> for super::ArithmeticValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ArithmeticValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                operation: value.operation?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ArithmeticValue> for ArithmeticValue {
        fn from(value: super::ArithmeticValue) -> Self {
            Self {
                operation: Ok(value.operation),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BooleanToStringValue {
        false_string: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        true_string: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedBooleanValue, ::std::string::String>,
    }
    impl ::std::default::Default for BooleanToStringValue {
        fn default() -> Self {
            Self {
                false_string: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                true_string: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl BooleanToStringValue {
        pub fn false_string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.false_string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for false_string: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn true_string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.true_string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for true_string: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedBooleanValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BooleanToStringValue> for super::BooleanToStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BooleanToStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                false_string: value.false_string?,
                source: value.source?,
                true_string: value.true_string?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::BooleanToStringValue> for BooleanToStringValue {
        fn from(value: super::BooleanToStringValue) -> Self {
            Self {
                false_string: Ok(value.false_string),
                source: Ok(value.source),
                true_string: Ok(value.true_string),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConcatenatedStringValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedStringListValue, ::std::string::String>,
    }
    impl ::std::default::Default for ConcatenatedStringValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConcatenatedStringValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConcatenatedStringValue> for super::ConcatenatedStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConcatenatedStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConcatenatedStringValue> for ConcatenatedStringValue {
        fn from(value: super::ConcatenatedStringValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanListValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantBooleanListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantBooleanListValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanListValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConstantBooleanListValueValueItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantBooleanListValue> for super::ConstantBooleanListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantBooleanListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanListValue> for ConstantBooleanListValue {
        fn from(value: super::ConstantBooleanListValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::collections::HashMap<
                super::ConstantBooleanMapValueValueKey,
                super::ComputedBooleanValue,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantBooleanMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ConstantBooleanMapValueValueKey,
                    super::ComputedBooleanValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantBooleanMapValue> for super::ConstantBooleanMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantBooleanMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanMapValue> for ConstantBooleanMapValue {
        fn from(value: super::ConstantBooleanMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantBooleanValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantBooleanValue> for super::ConstantBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanValue> for ConstantBooleanValue {
        fn from(value: super::ConstantBooleanValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberListValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantNumberListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantNumberListValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberListValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConstantNumberListValueValueItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantNumberListValue> for super::ConstantNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberListValue> for ConstantNumberListValue {
        fn from(value: super::ConstantNumberListValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::collections::HashMap<
                super::ConstantNumberMapValueValueKey,
                super::ComputedNumberValue,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantNumberMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ConstantNumberMapValueValueKey,
                    super::ComputedNumberValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantNumberMapValue> for super::ConstantNumberMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantNumberMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberMapValue> for ConstantNumberMapValue {
        fn from(value: super::ConstantNumberMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantNumberValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantNumberValue> for super::ConstantNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberValue> for ConstantNumberValue {
        fn from(value: super::ConstantNumberValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringListValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantStringListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantStringListValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringListValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConstantStringListValueValueItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantStringListValue> for super::ConstantStringListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantStringListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringListValue> for ConstantStringListValue {
        fn from(value: super::ConstantStringListValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::collections::HashMap<
                super::ConstantStringMapValueValueKey,
                super::ComputedStringValue,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantStringMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ConstantStringMapValueValueKey,
                    super::ComputedStringValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantStringMapValue> for super::ConstantStringMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantStringMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringMapValue> for ConstantStringMapValue {
        fn from(value: super::ConstantStringMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantStringValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstantStringValue> for super::ConstantStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstantStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringValue> for ConstantStringValue {
        fn from(value: super::ConstantStringValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnvironmentVariable {
        name: ::std::result::Result<super::ComputedValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedValue, ::std::string::String>,
    }
    impl ::std::default::Default for EnvironmentVariable {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl EnvironmentVariable {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedValue>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedValue>,
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
                name: value.name?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::EnvironmentVariable> for EnvironmentVariable {
        fn from(value: super::EnvironmentVariable) -> Self {
            Self {
                name: Ok(value.name),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventListener {
        actions: ::std::result::Result<super::OrderedActions, ::std::string::String>,
        name: ::std::result::Result<i64, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for EventListener {
        fn default() -> Self {
            Self {
                actions: Err("no value supplied for actions".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl EventListener {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrderedActions>,
            T::Error: ::std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventListener> for super::EventListener {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventListener,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actions: value.actions?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::EventListener> for EventListener {
        fn from(value: super::EventListener) -> Self {
            Self {
                actions: Ok(value.actions),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventName {
        name: ::std::result::Result<i64, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EventName {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl EventName {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventName> for super::EventName {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventName,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                source: value.source?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::EventName> for EventName {
        fn from(value: super::EventName) -> Self {
            Self {
                name: Ok(value.name),
                source: Ok(value.source),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExitAction {
        actions: ::std::result::Result<super::OrderedActions, ::std::string::String>,
        code_end: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        code_start: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ExitAction {
        fn default() -> Self {
            Self {
                actions: Err("no value supplied for actions".to_string()),
                code_end: Ok(Default::default()),
                code_start: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl ExitAction {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrderedActions>,
            T::Error: ::std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {}", e));
            self
        }
        pub fn code_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_end: {}", e));
            self
        }
        pub fn code_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_start: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ExitAction> for super::ExitAction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExitAction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actions: value.actions?,
                code_end: value.code_end?,
                code_start: value.code_start?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ExitAction> for ExitAction {
        fn from(value: super::ExitAction) -> Self {
            Self {
                actions: Ok(value.actions),
                code_end: Ok(value.code_end),
                code_start: Ok(value.code_start),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExitBehavior {
        behavior: ::std::result::Result<super::ExitBehaviorBehavior, ::std::string::String>,
        code_end: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        code_start: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ExitBehavior {
        fn default() -> Self {
            Self {
                behavior: Err("no value supplied for behavior".to_string()),
                code_end: Ok(Default::default()),
                code_start: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl ExitBehavior {
        pub fn behavior<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExitBehaviorBehavior>,
            T::Error: ::std::fmt::Display,
        {
            self.behavior = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for behavior: {}", e));
            self
        }
        pub fn code_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_end: {}", e));
            self
        }
        pub fn code_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_start: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ExitBehavior> for super::ExitBehavior {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExitBehavior,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                behavior: value.behavior?,
                code_end: value.code_end?,
                code_start: value.code_start?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ExitBehavior> for ExitBehavior {
        fn from(value: super::ExitBehavior) -> Self {
            Self {
                behavior: Ok(value.behavior),
                code_end: Ok(value.code_end),
                code_start: Ok(value.code_start),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToStringValue {
        separator: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedStringListValue, ::std::string::String>,
    }
    impl ::std::default::Default for ListToStringValue {
        fn default() -> Self {
            Self {
                separator: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ListToStringValue {
        pub fn separator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.separator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for separator: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToStringValue> for super::ListToStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                separator: value.separator?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ListToStringValue> for ListToStringValue {
        fn from(value: super::ListToStringValue) -> Self {
            Self {
                separator: Ok(value.separator),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LogicalBooleanValue {
        operation:
            ::std::result::Result<super::LogicalBooleanValueOperation, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedBooleanListValue, ::std::string::String>,
    }
    impl ::std::default::Default for LogicalBooleanValue {
        fn default() -> Self {
            Self {
                operation: Err("no value supplied for operation".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl LogicalBooleanValue {
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LogicalBooleanValueOperation>,
            T::Error: ::std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedBooleanListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LogicalBooleanValue> for super::LogicalBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LogicalBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                operation: value.operation?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::LogicalBooleanValue> for LogicalBooleanValue {
        fn from(value: super::LogicalBooleanValue) -> Self {
            Self {
                operation: Ok(value.operation),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanListValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanListValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupBooleanListValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupBooleanListValue> for super::LookupBooleanListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanListValue> for LookupBooleanListValue {
        fn from(value: super::LookupBooleanListValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanMapValue {
        name: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        node: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanMapValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupBooleanMapValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupBooleanMapValue> for super::LookupBooleanMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanMapValue> for LookupBooleanMapValue {
        fn from(value: super::LookupBooleanMapValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupBooleanValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupBooleanValue> for super::LookupBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanValue> for LookupBooleanValue {
        fn from(value: super::LookupBooleanValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupEnvStringValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupEnvStringValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupEnvStringValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupEnvStringValue> for super::LookupEnvStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupEnvStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupEnvStringValue> for LookupEnvStringValue {
        fn from(value: super::LookupEnvStringValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberListValue {
        name: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        node: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberListValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupNumberListValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupNumberListValue> for super::LookupNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberListValue> for LookupNumberListValue {
        fn from(value: super::LookupNumberListValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberMapValue {
        name: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        node: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberMapValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupNumberMapValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupNumberMapValue> for super::LookupNumberMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberMapValue> for LookupNumberMapValue {
        fn from(value: super::LookupNumberMapValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupNumberValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupNumberValue> for super::LookupNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberValue> for LookupNumberValue {
        fn from(value: super::LookupNumberValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStateStringValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStateStringValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupStateStringValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupStateStringValue> for super::LookupStateStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStateStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStateStringValue> for LookupStateStringValue {
        fn from(value: super::LookupStateStringValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStringListValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStringListValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupStringListValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupStringListValue> for super::LookupStringListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringListValue> for LookupStringListValue {
        fn from(value: super::LookupStringListValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStringMapValue {
        name: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        node: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStringMapValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupStringMapValue {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LookupStringMapValue> for super::LookupStringMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringMapValue> for LookupStringMapValue {
        fn from(value: super::LookupStringMapValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapToStringValue {
        item_separator: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        key_separator: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedStringMapValue, ::std::string::String>,
    }
    impl ::std::default::Default for MapToStringValue {
        fn default() -> Self {
            Self {
                item_separator: Ok(Default::default()),
                key_separator: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl MapToStringValue {
        pub fn item_separator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.item_separator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_separator: {}", e));
            self
        }
        pub fn key_separator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.key_separator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key_separator: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringMapValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MapToStringValue> for super::MapToStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapToStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                item_separator: value.item_separator?,
                key_separator: value.key_separator?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MapToStringValue> for MapToStringValue {
        fn from(value: super::MapToStringValue) -> Self {
            Self {
                item_separator: Ok(value.item_separator),
                key_separator: Ok(value.key_separator),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NativeShellAstSchema {
        default_start:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        events: ::std::result::Result<::std::vec::Vec<super::EventName>, ::std::string::String>,
        nodes: ::std::result::Result<::std::vec::Vec<super::Node>, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for NativeShellAstSchema {
        fn default() -> Self {
            Self {
                default_start: Err("no value supplied for default_start".to_string()),
                events: Err("no value supplied for events".to_string()),
                nodes: Err("no value supplied for nodes".to_string()),
                source: Err("no value supplied for source".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl NativeShellAstSchema {
        pub fn default_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_start: {}", e));
            self
        }
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EventName>>,
            T::Error: ::std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {}", e));
            self
        }
        pub fn nodes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Node>>,
            T::Error: ::std::fmt::Display,
        {
            self.nodes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nodes: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NativeShellAstSchema> for super::NativeShellAstSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NativeShellAstSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_start: value.default_start?,
                events: value.events?,
                nodes: value.nodes?,
                source: value.source?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::NativeShellAstSchema> for NativeShellAstSchema {
        fn from(value: super::NativeShellAstSchema) -> Self {
            Self {
                default_start: Ok(value.default_start),
                events: Ok(value.events),
                nodes: Ok(value.nodes),
                source: Ok(value.source),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Node {
        environment: ::std::result::Result<super::NodeEnvironment, ::std::string::String>,
        event_listeners:
            ::std::result::Result<::std::vec::Vec<super::EventListener>, ::std::string::String>,
        exit_actions:
            ::std::result::Result<::std::vec::Vec<super::ExitAction>, ::std::string::String>,
        initial_parameters: ::std::result::Result<
            ::std::collections::HashMap<super::NodeInitialParametersKey, super::Parameter>,
            ::std::string::String,
        >,
        module: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        runtime_parameters: ::std::result::Result<super::NamedParameters, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        streams: ::std::result::Result<::std::vec::Vec<super::Stream>, ::std::string::String>,
    }
    impl ::std::default::Default for Node {
        fn default() -> Self {
            Self {
                environment: Err("no value supplied for environment".to_string()),
                event_listeners: Err("no value supplied for event_listeners".to_string()),
                exit_actions: Err("no value supplied for exit_actions".to_string()),
                initial_parameters: Err("no value supplied for initial_parameters".to_string()),
                module: Err("no value supplied for module".to_string()),
                name: Err("no value supplied for name".to_string()),
                runtime_parameters: Err("no value supplied for runtime_parameters".to_string()),
                source: Err("no value supplied for source".to_string()),
                streams: Err("no value supplied for streams".to_string()),
            }
        }
    }
    impl Node {
        pub fn environment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NodeEnvironment>,
            T::Error: ::std::fmt::Display,
        {
            self.environment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for environment: {}", e));
            self
        }
        pub fn event_listeners<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EventListener>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_listeners = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_listeners: {}", e));
            self
        }
        pub fn exit_actions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExitAction>>,
            T::Error: ::std::fmt::Display,
        {
            self.exit_actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_actions: {}", e));
            self
        }
        pub fn initial_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::NodeInitialParametersKey, super::Parameter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.initial_parameters = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for initial_parameters: {}",
                    e
                )
            });
            self
        }
        pub fn module<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.module = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for module: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn runtime_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NamedParameters>,
            T::Error: ::std::fmt::Display,
        {
            self.runtime_parameters = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for runtime_parameters: {}",
                    e
                )
            });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn streams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Stream>>,
            T::Error: ::std::fmt::Display,
        {
            self.streams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for streams: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Node> for super::Node {
        type Error = super::error::ConversionError;
        fn try_from(value: Node) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                environment: value.environment?,
                event_listeners: value.event_listeners?,
                exit_actions: value.exit_actions?,
                initial_parameters: value.initial_parameters?,
                module: value.module?,
                name: value.name?,
                runtime_parameters: value.runtime_parameters?,
                source: value.source?,
                streams: value.streams?,
            })
        }
    }
    impl ::std::convert::From<super::Node> for Node {
        fn from(value: super::Node) -> Self {
            Self {
                environment: Ok(value.environment),
                event_listeners: Ok(value.event_listeners),
                exit_actions: Ok(value.exit_actions),
                initial_parameters: Ok(value.initial_parameters),
                module: Ok(value.module),
                name: Ok(value.name),
                runtime_parameters: Ok(value.runtime_parameters),
                source: Ok(value.source),
                streams: Ok(value.streams),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NodeEnvironment {
        cwd: ::std::result::Result<
            ::std::option::Option<super::ComputedValue>,
            ::std::string::String,
        >,
        environment_variables: ::std::result::Result<
            ::std::vec::Vec<super::EnvironmentVariable>,
            ::std::string::String,
        >,
        parent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        umode: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for NodeEnvironment {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                environment_variables: Ok(Default::default()),
                parent: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                umode: Ok(Default::default()),
            }
        }
    }
    impl NodeEnvironment {
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ComputedValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {}", e));
            self
        }
        pub fn environment_variables<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EnvironmentVariable>>,
            T::Error: ::std::fmt::Display,
        {
            self.environment_variables = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for environment_variables: {}",
                    e
                )
            });
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn umode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.umode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for umode: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NodeEnvironment> for super::NodeEnvironment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NodeEnvironment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                environment_variables: value.environment_variables?,
                parent: value.parent?,
                source: value.source?,
                umode: value.umode?,
            })
        }
    }
    impl ::std::convert::From<super::NodeEnvironment> for NodeEnvironment {
        fn from(value: super::NodeEnvironment) -> Self {
            Self {
                cwd: Ok(value.cwd),
                environment_variables: Ok(value.environment_variables),
                parent: Ok(value.parent),
                source: Ok(value.source),
                umode: Ok(value.umode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NumberToStringValue {
        format: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberValue, ::std::string::String>,
    }
    impl ::std::default::Default for NumberToStringValue {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl NumberToStringValue {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedNumberValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NumberToStringValue> for super::NumberToStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NumberToStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::NumberToStringValue> for NumberToStringValue {
        fn from(value: super::NumberToStringValue) -> Self {
            Self {
                format: Ok(value.format),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Parameter {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ParameterValue, ::std::string::String>,
    }
    impl ::std::default::Default for Parameter {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Parameter {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
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
    impl ::std::convert::TryFrom<Parameter> for super::Parameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Parameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Parameter> for Parameter {
        fn from(value: super::Parameter) -> Self {
            Self {
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Source {
        column: ::std::result::Result<i64, ::std::string::String>,
        file: ::std::result::Result<::std::string::String, ::std::string::String>,
        line: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Source {
        fn default() -> Self {
            Self {
                column: Err("no value supplied for column".to_string()),
                file: Err("no value supplied for file".to_string()),
                line: Err("no value supplied for line".to_string()),
            }
        }
    }
    impl Source {
        pub fn column<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.column = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for column: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Source> for super::Source {
        type Error = super::error::ConversionError;
        fn try_from(value: Source) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                column: value.column?,
                file: value.file?,
                line: value.line?,
            })
        }
    }
    impl ::std::convert::From<super::Source> for Source {
        fn from(value: super::Source) -> Self {
            Self {
                column: Ok(value.column),
                file: Ok(value.file),
                line: Ok(value.line),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Stream {
        fd: ::std::result::Result<i64, ::std::string::String>,
        mode: ::std::result::Result<super::StreamMode, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        to_fd: ::std::result::Result<i64, ::std::string::String>,
        to_node: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Stream {
        fn default() -> Self {
            Self {
                fd: Err("no value supplied for fd".to_string()),
                mode: Err("no value supplied for mode".to_string()),
                name: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                to_fd: Err("no value supplied for to_fd".to_string()),
                to_node: Err("no value supplied for to_node".to_string()),
            }
        }
    }
    impl Stream {
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fd: {}", e));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn to_fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.to_fd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_fd: {}", e));
            self
        }
        pub fn to_node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.to_node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_node: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Stream> for super::Stream {
        type Error = super::error::ConversionError;
        fn try_from(value: Stream) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fd: value.fd?,
                mode: value.mode?,
                name: value.name?,
                source: value.source?,
                to_fd: value.to_fd?,
                to_node: value.to_node?,
            })
        }
    }
    impl ::std::convert::From<super::Stream> for Stream {
        fn from(value: super::Stream) -> Self {
            Self {
                fd: Ok(value.fd),
                mode: Ok(value.mode),
                name: Ok(value.name),
                source: Ok(value.source),
                to_fd: Ok(value.to_fd),
                to_node: Ok(value.to_node),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionBooleanMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedBooleanMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionBooleanMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionBooleanMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ComputedBooleanMapValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UnionBooleanMapValue> for super::UnionBooleanMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnionBooleanMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionBooleanMapValue> for UnionBooleanMapValue {
        fn from(value: super::UnionBooleanMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionNumberMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedNumberMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionNumberMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionNumberMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ComputedNumberMapValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UnionNumberMapValue> for super::UnionNumberMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnionNumberMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionNumberMapValue> for UnionNumberMapValue {
        fn from(value: super::UnionNumberMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionStringMapValue {
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedStringMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionStringMapValue {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionStringMapValue {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Source>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ComputedStringMapValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UnionStringMapValue> for super::UnionStringMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnionStringMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionStringMapValue> for UnionStringMapValue {
        fn from(value: super::UnionStringMapValue) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
                values: Ok(value.values),
            }
        }
    }
}
