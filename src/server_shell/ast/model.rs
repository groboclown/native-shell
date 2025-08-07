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
#[doc = "            \"on-failure\","]
#[doc = "            \"on-success\","]
#[doc = "            \"parameters\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"handler\": {"]
#[doc = "              \"description\": \"The name of the handler to run on the current node until the handler completes.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"on-failure\": {"]
#[doc = "              \"$ref\": \"#/$defs/ActionEndBehavior\""]
#[doc = "            },"]
#[doc = "            \"on-success\": {"]
#[doc = "              \"$ref\": \"#/$defs/ActionEndBehavior\""]
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
#[doc = "          \"title\": \"Spawn Node\","]
#[doc = "          \"description\": \"Requests the parallel execution of a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"node\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"node\": {"]
#[doc = "              \"description\": \"The node ID to run.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"spawn-node\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Ensure Node Started At Least Once\","]
#[doc = "          \"description\": \"If the node has never started, then start it through the action reference.  Otherwise, do nothing.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"node\","]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"node\": {"]
#[doc = "              \"description\": \"The node ID to run.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"ensure-node-started-at-least-once\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Wait For Node\","]
#[doc = "          \"description\": \"Wait for a node to finish executing.  This will block until the node's execution exits.  If it has already finished exiting, this will continue without waiting.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"for-exit\","]
#[doc = "            \"if-not-started\","]
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
#[doc = "                    \"$ref\": \"#/$defs/ActionEndBehavior\""]
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
#[doc = "            \"if-not-started\": {"]
#[doc = "              \"$ref\": \"#/$defs/ActionEndBehavior\""]
#[doc = "            },"]
#[doc = "            \"node\": {"]
#[doc = "              \"description\": \"The node ID to run.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"wait-for-node\""]
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
pub enum ActionEndBehavior {
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "skip-next")]
    SkipNext,
    #[serde(rename = "skip-all")]
    SkipAll,
    #[serde(rename = "abort-script")]
    AbortScript,
}
impl ::std::convert::From<&Self> for ActionEndBehavior {
    fn from(value: &ActionEndBehavior) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ActionEndBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Run => write!(f, "run"),
            Self::SkipNext => write!(f, "skip-next"),
            Self::SkipAll => write!(f, "skip-all"),
            Self::AbortScript => write!(f, "abort-script"),
        }
    }
}
impl ::std::str::FromStr for ActionEndBehavior {
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
impl ::std::convert::TryFrom<&str> for ActionEndBehavior {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActionEndBehavior {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActionEndBehavior {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "    \"source\","]
#[doc = "    \"value\""]
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
    pub value: ComputedValue,
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
#[doc = "        \"on-failure\","]
#[doc = "        \"on-success\","]
#[doc = "        \"parameters\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"handler\": {"]
#[doc = "          \"description\": \"The name of the handler to run on the current node until the handler completes.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"on-failure\": {"]
#[doc = "          \"$ref\": \"#/$defs/ActionEndBehavior\""]
#[doc = "        },"]
#[doc = "        \"on-success\": {"]
#[doc = "          \"$ref\": \"#/$defs/ActionEndBehavior\""]
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
#[doc = "      \"title\": \"Spawn Node\","]
#[doc = "      \"description\": \"Requests the parallel execution of a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"node\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"node\": {"]
#[doc = "          \"description\": \"The node ID to run.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"spawn-node\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Ensure Node Started At Least Once\","]
#[doc = "      \"description\": \"If the node has never started, then start it through the action reference.  Otherwise, do nothing.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"node\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"node\": {"]
#[doc = "          \"description\": \"The node ID to run.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"ensure-node-started-at-least-once\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Wait For Node\","]
#[doc = "      \"description\": \"Wait for a node to finish executing.  This will block until the node's execution exits.  If it has already finished exiting, this will continue without waiting.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"for-exit\","]
#[doc = "        \"if-not-started\","]
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
#[doc = "                \"$ref\": \"#/$defs/ActionEndBehavior\""]
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
#[doc = "        \"if-not-started\": {"]
#[doc = "          \"$ref\": \"#/$defs/ActionEndBehavior\""]
#[doc = "        },"]
#[doc = "        \"node\": {"]
#[doc = "          \"description\": \"The node ID to run.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"wait-for-node\""]
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
        #[doc = "The name of the handler to run on the current node until the handler completes."]
        handler: ::std::string::String,
        #[serde(rename = "on-failure")]
        on_failure: ActionEndBehavior,
        #[serde(rename = "on-success")]
        on_success: ActionEndBehavior,
        parameters: NamedParameters,
        source: Source,
    },
    #[doc = "Spawn Node\n\nRequests the parallel execution of a node, either start it or restart it.  If it's currently running, this will wait for it to finish and reuse its exit code."]
    #[serde(rename = "spawn-node")]
    SpawnNode {
        #[doc = "The node ID to run."]
        node: ::std::string::String,
        source: Source,
    },
    #[doc = "Ensure Node Started At Least Once\n\nIf the node has never started, then start it through the action reference.  Otherwise, do nothing."]
    #[serde(rename = "ensure-node-started-at-least-once")]
    EnsureNodeStartedAtLeastOnce {
        #[doc = "The node ID to run."]
        node: ::std::string::String,
        source: Source,
    },
    #[doc = "Wait For Node\n\nWait for a node to finish executing.  This will block until the node's execution exits.  If it has already finished exiting, this will continue without waiting."]
    #[serde(rename = "wait-for-node")]
    WaitForNode {
        #[doc = "Behavior for the next action in the list depending on the executed node's exit code."]
        #[serde(rename = "for-exit")]
        for_exit: ::std::vec::Vec<ExitBehavior>,
        #[serde(rename = "if-not-started")]
        if_not_started: ActionEndBehavior,
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
#[doc = "      \"$ref\": \"#/$defs/RangeBooleanListValue\""]
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
    RangeBooleanListValue(RangeBooleanListValue),
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
impl ::std::convert::From<RangeBooleanListValue> for ComputedBooleanListValue {
    fn from(value: RangeBooleanListValue) -> Self {
        Self::RangeBooleanListValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/ListIndexBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapKeyBooleanValue\""]
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
    ListIndexBooleanValue(ListIndexBooleanValue),
    MapKeyBooleanValue(MapKeyBooleanValue),
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
impl ::std::convert::From<ListIndexBooleanValue> for ComputedBooleanValue {
    fn from(value: ListIndexBooleanValue) -> Self {
        Self::ListIndexBooleanValue(value)
    }
}
impl ::std::convert::From<MapKeyBooleanValue> for ComputedBooleanValue {
    fn from(value: MapKeyBooleanValue) -> Self {
        Self::MapKeyBooleanValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/RangeNumberListValue\""]
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
    RangeNumberListValue(RangeNumberListValue),
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
impl ::std::convert::From<RangeNumberListValue> for ComputedNumberListValue {
    fn from(value: RangeNumberListValue) -> Self {
        Self::RangeNumberListValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/ListIndexNumberValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapKeyNumberValue\""]
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
    ListIndexNumberValue(ListIndexNumberValue),
    MapKeyNumberValue(MapKeyNumberValue),
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
impl ::std::convert::From<ListIndexNumberValue> for ComputedNumberValue {
    fn from(value: ListIndexNumberValue) -> Self {
        Self::ListIndexNumberValue(value)
    }
}
impl ::std::convert::From<MapKeyNumberValue> for ComputedNumberValue {
    fn from(value: MapKeyNumberValue) -> Self {
        Self::MapKeyNumberValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/SplitStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RangeStringListValue\""]
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
    SplitStringValue(SplitStringValue),
    RangeStringListValue(RangeStringListValue),
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
impl ::std::convert::From<SplitStringValue> for ComputedStringListValue {
    fn from(value: SplitStringValue) -> Self {
        Self::SplitStringValue(value)
    }
}
impl ::std::convert::From<RangeStringListValue> for ComputedStringListValue {
    fn from(value: RangeStringListValue) -> Self {
        Self::RangeStringListValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/LookupStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ListIndexStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapKeyStringValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SubStringValue\""]
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
    LookupStringValue(LookupStringValue),
    ListIndexStringValue(ListIndexStringValue),
    MapKeyStringValue(MapKeyStringValue),
    SubStringValue(SubStringValue),
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
impl ::std::convert::From<LookupStringValue> for ComputedStringValue {
    fn from(value: LookupStringValue) -> Self {
        Self::LookupStringValue(value)
    }
}
impl ::std::convert::From<ListIndexStringValue> for ComputedStringValue {
    fn from(value: ListIndexStringValue) -> Self {
        Self::ListIndexStringValue(value)
    }
}
impl ::std::convert::From<MapKeyStringValue> for ComputedStringValue {
    fn from(value: MapKeyStringValue) -> Self {
        Self::MapKeyStringValue(value)
    }
}
impl ::std::convert::From<SubStringValue> for ComputedStringValue {
    fn from(value: SubStringValue) -> Self {
        Self::SubStringValue(value)
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
#[doc = "A constant boolean map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Boolean Map Value\","]
#[doc = "  \"description\": \"A constant boolean map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union.\","]
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
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
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
    pub value: ::std::collections::HashMap<
        ConstantBooleanMapValueValueKey,
        ::std::option::Option<ComputedBooleanValue>,
    >,
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
#[doc = "A constant number map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Number Map Value\","]
#[doc = "  \"description\": \"A constant number map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union.\","]
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
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
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
    pub value: ::std::collections::HashMap<
        ConstantNumberMapValueValueKey,
        ::std::option::Option<ComputedNumberValue>,
    >,
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
#[doc = "A constant string map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String Map Value\","]
#[doc = "  \"description\": \"A constant string map value.  Can include expanding a sub-map within the map.  This also includes a 'null' value for the key to allow blanking out values if used in a union.\","]
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
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
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
    pub value: ::std::collections::HashMap<
        ConstantStringMapValueValueKey,
        ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    >,
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
#[doc = "      \"description\": \"The event name that triggers this action list.\","]
#[doc = "      \"type\": \"string\""]
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
    #[doc = "The event name that triggers this action list."]
    pub name: ::std::string::String,
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
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text description of the event.  This is used for debugging and logging.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the event.  Each type has a different way to run and takes different arguments.  Note that the 'main' node's starting event will always be a signal with value '0'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"signal\","]
#[doc = "        \"message\""]
#[doc = "      ]"]
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
    pub name: ::std::string::String,
    pub source: Source,
    #[doc = "The text description of the event.  This is used for debugging and logging."]
    pub text: ::std::string::String,
    #[doc = "The type of the event.  Each type has a different way to run and takes different arguments.  Note that the 'main' node's starting event will always be a signal with value '0'."]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EventNameType>,
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
#[doc = "The type of the event.  Each type has a different way to run and takes different arguments.  Note that the 'main' node's starting event will always be a signal with value '0'."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of the event.  Each type has a different way to run and takes different arguments.  Note that the 'main' node's starting event will always be a signal with value '0'.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"signal\","]
#[doc = "    \"message\""]
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
pub enum EventNameType {
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "message")]
    Message,
}
impl ::std::convert::From<&Self> for EventNameType {
    fn from(value: &EventNameType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventNameType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Signal => write!(f, "signal"),
            Self::Message => write!(f, "message"),
        }
    }
}
impl ::std::str::FromStr for EventNameType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "signal" => Ok(Self::Signal),
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventNameType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventNameType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventNameType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "      \"$ref\": \"#/$defs/ActionEndBehavior\""]
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
    pub behavior: ActionEndBehavior,
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
#[doc = "Extracts a single indexed boolean from a string list.  A default must be given, in case the index is out of bounds."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Indexed Boolean Value\","]
#[doc = "  \"description\": \"Extracts a single indexed boolean from a string list.  A default must be given, in case the index is out of bounds.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"index\","]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"list-index-boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListIndexBooleanValue {
    pub default: ::std::boxed::Box<ComputedBooleanValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub list: ComputedBooleanListValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&ListIndexBooleanValue> for ListIndexBooleanValue {
    fn from(value: &ListIndexBooleanValue) -> Self {
        value.clone()
    }
}
impl ListIndexBooleanValue {
    pub fn builder() -> builder::ListIndexBooleanValue {
        Default::default()
    }
}
#[doc = "Extracts a single indexed number from a string list.  A default must be given, in case the index is out of bounds."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Indexed Number Value\","]
#[doc = "  \"description\": \"Extracts a single indexed number from a string list.  A default must be given, in case the index is out of bounds.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"index\","]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"list-index-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListIndexNumberValue {
    pub default: ::std::boxed::Box<ComputedNumberValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub list: ::std::boxed::Box<ComputedNumberListValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&ListIndexNumberValue> for ListIndexNumberValue {
    fn from(value: &ListIndexNumberValue) -> Self {
        value.clone()
    }
}
impl ListIndexNumberValue {
    pub fn builder() -> builder::ListIndexNumberValue {
        Default::default()
    }
}
#[doc = "Extracts a single indexed string from a string list.  A default must be given, in case the index is out of bounds."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Indexed String Value\","]
#[doc = "  \"description\": \"Extracts a single indexed string from a string list.  A default must be given, in case the index is out of bounds.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"index\","]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"list-index-string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListIndexStringValue {
    pub default: ::std::boxed::Box<ComputedStringValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub list: ComputedStringListValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&ListIndexStringValue> for ListIndexStringValue {
    fn from(value: &ListIndexStringValue) -> Self {
        value.clone()
    }
}
impl ListIndexStringValue {
    pub fn builder() -> builder::ListIndexStringValue {
        Default::default()
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
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
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
    pub name: ComputedStringValue,
    pub node: ComputedStringValue,
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
#[doc = "      \"const\": \"lookup-string-map\""]
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
#[doc = "      \"const\": \"lookup-string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LookupStringValue {
    pub name: ::std::boxed::Box<ComputedStringValue>,
    pub node: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&LookupStringValue> for LookupStringValue {
    fn from(value: &LookupStringValue) -> Self {
        value.clone()
    }
}
impl LookupStringValue {
    pub fn builder() -> builder::LookupStringValue {
        Default::default()
    }
}
#[doc = "A key's boolean value from a map.  A default must be given, in case the key is not found."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Map Keyed Boolean Value\","]
#[doc = "  \"description\": \"A key's boolean value from a map.  A default must be given, in case the key is not found.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"key\","]
#[doc = "    \"map\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanMapValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"map-key-boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapKeyBooleanValue {
    pub default: ::std::boxed::Box<ComputedBooleanValue>,
    pub key: ::std::boxed::Box<ComputedStringValue>,
    pub map: ComputedBooleanMapValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&MapKeyBooleanValue> for MapKeyBooleanValue {
    fn from(value: &MapKeyBooleanValue) -> Self {
        value.clone()
    }
}
impl MapKeyBooleanValue {
    pub fn builder() -> builder::MapKeyBooleanValue {
        Default::default()
    }
}
#[doc = "A key's number value from a map.  A default must be given, in case the key is not found."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Map Keyed Number Value\","]
#[doc = "  \"description\": \"A key's number value from a map.  A default must be given, in case the key is not found.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"key\","]
#[doc = "    \"map\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberMapValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"map-key-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapKeyNumberValue {
    pub default: ::std::boxed::Box<ComputedNumberValue>,
    pub key: ComputedStringValue,
    pub map: ComputedNumberMapValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&MapKeyNumberValue> for MapKeyNumberValue {
    fn from(value: &MapKeyNumberValue) -> Self {
        value.clone()
    }
}
impl MapKeyNumberValue {
    pub fn builder() -> builder::MapKeyNumberValue {
        Default::default()
    }
}
#[doc = "A key's string value from a map.  A default must be given, in case the key is not found."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Map Keyed String Value\","]
#[doc = "  \"description\": \"A key's string value from a map.  A default must be given, in case the key is not found.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"key\","]
#[doc = "    \"map\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringMapValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"map-key-string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapKeyStringValue {
    pub default: ::std::boxed::Box<ComputedStringValue>,
    pub key: ::std::boxed::Box<ComputedStringValue>,
    pub map: ComputedStringMapValue,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&MapKeyStringValue> for MapKeyStringValue {
    fn from(value: &MapKeyStringValue) -> Self {
        value.clone()
    }
}
impl MapKeyStringValue {
    pub fn builder() -> builder::MapKeyStringValue {
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
#[doc = "      \"source\","]
#[doc = "      \"value\""]
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
#[doc = "    \"events\","]
#[doc = "    \"name\","]
#[doc = "    \"nodes\","]
#[doc = "    \"schema-version\","]
#[doc = "    \"source\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
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
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          },"]
#[doc = "          \"text\": {"]
#[doc = "            \"description\": \"The text description of the event.  This is used for debugging and logging.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"description\": \"The type of the event.  Each type has a different way to run and takes different arguments.  Note that the 'main' node's starting event will always be a signal with value '0'.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"signal\","]
#[doc = "              \"message\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The script name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"title\": \"Node List\","]
#[doc = "      \"description\": \"The nodes in the AST.  Each node is a module with a backing builder.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Node\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"schema-version\": {"]
#[doc = "      \"description\": \"The version of the schema for this AST.  This is used to ensure compatibility with the parser and runtime.\","]
#[doc = "      \"const\": \"1.0.0\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"The version of the script.\","]
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
    #[doc = "The event names that nodes can trigger.  These construct a named group of event brokers, which are referenced by the nodes through the index.  Eventually, this may have a parameter list, but for the moment, events can carry a message and/or a code."]
    pub events: ::std::vec::Vec<EventName>,
    #[doc = "The script name."]
    pub name: NativeShellAstSchemaName,
    #[doc = "The nodes in the AST.  Each node is a module with a backing builder."]
    pub nodes: ::std::vec::Vec<Node>,
    #[doc = "The version of the schema for this AST.  This is used to ensure compatibility with the parser and runtime."]
    #[serde(rename = "schema-version")]
    pub schema_version: ::serde_json::Value,
    pub source: Source,
    #[doc = "The version of the script."]
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
#[doc = "The script name."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The script name.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NativeShellAstSchemaName(::std::string::String);
impl ::std::ops::Deref for NativeShellAstSchemaName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellAstSchemaName> for ::std::string::String {
    fn from(value: NativeShellAstSchemaName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellAstSchemaName> for NativeShellAstSchemaName {
    fn from(value: &NativeShellAstSchemaName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellAstSchemaName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellAstSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellAstSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellAstSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellAstSchemaName {
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
#[doc = "            \"description\": \"The event name that triggers this action list.\","]
#[doc = "            \"type\": \"string\""]
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
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time String\","]
#[doc = "                  \"description\": \"A constant string value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"description\": \"The constant string value.\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Number\","]
#[doc = "                  \"description\": \"A constant number value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"description\": \"The constant number value.\","]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Boolean\","]
#[doc = "                  \"description\": \"A constant boolean value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"boolean\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"description\": \"The constant boolean value.\","]
#[doc = "                      \"type\": \"boolean\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Null\","]
#[doc = "                  \"description\": \"A constant null value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"null\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time String List\","]
#[doc = "                  \"description\": \"A constant string list value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"string-list\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Number List\","]
#[doc = "                  \"description\": \"A constant number list value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"number-list\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"array\","]
#[doc = "                        \"items\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Boolean List\","]
#[doc = "                  \"description\": \"A constant boolean list value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"boolean-list\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"array\","]
#[doc = "                        \"items\": {"]
#[doc = "                          \"type\": \"boolean\""]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time String Map\","]
#[doc = "                  \"description\": \"A constant string map value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"string-map\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"patternProperties\": {"]
#[doc = "                        \".*\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Number Map\","]
#[doc = "                  \"description\": \"A constant number map value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"number-map\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"patternProperties\": {"]
#[doc = "                        \".*\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"title\": \"Compile-Time Boolean Map\","]
#[doc = "                  \"description\": \"A constant boolean map value.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"source\","]
#[doc = "                    \"type\","]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"source\": {"]
#[doc = "                      \"$ref\": \"#/$defs/Source\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"boolean-map\""]
#[doc = "                    },"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"patternProperties\": {"]
#[doc = "                        \".*\": {"]
#[doc = "                          \"type\": \"boolean\""]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                }"]
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
#[doc = "          \"mode\","]
#[doc = "          \"source\","]
#[doc = "          \"to-node\","]
#[doc = "          \"to-stream\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"fd\": {"]
#[doc = "            \"description\": \"The FD of the stream.\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"mode\": {"]
#[doc = "            \"description\": \"The type of the stream.  Even for modules where this maps to a fixed stream, this helps ensure the script's assumptions and the module align.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"input\","]
#[doc = "              \"output\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"The name of the stream, if associated with a named stream.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          },"]
#[doc = "          \"to-node\": {"]
#[doc = "            \"description\": \"The name of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"to-stream\": {"]
#[doc = "            \"description\": \"The FD or named stream of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"integer\","]
#[doc = "              \"string\""]
#[doc = "            ]"]
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
    pub value: ::std::boxed::Box<ComputedNumberValue>,
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
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time String\","]
#[doc = "          \"description\": \"A constant string value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"string\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"The constant string value.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Number\","]
#[doc = "          \"description\": \"A constant number value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"number\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"The constant number value.\","]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Boolean\","]
#[doc = "          \"description\": \"A constant boolean value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"The constant boolean value.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Null\","]
#[doc = "          \"description\": \"A constant null value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"null\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time String List\","]
#[doc = "          \"description\": \"A constant string list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"string-list\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Number List\","]
#[doc = "          \"description\": \"A constant number list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"number-list\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Boolean List\","]
#[doc = "          \"description\": \"A constant boolean list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"boolean-list\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time String Map\","]
#[doc = "          \"description\": \"A constant string map value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"string-map\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \".*\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Number Map\","]
#[doc = "          \"description\": \"A constant number map value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"number-map\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \".*\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Boolean Map\","]
#[doc = "          \"description\": \"A constant boolean map value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"source\","]
#[doc = "            \"type\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"boolean-map\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \".*\": {"]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
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
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time String\","]
#[doc = "      \"description\": \"A constant string value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"string\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The constant string value.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Number\","]
#[doc = "      \"description\": \"A constant number value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"number\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The constant number value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Boolean\","]
#[doc = "      \"description\": \"A constant boolean value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The constant boolean value.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Null\","]
#[doc = "      \"description\": \"A constant null value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"null\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time String List\","]
#[doc = "      \"description\": \"A constant string list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"string-list\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Number List\","]
#[doc = "      \"description\": \"A constant number list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"number-list\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Boolean List\","]
#[doc = "      \"description\": \"A constant boolean list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"boolean-list\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time String Map\","]
#[doc = "      \"description\": \"A constant string map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"string-map\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Number Map\","]
#[doc = "      \"description\": \"A constant number map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"number-map\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Boolean Map\","]
#[doc = "      \"description\": \"A constant boolean map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"boolean-map\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          }"]
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
pub enum ParameterValue {
    #[doc = "Compile-Time String\n\nA constant string value."]
    #[serde(rename = "string")]
    String {
        source: Source,
        #[doc = "The constant string value."]
        value: ::std::string::String,
    },
    #[doc = "Compile-Time Number\n\nA constant number value."]
    #[serde(rename = "number")]
    Number { source: Source, value: f64 },
    #[doc = "Compile-Time Boolean\n\nA constant boolean value."]
    #[serde(rename = "boolean")]
    Boolean {
        source: Source,
        #[doc = "The constant boolean value."]
        value: bool,
    },
    #[doc = "Compile-Time Null\n\nA constant null value."]
    #[serde(rename = "null")]
    Null { source: Source },
    #[doc = "Compile-Time String List\n\nA constant string list value."]
    #[serde(rename = "string-list")]
    StringList {
        source: Source,
        value: ::std::vec::Vec<::std::string::String>,
    },
    #[doc = "Compile-Time Number List\n\nA constant number list value."]
    #[serde(rename = "number-list")]
    NumberList {
        source: Source,
        value: ::std::vec::Vec<::std::vec::Vec<f64>>,
    },
    #[doc = "Compile-Time Boolean List\n\nA constant boolean list value."]
    #[serde(rename = "boolean-list")]
    BooleanList {
        source: Source,
        value: ::std::vec::Vec<::std::vec::Vec<bool>>,
    },
    #[doc = "Compile-Time String Map\n\nA constant string map value."]
    #[serde(rename = "string-map")]
    StringMap {
        source: Source,
        value: ::std::collections::HashMap<ParameterValueValueKey, ::std::string::String>,
    },
    #[doc = "Compile-Time Number Map\n\nA constant number map value."]
    #[serde(rename = "number-map")]
    NumberMap {
        source: Source,
        value: ::std::collections::HashMap<ParameterValueValueKey, f64>,
    },
    #[doc = "Compile-Time Boolean Map\n\nA constant boolean map value."]
    #[serde(rename = "boolean-map")]
    BooleanMap {
        source: Source,
        value: ::std::collections::HashMap<ParameterValueValueKey, bool>,
    },
}
impl ::std::convert::From<&Self> for ParameterValue {
    fn from(value: &ParameterValue) -> Self {
        value.clone()
    }
}
#[doc = "`ParameterValueValueKey`"]
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
pub struct ParameterValueValueKey(::std::string::String);
impl ::std::ops::Deref for ParameterValueValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParameterValueValueKey> for ::std::string::String {
    fn from(value: ParameterValueValueKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ParameterValueValueKey> for ParameterValueValueKey {
    fn from(value: &ParameterValueValueKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ParameterValueValueKey {
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
impl ::std::convert::TryFrom<&str> for ParameterValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParameterValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParameterValueValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParameterValueValueKey {
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
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range Boolean List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"range-boolean-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeBooleanListValue {
    #[serde(default)]
    pub count: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(default)]
    pub end: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    pub list: ::std::boxed::Box<ComputedBooleanListValue>,
    pub source: Source,
    #[serde(default)]
    pub start: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&RangeBooleanListValue> for RangeBooleanListValue {
    fn from(value: &RangeBooleanListValue) -> Self {
        value.clone()
    }
}
impl RangeBooleanListValue {
    pub fn builder() -> builder::RangeBooleanListValue {
        Default::default()
    }
}
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range Number List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"range-number-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeNumberListValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<ComputedNumberValue>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<ComputedNumberValue>,
    pub list: ::std::boxed::Box<ComputedNumberListValue>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<ComputedNumberValue>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&RangeNumberListValue> for RangeNumberListValue {
    fn from(value: &RangeNumberListValue) -> Self {
        value.clone()
    }
}
impl RangeNumberListValue {
    pub fn builder() -> builder::RangeNumberListValue {
        Default::default()
    }
}
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range String List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"list\","]
#[doc = "    \"source\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"range-string-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeStringListValue {
    #[serde(default)]
    pub count: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(default)]
    pub end: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    pub list: ::std::boxed::Box<ComputedStringListValue>,
    pub source: Source,
    #[serde(default)]
    pub start: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&RangeStringListValue> for RangeStringListValue {
    fn from(value: &RangeStringListValue) -> Self {
        value.clone()
    }
}
impl RangeStringListValue {
    pub fn builder() -> builder::RangeStringListValue {
        Default::default()
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
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The script's code.  If present, allows better error reporting by showing the related script text.\","]
#[doc = "      \"type\": \"string\""]
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
    #[doc = "The script's code.  If present, allows better error reporting by showing the related script text."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
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
#[doc = "Turns a string into a string array, splitting items by a specific string."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Split-String Value\","]
#[doc = "  \"description\": \"Turns a string into a string array, splitting items by a specific string.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"separator\","]
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
#[doc = "      \"const\": \"split-string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SplitStringValue {
    pub separator: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ::std::boxed::Box<ComputedStringValue>,
}
impl ::std::convert::From<&SplitStringValue> for SplitStringValue {
    fn from(value: &SplitStringValue) -> Self {
        value.clone()
    }
}
impl SplitStringValue {
    pub fn builder() -> builder::SplitStringValue {
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
#[doc = "    \"mode\","]
#[doc = "    \"source\","]
#[doc = "    \"to-node\","]
#[doc = "    \"to-stream\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fd\": {"]
#[doc = "      \"description\": \"The FD of the stream.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mode\": {"]
#[doc = "      \"description\": \"The type of the stream.  Even for modules where this maps to a fixed stream, this helps ensure the script's assumptions and the module align.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"input\","]
#[doc = "        \"output\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the stream, if associated with a named stream.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"to-node\": {"]
#[doc = "      \"description\": \"The name of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"to-stream\": {"]
#[doc = "      \"description\": \"The FD or named stream of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"string\""]
#[doc = "      ]"]
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fd: ::std::option::Option<i64>,
    #[doc = "The type of the stream.  Even for modules where this maps to a fixed stream, this helps ensure the script's assumptions and the module align."]
    pub mode: StreamMode,
    #[doc = "The name of the stream, if associated with a named stream."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub source: Source,
    #[doc = "The name of the node that this stream connects to.  This is used to link the node to other nodes."]
    #[serde(rename = "to-node")]
    pub to_node: ::std::string::String,
    #[doc = "The FD or named stream of the node that this stream connects to.  This is used to link the node to other nodes."]
    #[serde(rename = "to-stream")]
    pub to_stream: StreamToStream,
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
#[doc = "The type of the stream.  Even for modules where this maps to a fixed stream, this helps ensure the script's assumptions and the module align."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of the stream.  Even for modules where this maps to a fixed stream, this helps ensure the script's assumptions and the module align.\","]
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
#[doc = "The FD or named stream of the node that this stream connects to.  This is used to link the node to other nodes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The FD or named stream of the node that this stream connects to.  This is used to link the node to other nodes.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum StreamToStream {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for StreamToStream {
    fn from(value: &StreamToStream) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StreamToStream {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamToStream {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamToStream {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamToStream {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for StreamToStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for StreamToStream {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Extract an internal part of another string.  The 'count' field cannot be used with 'end'.  If 'start' is not given, then it defaults to 0, and if 'end' or 'count' are not given, then it defaults to the end of the string."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Sub-String Value\","]
#[doc = "  \"description\": \"Extract an internal part of another string.  The 'count' field cannot be used with 'end'.  If 'start' is not given, then it defaults to 0, and if 'end' or 'count' are not given, then it defaults to the end of the string.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"substring\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubStringValue {
    #[serde(default)]
    pub count: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(default)]
    pub end: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    pub source: Source,
    #[serde(default)]
    pub start: ::std::boxed::Box<::std::option::Option<ComputedNumberValue>>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
    pub value: ::std::boxed::Box<ComputedStringValue>,
}
impl ::std::convert::From<&SubStringValue> for SubStringValue {
    fn from(value: &SubStringValue) -> Self {
        value.clone()
    }
}
impl SubStringValue {
    pub fn builder() -> builder::SubStringValue {
        Default::default()
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
        value: ::std::result::Result<super::ComputedValue, ::std::string::String>,
    }
    impl ::std::default::Default for ActionParameter {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
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
            T: ::std::convert::TryInto<super::ComputedValue>,
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
                ::std::option::Option<super::ComputedBooleanValue>,
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
                    ::std::option::Option<super::ComputedBooleanValue>,
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
                ::std::option::Option<super::ComputedNumberValue>,
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
                    ::std::option::Option<super::ComputedNumberValue>,
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
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
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
                    ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
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
    pub struct EventListener {
        actions: ::std::result::Result<super::OrderedActions, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
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
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::EventNameType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventName {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EventName {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventNameType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
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
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EventName> for EventName {
        fn from(value: super::EventName) -> Self {
            Self {
                name: Ok(value.name),
                source: Ok(value.source),
                text: Ok(value.text),
                type_: Ok(value.type_),
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
        behavior: ::std::result::Result<super::ActionEndBehavior, ::std::string::String>,
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
            T: ::std::convert::TryInto<super::ActionEndBehavior>,
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
    pub struct ListIndexBooleanValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        index: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        list: ::std::result::Result<super::ComputedBooleanListValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexBooleanValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ListIndexBooleanValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedBooleanListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
    impl ::std::convert::TryFrom<ListIndexBooleanValue> for super::ListIndexBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                list: value.list?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexBooleanValue> for ListIndexBooleanValue {
        fn from(value: super::ListIndexBooleanValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                list: Ok(value.list),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListIndexNumberValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        index: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexNumberValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ListIndexNumberValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberListValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
    impl ::std::convert::TryFrom<ListIndexNumberValue> for super::ListIndexNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                list: value.list?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexNumberValue> for ListIndexNumberValue {
        fn from(value: super::ListIndexNumberValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                list: Ok(value.list),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListIndexStringValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        index: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        list: ::std::result::Result<super::ComputedStringListValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexStringValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ListIndexStringValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
    impl ::std::convert::TryFrom<ListIndexStringValue> for super::ListIndexStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                list: value.list?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexStringValue> for ListIndexStringValue {
        fn from(value: super::ListIndexStringValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                list: Ok(value.list),
                source: Ok(value.source),
                type_: Ok(value.type_),
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
        name: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        node: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
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
    pub struct LookupStringValue {
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
    impl ::std::default::Default for LookupStringValue {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LookupStringValue {
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
    impl ::std::convert::TryFrom<LookupStringValue> for super::LookupStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                node: value.node?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringValue> for LookupStringValue {
        fn from(value: super::LookupStringValue) -> Self {
            Self {
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapKeyBooleanValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        key: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        map: ::std::result::Result<super::ComputedBooleanMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyBooleanValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MapKeyBooleanValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn map<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedBooleanMapValue>,
            T::Error: ::std::fmt::Display,
        {
            self.map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map: {}", e));
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
    impl ::std::convert::TryFrom<MapKeyBooleanValue> for super::MapKeyBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                map: value.map?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyBooleanValue> for MapKeyBooleanValue {
        fn from(value: super::MapKeyBooleanValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                map: Ok(value.map),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapKeyNumberValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        key: ::std::result::Result<super::ComputedStringValue, ::std::string::String>,
        map: ::std::result::Result<super::ComputedNumberMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyNumberValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MapKeyNumberValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringValue>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn map<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedNumberMapValue>,
            T::Error: ::std::fmt::Display,
        {
            self.map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map: {}", e));
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
    impl ::std::convert::TryFrom<MapKeyNumberValue> for super::MapKeyNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                map: value.map?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyNumberValue> for MapKeyNumberValue {
        fn from(value: super::MapKeyNumberValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                map: Ok(value.map),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapKeyStringValue {
        default: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        key: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        map: ::std::result::Result<super::ComputedStringMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyStringValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MapKeyStringValue {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn map<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedStringMapValue>,
            T::Error: ::std::fmt::Display,
        {
            self.map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map: {}", e));
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
    impl ::std::convert::TryFrom<MapKeyStringValue> for super::MapKeyStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                map: value.map?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyStringValue> for MapKeyStringValue {
        fn from(value: super::MapKeyStringValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                map: Ok(value.map),
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
        events: ::std::result::Result<::std::vec::Vec<super::EventName>, ::std::string::String>,
        name: ::std::result::Result<super::NativeShellAstSchemaName, ::std::string::String>,
        nodes: ::std::result::Result<::std::vec::Vec<super::Node>, ::std::string::String>,
        schema_version: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for NativeShellAstSchema {
        fn default() -> Self {
            Self {
                events: Err("no value supplied for events".to_string()),
                name: Err("no value supplied for name".to_string()),
                nodes: Err("no value supplied for nodes".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                source: Err("no value supplied for source".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl NativeShellAstSchema {
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
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NativeShellAstSchemaName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
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
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema_version: {}", e));
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
                events: value.events?,
                name: value.name?,
                nodes: value.nodes?,
                schema_version: value.schema_version?,
                source: value.source?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::NativeShellAstSchema> for NativeShellAstSchema {
        fn from(value: super::NativeShellAstSchema) -> Self {
            Self {
                events: Ok(value.events),
                name: Ok(value.name),
                nodes: Ok(value.nodes),
                schema_version: Ok(value.schema_version),
                source: Ok(value.source),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Node {
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
    pub struct NumberToStringValue {
        format: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
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
    pub struct RangeBooleanListValue {
        count: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for RangeBooleanListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RangeBooleanListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanListValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
    impl ::std::convert::TryFrom<RangeBooleanListValue> for super::RangeBooleanListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RangeBooleanListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                end: value.end?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RangeBooleanListValue> for RangeBooleanListValue {
        fn from(value: super::RangeBooleanListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RangeNumberListValue {
        count: ::std::result::Result<
            ::std::option::Option<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::option::Option<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::option::Option<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for RangeNumberListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RangeNumberListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberListValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
    impl ::std::convert::TryFrom<RangeNumberListValue> for super::RangeNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RangeNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                end: value.end?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RangeNumberListValue> for RangeNumberListValue {
        fn from(value: super::RangeNumberListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RangeStringListValue {
        count: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for RangeStringListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RangeStringListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringListValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {}", e));
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
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
    impl ::std::convert::TryFrom<RangeStringListValue> for super::RangeStringListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RangeStringListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                end: value.end?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RangeStringListValue> for RangeStringListValue {
        fn from(value: super::RangeStringListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Source {
        column: ::std::result::Result<i64, ::std::string::String>,
        file: ::std::result::Result<::std::string::String, ::std::string::String>,
        line: ::std::result::Result<i64, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Source {
        fn default() -> Self {
            Self {
                column: Err("no value supplied for column".to_string()),
                file: Err("no value supplied for file".to_string()),
                line: Err("no value supplied for line".to_string()),
                text: Ok(Default::default()),
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
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
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::Source> for Source {
        fn from(value: super::Source) -> Self {
            Self {
                column: Ok(value.column),
                file: Ok(value.file),
                line: Ok(value.line),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SplitStringValue {
        separator: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SplitStringValue {
        fn default() -> Self {
            Self {
                separator: Err("no value supplied for separator".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SplitStringValue {
        pub fn separator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SplitStringValue> for super::SplitStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SplitStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                separator: value.separator?,
                source: value.source?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SplitStringValue> for SplitStringValue {
        fn from(value: super::SplitStringValue) -> Self {
            Self {
                separator: Ok(value.separator),
                source: Ok(value.source),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Stream {
        fd: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mode: ::std::result::Result<super::StreamMode, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        to_node: ::std::result::Result<::std::string::String, ::std::string::String>,
        to_stream: ::std::result::Result<super::StreamToStream, ::std::string::String>,
    }
    impl ::std::default::Default for Stream {
        fn default() -> Self {
            Self {
                fd: Ok(Default::default()),
                mode: Err("no value supplied for mode".to_string()),
                name: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                to_node: Err("no value supplied for to_node".to_string()),
                to_stream: Err("no value supplied for to_stream".to_string()),
            }
        }
    }
    impl Stream {
        pub fn fd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
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
        pub fn to_stream<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamToStream>,
            T::Error: ::std::fmt::Display,
        {
            self.to_stream = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_stream: {}", e));
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
                to_node: value.to_node?,
                to_stream: value.to_stream?,
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
                to_node: Ok(value.to_node),
                to_stream: Ok(value.to_stream),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubStringValue {
        count: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubStringValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SubStringValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
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
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::boxed::Box<::std::option::Option<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedStringValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SubStringValue> for super::SubStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                end: value.end?,
                source: value.source?,
                start: value.start?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SubStringValue> for SubStringValue {
        fn from(value: super::SubStringValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                source: Ok(value.source),
                start: Ok(value.start),
                type_: Ok(value.type_),
                value: Ok(value.value),
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
