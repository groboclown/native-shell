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
#[doc = "A number value that is the absolute value of another number."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Absolute Value\","]
#[doc = "  \"description\": \"A number value that is the absolute value of another number.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"abs-number\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AbsValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberValue,
}
impl ::std::convert::From<&AbsValue> for AbsValue {
    fn from(value: &AbsValue) -> Self {
        value.clone()
    }
}
impl AbsValue {
    pub fn builder() -> builder::AbsValue {
        Default::default()
    }
}
#[doc = "A parameter for the job. Must provide exactly one of a value or a value-array."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action Parameter\","]
#[doc = "  \"description\": \"A parameter for the job. Must provide exactly one of a value or a value-array.\","]
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
#[doc = "A number value that is the sum of two other number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Add Two Values\","]
#[doc = "  \"description\": \"A number value that is the sum of two other number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"add-number\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct AddTwoValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedNumberValue>,
    pub right: ::std::boxed::Box<ComputedNumberValue>,
    pub source: Source,
}
impl ::std::convert::From<&AddTwoValues> for AddTwoValues {
    fn from(value: &AddTwoValues) -> Self {
        value.clone()
    }
}
impl AddTwoValues {
    pub fn builder() -> builder::AddTwoValues {
        Default::default()
    }
}
#[doc = "`AliasListItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AliasListItem(::std::string::String);
impl ::std::ops::Deref for AliasListItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AliasListItem> for ::std::string::String {
    fn from(value: AliasListItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AliasListItem> for AliasListItem {
    fn from(value: &AliasListItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AliasListItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AliasListItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AliasListItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AliasListItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AliasListItem {
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
#[doc = "A boolean value that is the result of a logical AND operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"And Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical AND operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"and-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct AndTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&AndTwoBooleanValues> for AndTwoBooleanValues {
    fn from(value: &AndTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl AndTwoBooleanValues {
    pub fn builder() -> builder::AndTwoBooleanValues {
        Default::default()
    }
}
#[doc = "A number value that is the average of all numbers in a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Average Number List Value\","]
#[doc = "  \"description\": \"A number value that is the average of all numbers in a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"average-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct AverageNumberListValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&AverageNumberListValue> for AverageNumberListValue {
    fn from(value: &AverageNumberListValue) -> Self {
        value.clone()
    }
}
impl AverageNumberListValue {
    pub fn builder() -> builder::AverageNumberListValue {
        Default::default()
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"false-string\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"boolean-to-string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"true-string\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
    #[serde(
        rename = "true-string",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub true_string: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
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
#[doc = "A number value that is the result of rounding another number up to the nearest integer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ceil Value\","]
#[doc = "  \"description\": \"A number value that is the result of rounding another number up to the nearest integer.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"ceil-number\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CeilValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ::std::boxed::Box<ComputedNumberValue>,
}
impl ::std::convert::From<&CeilValue> for CeilValue {
    fn from(value: &CeilValue) -> Self {
        value.clone()
    }
}
impl CeilValue {
    pub fn builder() -> builder::CeilValue {
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
#[doc = "      \"$ref\": \"#/$defs/AndTwoBooleanValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/OrTwoBooleanValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/NotBooleanValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/XorTwoBooleanValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/NandTwoBooleanValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/NorTwoBooleanValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/XnorTwoBooleanValues\""]
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
    AndTwoBooleanValues(AndTwoBooleanValues),
    OrTwoBooleanValues(OrTwoBooleanValues),
    NotBooleanValue(NotBooleanValue),
    XorTwoBooleanValues(XorTwoBooleanValues),
    NandTwoBooleanValues(NandTwoBooleanValues),
    NorTwoBooleanValues(NorTwoBooleanValues),
    XnorTwoBooleanValues(XnorTwoBooleanValues),
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
impl ::std::convert::From<AndTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: AndTwoBooleanValues) -> Self {
        Self::AndTwoBooleanValues(value)
    }
}
impl ::std::convert::From<OrTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: OrTwoBooleanValues) -> Self {
        Self::OrTwoBooleanValues(value)
    }
}
impl ::std::convert::From<NotBooleanValue> for ComputedBooleanValue {
    fn from(value: NotBooleanValue) -> Self {
        Self::NotBooleanValue(value)
    }
}
impl ::std::convert::From<XorTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: XorTwoBooleanValues) -> Self {
        Self::XorTwoBooleanValues(value)
    }
}
impl ::std::convert::From<NandTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: NandTwoBooleanValues) -> Self {
        Self::NandTwoBooleanValues(value)
    }
}
impl ::std::convert::From<NorTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: NorTwoBooleanValues) -> Self {
        Self::NorTwoBooleanValues(value)
    }
}
impl ::std::convert::From<XnorTwoBooleanValues> for ComputedBooleanValue {
    fn from(value: XnorTwoBooleanValues) -> Self {
        Self::XnorTwoBooleanValues(value)
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
#[doc = "      \"$ref\": \"#/$defs/AddTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SubtractTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MultiplyTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/DivideTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ModulusTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/PowerTwoValues\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RoundValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/FloorValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/CeilValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/AbsValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SumNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ProductNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/AverageNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MinNumberListValue\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MaxNumberListValue\""]
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
    AddTwoValues(AddTwoValues),
    SubtractTwoValues(SubtractTwoValues),
    MultiplyTwoValues(MultiplyTwoValues),
    DivideTwoValues(DivideTwoValues),
    ModulusTwoValues(ModulusTwoValues),
    PowerTwoValues(PowerTwoValues),
    RoundValue(RoundValue),
    FloorValue(FloorValue),
    CeilValue(CeilValue),
    AbsValue(::std::boxed::Box<AbsValue>),
    SumNumberListValue(SumNumberListValue),
    ProductNumberListValue(ProductNumberListValue),
    AverageNumberListValue(AverageNumberListValue),
    MinNumberListValue(MinNumberListValue),
    MaxNumberListValue(MaxNumberListValue),
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
impl ::std::convert::From<AddTwoValues> for ComputedNumberValue {
    fn from(value: AddTwoValues) -> Self {
        Self::AddTwoValues(value)
    }
}
impl ::std::convert::From<SubtractTwoValues> for ComputedNumberValue {
    fn from(value: SubtractTwoValues) -> Self {
        Self::SubtractTwoValues(value)
    }
}
impl ::std::convert::From<MultiplyTwoValues> for ComputedNumberValue {
    fn from(value: MultiplyTwoValues) -> Self {
        Self::MultiplyTwoValues(value)
    }
}
impl ::std::convert::From<DivideTwoValues> for ComputedNumberValue {
    fn from(value: DivideTwoValues) -> Self {
        Self::DivideTwoValues(value)
    }
}
impl ::std::convert::From<ModulusTwoValues> for ComputedNumberValue {
    fn from(value: ModulusTwoValues) -> Self {
        Self::ModulusTwoValues(value)
    }
}
impl ::std::convert::From<PowerTwoValues> for ComputedNumberValue {
    fn from(value: PowerTwoValues) -> Self {
        Self::PowerTwoValues(value)
    }
}
impl ::std::convert::From<RoundValue> for ComputedNumberValue {
    fn from(value: RoundValue) -> Self {
        Self::RoundValue(value)
    }
}
impl ::std::convert::From<FloorValue> for ComputedNumberValue {
    fn from(value: FloorValue) -> Self {
        Self::FloorValue(value)
    }
}
impl ::std::convert::From<CeilValue> for ComputedNumberValue {
    fn from(value: CeilValue) -> Self {
        Self::CeilValue(value)
    }
}
impl ::std::convert::From<::std::boxed::Box<AbsValue>> for ComputedNumberValue {
    fn from(value: ::std::boxed::Box<AbsValue>) -> Self {
        Self::AbsValue(value)
    }
}
impl ::std::convert::From<SumNumberListValue> for ComputedNumberValue {
    fn from(value: SumNumberListValue) -> Self {
        Self::SumNumberListValue(value)
    }
}
impl ::std::convert::From<ProductNumberListValue> for ComputedNumberValue {
    fn from(value: ProductNumberListValue) -> Self {
        Self::ProductNumberListValue(value)
    }
}
impl ::std::convert::From<AverageNumberListValue> for ComputedNumberValue {
    fn from(value: AverageNumberListValue) -> Self {
        Self::AverageNumberListValue(value)
    }
}
impl ::std::convert::From<MinNumberListValue> for ComputedNumberValue {
    fn from(value: MinNumberListValue) -> Self {
        Self::MinNumberListValue(value)
    }
}
impl ::std::convert::From<MaxNumberListValue> for ComputedNumberValue {
    fn from(value: MaxNumberListValue) -> Self {
        Self::MaxNumberListValue(value)
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
#[doc = "      \"$ref\": \"#/$defs/TrimStringValue\""]
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
    TrimStringValue(TrimStringValue),
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
impl ::std::convert::From<TrimStringValue> for ComputedStringValue {
    fn from(value: TrimStringValue) -> Self {
        Self::TrimStringValue(value)
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
#[doc = "A value that is computed at runtime.  This can be a constant value, a lookup from a job's state, or an array of values, or an operation on a list of values.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Computed Value\","]
#[doc = "  \"description\": \"A value that is computed at runtime.  This can be a constant value, a lookup from a job's state, or an array of values, or an operation on a list of values.\\n\","]
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
#[doc = "A constant integer value declared within the script.  Used as an inline value.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Integer Value\","]
#[doc = "  \"description\": \"A constant integer value declared within the script.  Used as an inline value.\\n\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maxValue\": 4294967295,"]
#[doc = "  \"minValue\": -4294967296"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ConstInt(pub i64);
impl ::std::ops::Deref for ConstInt {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<ConstInt> for i64 {
    fn from(value: ConstInt) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConstInt> for ConstInt {
    fn from(value: &ConstInt) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for ConstInt {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ConstInt {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ConstInt {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ConstInt {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ConstInt {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ConstInt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "A constant string value declared within the script.  Used as an inline value.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String Value\","]
#[doc = "  \"description\": \"A constant string value declared within the script.  Used as an inline value.\\n\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 65535,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConstString(::std::string::String);
impl ::std::ops::Deref for ConstString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConstString> for ::std::string::String {
    fn from(value: ConstString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConstString> for ConstString {
    fn from(value: &ConstString) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ConstString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 65535usize {
            return Err("longer than 65535 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConstString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConstString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConstString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConstString {
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-boolean-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "A constant boolean map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Boolean Map Value\","]
#[doc = "  \"description\": \"A constant boolean map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-boolean-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
        if PATTERN.find(value).is_none() {
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-boolean\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "A constant number map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant Number Map Value\","]
#[doc = "  \"description\": \"A constant number map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-number-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
    #[doc = "The constant number map value."]
    pub value: ::std::collections::HashMap<
        ConstantNumberMapValueValueKey,
        ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
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
        if PATTERN.find(value).is_none() {
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-number\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-string-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "A constant string map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant String Map Value\","]
#[doc = "  \"description\": \"A constant string map value.  Can include expanding a sub-map within the map. This also includes a 'null' value for the key to allow blanking out values if used in a union.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-string-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
        if PATTERN.find(value).is_none() {
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"constant-string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "`DescriptionItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 10000,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DescriptionItem(::std::string::String);
impl ::std::ops::Deref for DescriptionItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DescriptionItem> for ::std::string::String {
    fn from(value: DescriptionItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DescriptionItem> for DescriptionItem {
    fn from(value: &DescriptionItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DescriptionItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10000usize {
            return Err("longer than 10000 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DescriptionItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DescriptionItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DescriptionItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DescriptionItem {
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
#[doc = "A number value that is the quotient of two other number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Divide Two Values\","]
#[doc = "  \"description\": \"A number value that is the quotient of two other number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"divide-number\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct DivideTwoValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedNumberValue>,
    pub right: ::std::boxed::Box<ComputedNumberValue>,
    pub source: Source,
}
impl ::std::convert::From<&DivideTwoValues> for DivideTwoValues {
    fn from(value: &DivideTwoValues) -> Self {
        value.clone()
    }
}
impl DivideTwoValues {
    pub fn builder() -> builder::DivideTwoValues {
        Default::default()
    }
}
#[doc = "A description of a parameter that comes from the environment. This will grant jobs to reference the value from runtime parameters.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Environment Parameter\","]
#[doc = "  \"description\": \"A description of a parameter that comes from the environment. This will grant jobs to reference the value from runtime parameters.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Alias List\","]
#[doc = "      \"description\": \"A list of aliases the user can give to fill in the value.  Useful for allowing an environment variable in the form THE_NAME and the CLI parameters '--the-name' and '-n'.\\n\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 100,"]
#[doc = "        \"minLength\": 0"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 1000,"]
#[doc = "      \"minItems\": 0"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"title\": \"Description\","]
#[doc = "      \"description\": \"Help text for the parameter. Currently, only allows for a list of text for a single language.  Eventually may allow for multiple langauges.\\n\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 10000,"]
#[doc = "        \"minLength\": 0"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 1000,"]
#[doc = "      \"minItems\": 0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"title\": \"Value type\","]
#[doc = "      \"description\": \"Allowed type of value.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\","]
#[doc = "        \"string-list\","]
#[doc = "        \"int\","]
#[doc = "        \"int-list\","]
#[doc = "        \"flag\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EnvironmentParameter {
    #[doc = "A list of aliases the user can give to fill in the value.  Useful for allowing an environment variable in the form THE_NAME and the CLI parameters '--the-name' and '-n'.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub aliases: ::std::vec::Vec<AliasListItem>,
    #[doc = "Help text for the parameter. Currently, only allows for a list of text for a single language.  Eventually may allow for multiple langauges.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub description: ::std::vec::Vec<DescriptionItem>,
    #[doc = "Allowed type of value."]
    pub kind: ValueType,
    pub source: Source,
}
impl ::std::convert::From<&EnvironmentParameter> for EnvironmentParameter {
    fn from(value: &EnvironmentParameter) -> Self {
        value.clone()
    }
}
impl EnvironmentParameter {
    pub fn builder() -> builder::EnvironmentParameter {
        Default::default()
    }
}
#[doc = "The text payload for a message kind of event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Payload Message\","]
#[doc = "  \"description\": \"The text payload for a message kind of event.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 10000,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventPayloadMessage(::std::string::String);
impl ::std::ops::Deref for EventPayloadMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventPayloadMessage> for ::std::string::String {
    fn from(value: EventPayloadMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EventPayloadMessage> for EventPayloadMessage {
    fn from(value: &EventPayloadMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EventPayloadMessage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10000usize {
            return Err("longer than 10000 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventPayloadMessage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventPayloadMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventPayloadMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventPayloadMessage {
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
#[doc = "The number payload for a signal kind of event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Payload Signal\","]
#[doc = "  \"description\": \"The number payload for a signal kind of event.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maxValue\": 32767,"]
#[doc = "  \"minValue\": -32768"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventPayloadSignal(pub i64);
impl ::std::ops::Deref for EventPayloadSignal {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<EventPayloadSignal> for i64 {
    fn from(value: EventPayloadSignal) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EventPayloadSignal> for EventPayloadSignal {
    fn from(value: &EventPayloadSignal) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for EventPayloadSignal {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EventPayloadSignal {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EventPayloadSignal {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for EventPayloadSignal {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EventPayloadSignal {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EventPayloadSignal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Reference to a named event.  Named events exist within the script, and some modules have expectations for certain kinds of events.  These must all align, especially with the payload kind (either a Message (string) or Signal (number)).\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"EventRef\","]
#[doc = "  \"description\": \"Reference to a named event.  Named events exist within the script, and some modules have expectations for certain kinds of events.  These must all align, especially with the payload kind (either a Message (string) or Signal (number)).\\n\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventRef(::std::string::String);
impl ::std::ops::Deref for EventRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventRef> for ::std::string::String {
    fn from(value: EventRef) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EventRef> for EventRef {
    fn from(value: &EventRef) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EventRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventRef {
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
#[doc = "The action to take after waiting on a job or thread, based on how it stopped."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Exit Behavior\","]
#[doc = "  \"description\": \"The action to take after waiting on a job or thread, based on how it stopped.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default-behavior\": {"]
#[doc = "      \"$ref\": \"#/$defs/OnExitBehavior\""]
#[doc = "    },"]
#[doc = "    \"exit-code-behaviors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Exit Code Behavior\","]
#[doc = "        \"description\": \"A behavior based on the exit code numeric value. At least one of 'min-code' or 'max-code' must be set.\\n\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"behavior\","]
#[doc = "          \"source\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"behavior\": {"]
#[doc = "            \"$ref\": \"#/$defs/OnExitBehavior\""]
#[doc = "          },"]
#[doc = "          \"max-code\": {"]
#[doc = "            \"$ref\": \"#/$defs/ExitCode\""]
#[doc = "          },"]
#[doc = "          \"min-code\": {"]
#[doc = "            \"$ref\": \"#/$defs/ExitCode\""]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"$ref\": \"#/$defs/Source\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 1000,"]
#[doc = "      \"minItems\": 0"]
#[doc = "    },"]
#[doc = "    \"never-started\": {"]
#[doc = "      \"$ref\": \"#/$defs/OnExitBehavior\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"requires\": ["]
#[doc = "    \"source\","]
#[doc = "    \"never-started\","]
#[doc = "    \"exit-code-behaviors\","]
#[doc = "    \"default-behavior\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExitBehavior {
    #[serde(
        rename = "default-behavior",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_behavior: ::std::option::Option<OnExitBehavior>,
    #[serde(
        rename = "exit-code-behaviors",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_code_behaviors: ::std::vec::Vec<ExitCodeBehavior>,
    #[serde(
        rename = "never-started",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub never_started: ::std::option::Option<OnExitBehavior>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<Source>,
}
impl ::std::convert::From<&ExitBehavior> for ExitBehavior {
    fn from(value: &ExitBehavior) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ExitBehavior {
    fn default() -> Self {
        Self {
            default_behavior: Default::default(),
            exit_code_behaviors: Default::default(),
            never_started: Default::default(),
            source: Default::default(),
        }
    }
}
impl ExitBehavior {
    pub fn builder() -> builder::ExitBehavior {
        Default::default()
    }
}
#[doc = "The script exit code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The script exit code.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maxValue\": 32767,"]
#[doc = "  \"minValue\": -32768"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ExitCode(pub i64);
impl ::std::ops::Deref for ExitCode {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<ExitCode> for i64 {
    fn from(value: ExitCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExitCode> for ExitCode {
    fn from(value: &ExitCode) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for ExitCode {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ExitCode {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ExitCode {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ExitCode {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ExitCode {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "A behavior based on the exit code numeric value. At least one of 'min-code' or 'max-code' must be set.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Exit Code Behavior\","]
#[doc = "  \"description\": \"A behavior based on the exit code numeric value. At least one of 'min-code' or 'max-code' must be set.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"behavior\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"behavior\": {"]
#[doc = "      \"$ref\": \"#/$defs/OnExitBehavior\""]
#[doc = "    },"]
#[doc = "    \"max-code\": {"]
#[doc = "      \"$ref\": \"#/$defs/ExitCode\""]
#[doc = "    },"]
#[doc = "    \"min-code\": {"]
#[doc = "      \"$ref\": \"#/$defs/ExitCode\""]
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
pub struct ExitCodeBehavior {
    pub behavior: OnExitBehavior,
    #[serde(
        rename = "max-code",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_code: ::std::option::Option<ExitCode>,
    #[serde(
        rename = "min-code",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_code: ::std::option::Option<ExitCode>,
    pub source: Source,
}
impl ::std::convert::From<&ExitCodeBehavior> for ExitCodeBehavior {
    fn from(value: &ExitCodeBehavior) -> Self {
        value.clone()
    }
}
impl ExitCodeBehavior {
    pub fn builder() -> builder::ExitCodeBehavior {
        Default::default()
    }
}
#[doc = "The file where the node is defined."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"File\","]
#[doc = "  \"description\": \"The file where the node is defined.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 10000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct File(::std::string::String);
impl ::std::ops::Deref for File {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<File> for ::std::string::String {
    fn from(value: File) -> Self {
        value.0
    }
}
impl ::std::convert::From<&File> for File {
    fn from(value: &File) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for File {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10000usize {
            return Err("longer than 10000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for File {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for File {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for File {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for File {
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
#[doc = "A numeric reference to a stream, usually corresponding to the inputs and outputs for a program.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"File Descriptor\","]
#[doc = "  \"description\": \"A numeric reference to a stream, usually corresponding to the inputs and outputs for a program.\\n\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maxValue\": 65535,"]
#[doc = "  \"minValue\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FileDescriptor(pub i64);
impl ::std::ops::Deref for FileDescriptor {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<FileDescriptor> for i64 {
    fn from(value: FileDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileDescriptor> for FileDescriptor {
    fn from(value: &FileDescriptor) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for FileDescriptor {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for FileDescriptor {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for FileDescriptor {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for FileDescriptor {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for FileDescriptor {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for FileDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "A number value that is the result of rounding another number down to the nearest integer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Floor Value\","]
#[doc = "  \"description\": \"A number value that is the result of rounding another number down to the nearest integer.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"floor-number\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FloorValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ::std::boxed::Box<ComputedNumberValue>,
}
impl ::std::convert::From<&FloorValue> for FloorValue {
    fn from(value: &FloorValue) -> Self {
        value.clone()
    }
}
impl FloorValue {
    pub fn builder() -> builder::FloorValue {
        Default::default()
    }
}
#[doc = "The compile-time parameters for the node.  These parameters are static and help initialize the module."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Initial Parameters\","]
#[doc = "  \"description\": \"The compile-time parameters for the node.  These parameters are static and help initialize the module.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \".*\": {"]
#[doc = "      \"title\": \"Parameter\","]
#[doc = "      \"description\": \"A compile-time parameter for the node.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time String\","]
#[doc = "              \"description\": \"A constant string value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"string\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"$ref\": \"#/$defs/ConstString\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Number\","]
#[doc = "              \"description\": \"A constant number value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"number\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Boolean\","]
#[doc = "              \"description\": \"A constant boolean (true/false) value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"description\": \"The constant boolean value.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Null\","]
#[doc = "              \"description\": \"A constant null value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"null\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time String List\","]
#[doc = "              \"description\": \"A constant string list value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"string-list\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"$ref\": \"#/$defs/ConstString\""]
#[doc = "                  },"]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Number List\","]
#[doc = "              \"description\": \"A constant number list value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"number-list\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "                  },"]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Boolean List\","]
#[doc = "              \"description\": \"A constant boolean list value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"boolean-list\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"type\": \"boolean\""]
#[doc = "                  },"]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time String Map\","]
#[doc = "              \"description\": \"A constant string map value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"string-map\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0,"]
#[doc = "                  \"patternProperties\": {"]
#[doc = "                    \".*\": {"]
#[doc = "                      \"$ref\": \"#/$defs/ConstString\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Number Map\","]
#[doc = "              \"description\": \"A constant number map value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"number-map\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0,"]
#[doc = "                  \"patternProperties\": {"]
#[doc = "                    \".*\": {"]
#[doc = "                      \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"title\": \"Compile-Time Boolean Map\","]
#[doc = "              \"description\": \"A constant boolean map value.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"kind\","]
#[doc = "                \"source\","]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"boolean-map\""]
#[doc = "                },"]
#[doc = "                \"source\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Source\""]
#[doc = "                },"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"maxItems\": 1000,"]
#[doc = "                  \"minItems\": 0,"]
#[doc = "                  \"patternProperties\": {"]
#[doc = "                    \".*\": {"]
#[doc = "                      \"type\": \"boolean\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct InitialParameters(pub ::std::collections::HashMap<InitialParametersKey, Parameter>);
impl ::std::ops::Deref for InitialParameters {
    type Target = ::std::collections::HashMap<InitialParametersKey, Parameter>;
    fn deref(&self) -> &::std::collections::HashMap<InitialParametersKey, Parameter> {
        &self.0
    }
}
impl ::std::convert::From<InitialParameters>
    for ::std::collections::HashMap<InitialParametersKey, Parameter>
{
    fn from(value: InitialParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InitialParameters> for InitialParameters {
    fn from(value: &InitialParameters) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<InitialParametersKey, Parameter>>
    for InitialParameters
{
    fn from(value: ::std::collections::HashMap<InitialParametersKey, Parameter>) -> Self {
        Self(value)
    }
}
#[doc = "`InitialParametersKey`"]
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
pub struct InitialParametersKey(::std::string::String);
impl ::std::ops::Deref for InitialParametersKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InitialParametersKey> for ::std::string::String {
    fn from(value: InitialParametersKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InitialParametersKey> for InitialParametersKey {
    fn from(value: &InitialParametersKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InitialParametersKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InitialParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InitialParametersKey {
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
#[doc = "A job with explicit code that describes the execution behavior."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Inline Job\","]
#[doc = "  \"description\": \"A job with explicit code that describes the execution behavior.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The source code to execute within the job.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 65535,"]
#[doc = "      \"minLength\": 0"]
#[doc = "    },"]
#[doc = "    \"initial-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/InitialParameters\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"title\": \"Kind\","]
#[doc = "      \"description\": \"Job definition distinguisher\","]
#[doc = "      \"const\": \"inline\""]
#[doc = "    },"]
#[doc = "    \"runtime-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InlineJob {
    #[doc = "The source code to execute within the job."]
    pub code: InlineJobCode,
    #[serde(
        rename = "initial-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_parameters: ::std::option::Option<InitialParameters>,
    #[doc = "Job definition distinguisher"]
    pub kind: ::serde_json::Value,
    #[serde(
        rename = "runtime-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub runtime_parameters: ::std::option::Option<NamedParameters>,
}
impl ::std::convert::From<&InlineJob> for InlineJob {
    fn from(value: &InlineJob) -> Self {
        value.clone()
    }
}
impl InlineJob {
    pub fn builder() -> builder::InlineJob {
        Default::default()
    }
}
#[doc = "The source code to execute within the job."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The source code to execute within the job.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 65535,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InlineJobCode(::std::string::String);
impl ::std::ops::Deref for InlineJobCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InlineJobCode> for ::std::string::String {
    fn from(value: InlineJobCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InlineJobCode> for InlineJobCode {
    fn from(value: &InlineJobCode) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InlineJobCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 65535usize {
            return Err("longer than 65535 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InlineJobCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InlineJobCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InlineJobCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InlineJobCode {
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
#[doc = "A configured execution element.  The 'name' of the job is in the property key of the owning object, which other objects may reference through a JobRef type.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Job\","]
#[doc = "  \"description\": \"A configured execution element.  The 'name' of the job is in the property key of the owning object, which other objects may reference through a JobRef type.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"def\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"def\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/ModuleJob\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/StreamJob\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/InlineJob\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/MacroJob\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/StateJob\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"rerunnable\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct Job {
    pub def: JobDef,
    #[serde(default = "defaults::default_bool::<true>")]
    pub rerunnable: bool,
    pub source: Source,
}
impl ::std::convert::From<&Job> for Job {
    fn from(value: &Job) -> Self {
        value.clone()
    }
}
impl Job {
    pub fn builder() -> builder::Job {
        Default::default()
    }
}
#[doc = "`JobDef`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ModuleJob\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/StreamJob\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/InlineJob\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MacroJob\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/StateJob\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JobDef {
    ModuleJob(ModuleJob),
    StreamJob(StreamJob),
    InlineJob(InlineJob),
    MacroJob(MacroJob),
    StateJob(StateJob),
}
impl ::std::convert::From<&Self> for JobDef {
    fn from(value: &JobDef) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ModuleJob> for JobDef {
    fn from(value: ModuleJob) -> Self {
        Self::ModuleJob(value)
    }
}
impl ::std::convert::From<StreamJob> for JobDef {
    fn from(value: StreamJob) -> Self {
        Self::StreamJob(value)
    }
}
impl ::std::convert::From<InlineJob> for JobDef {
    fn from(value: InlineJob) -> Self {
        Self::InlineJob(value)
    }
}
impl ::std::convert::From<MacroJob> for JobDef {
    fn from(value: MacroJob) -> Self {
        Self::MacroJob(value)
    }
}
impl ::std::convert::From<StateJob> for JobDef {
    fn from(value: StateJob) -> Self {
        Self::StateJob(value)
    }
}
#[doc = "Reference to a job.  It must reference one of the keys in the \"job\" collection."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"JobRef\","]
#[doc = "  \"description\": \"Reference to a job.  It must reference one of the keys in the \\\"job\\\" collection.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobRef(::std::string::String);
impl ::std::ops::Deref for JobRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobRef> for ::std::string::String {
    fn from(value: JobRef) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JobRef> for JobRef {
    fn from(value: &JobRef) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for JobRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobRef {
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
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"list-index-boolean\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
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
pub struct ListIndexBooleanValue {
    pub default: ::std::boxed::Box<ComputedBooleanValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub kind: ::serde_json::Value,
    pub list: ComputedBooleanListValue,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"list-index-number\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
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
pub struct ListIndexNumberValue {
    pub default: ::std::boxed::Box<ComputedNumberValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub kind: ::serde_json::Value,
    pub list: ComputedNumberListValue,
    pub source: Source,
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
#[doc = "Extracts a single indexed string from a string list.  A default must be given, in case the index is out of bounds.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Indexed String Value\","]
#[doc = "  \"description\": \"Extracts a single indexed string from a string list.  A default must be given, in case the index is out of bounds.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default\","]
#[doc = "    \"index\","]
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"list-index-string\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
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
pub struct ListIndexStringValue {
    pub default: ::std::boxed::Box<ComputedStringValue>,
    pub index: ::std::boxed::Box<ComputedNumberValue>,
    pub kind: ::serde_json::Value,
    pub list: ComputedStringListValue,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"list-to-string\""]
#[doc = "    },"]
#[doc = "    \"separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub separator: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub source: Source,
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
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-boolean-list\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupBooleanListValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "A boolean map value that is looked up from a job's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Boolean Map Value\","]
#[doc = "  \"description\": \"A boolean map value that is looked up from a job's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-boolean-map\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupBooleanMapValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupBooleanValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-number-list\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupNumberListValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "A number map value that is looked up from a job's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup Number Map Value\","]
#[doc = "  \"description\": \"A number map value that is looked up from a job's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-number-map\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupNumberMapValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupNumberValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-state-string-list\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupStringListValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "A string map value that is looked up from a job's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String Map Value\","]
#[doc = "  \"description\": \"A string map value that is looked up from a job's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"job\","]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/$defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-string-map\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParamName\""]
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
pub struct LookupStringMapValue {
    pub job: JobRef,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub source: Source,
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
#[doc = "A string value that is looked up from a job's state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Lookup String Value\","]
#[doc = "  \"description\": \"A string value that is looked up from a job's state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"node\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"job\": {"]
#[doc = "      \"$ref\": \"#/defs/JobRef\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"lookup-string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/defs/ParamName\""]
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
pub struct LookupStringValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub job: ::std::option::Option<JobRef>,
    pub kind: ::serde_json::Value,
    pub name: ParamName,
    pub node: ::serde_json::Value,
    pub source: Source,
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
#[doc = "A job that constructs through a well-defined process, usually through a series of templates.  Example: wiring together multiple jobs' streams.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Macro Job\","]
#[doc = "  \"description\": \"A job that constructs through a well-defined process, usually through a series of templates.  Example: wiring together multiple jobs' streams.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"macro\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"macro\""]
#[doc = "    },"]
#[doc = "    \"macro\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MacroJob {
    pub kind: ::serde_json::Value,
    #[serde(rename = "macro")]
    pub macro_: MacroJobMacro,
}
impl ::std::convert::From<&MacroJob> for MacroJob {
    fn from(value: &MacroJob) -> Self {
        value.clone()
    }
}
impl MacroJob {
    pub fn builder() -> builder::MacroJob {
        Default::default()
    }
}
#[doc = "`MacroJobMacro`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MacroJobMacro(::std::string::String);
impl ::std::ops::Deref for MacroJobMacro {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MacroJobMacro> for ::std::string::String {
    fn from(value: MacroJobMacro) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MacroJobMacro> for MacroJobMacro {
    fn from(value: &MacroJobMacro) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for MacroJobMacro {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MacroJobMacro {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MacroJobMacro {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MacroJobMacro {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MacroJobMacro {
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
#[doc = "    \"kind\","]
#[doc = "    \"map\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"map-key-boolean\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanMapValue\""]
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
pub struct MapKeyBooleanValue {
    pub default: ::std::boxed::Box<ComputedBooleanValue>,
    pub key: ::std::boxed::Box<ComputedStringValue>,
    pub kind: ::serde_json::Value,
    pub map: ComputedBooleanMapValue,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"map\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"map-key-number\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberMapValue\""]
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
pub struct MapKeyNumberValue {
    pub default: ::std::boxed::Box<ComputedNumberValue>,
    pub key: ComputedStringValue,
    pub kind: ::serde_json::Value,
    pub map: ComputedNumberMapValue,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"map\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"map-key-string\""]
#[doc = "    },"]
#[doc = "    \"map\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringMapValue\""]
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
pub struct MapKeyStringValue {
    pub default: ::std::boxed::Box<ComputedStringValue>,
    pub key: ::std::boxed::Box<ComputedStringValue>,
    pub kind: ::serde_json::Value,
    pub map: ComputedStringMapValue,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item-separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"key-separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"map-to-string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "A number value that is the maximum of all numbers in a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Maximum Number List Value\","]
#[doc = "  \"description\": \"A number value that is the maximum of all numbers in a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"max-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct MaxNumberListValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&MaxNumberListValue> for MaxNumberListValue {
    fn from(value: &MaxNumberListValue) -> Self {
        value.clone()
    }
}
impl MaxNumberListValue {
    pub fn builder() -> builder::MaxNumberListValue {
        Default::default()
    }
}
#[doc = "Information about the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Metadata\","]
#[doc = "  \"description\": \"Information about the script.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"source\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authors\": {"]
#[doc = "      \"title\": \"Script Author List\","]
#[doc = "      \"description\": \"All authors of the script, if the authors desire to include it.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"Script Author\","]
#[doc = "        \"description\": \"An author of the script.\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 200,"]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 200,"]
#[doc = "      \"minItems\": 0"]
#[doc = "    },"]
#[doc = "    \"license\": {"]
#[doc = "      \"title\": \"Script License\","]
#[doc = "      \"description\": \"The SPDX description of the script's license.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"spdx\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"copyright\": {"]
#[doc = "          \"title\": \"Copyright\","]
#[doc = "          \"description\": \"The copyright description that accompanies the license type.  This usually relates to the first line of the license text.\\n\""]
#[doc = "        },"]
#[doc = "        \"spdx-id\": {"]
#[doc = "          \"title\": \"SPDX ID\","]
#[doc = "          \"description\": \"The formal SPDX identifier string for this license.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 1000,"]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Script Name\","]
#[doc = "      \"description\": \"The script name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 200,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Script Version\","]
#[doc = "      \"description\": \"The version of the script.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 100,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[doc = "All authors of the script, if the authors desire to include it."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authors: ::std::vec::Vec<ScriptAuthor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub license: ::std::option::Option<ScriptLicense>,
    #[doc = "The script name."]
    pub name: ScriptName,
    pub source: Source,
    #[doc = "The version of the script."]
    pub version: ScriptVersion,
}
impl ::std::convert::From<&Metadata> for Metadata {
    fn from(value: &Metadata) -> Self {
        value.clone()
    }
}
impl Metadata {
    pub fn builder() -> builder::Metadata {
        Default::default()
    }
}
#[doc = "A number value that is the minimum of all numbers in a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Minimum Number List Value\","]
#[doc = "  \"description\": \"A number value that is the minimum of all numbers in a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"min-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct MinNumberListValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&MinNumberListValue> for MinNumberListValue {
    fn from(value: &MinNumberListValue) -> Self {
        value.clone()
    }
}
impl MinNumberListValue {
    pub fn builder() -> builder::MinNumberListValue {
        Default::default()
    }
}
#[doc = "A configuration for a pre-built execution path.  The shell comes with a set of pre-installed modules in the 'shell_lib/modules' path.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Module Job\","]
#[doc = "  \"description\": \"A configuration for a pre-built execution path.  The shell comes with a set of pre-installed modules in the 'shell_lib/modules' path.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"module\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"initial-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/InitialParameters\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"title\": \"Kind\","]
#[doc = "      \"description\": \"Job definition distinguisher\","]
#[doc = "      \"const\": \"module\""]
#[doc = "    },"]
#[doc = "    \"module\": {"]
#[doc = "      \"description\": \"The build implementation for the job.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"runtime-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ModuleJob {
    #[serde(
        rename = "initial-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_parameters: ::std::option::Option<InitialParameters>,
    #[doc = "Job definition distinguisher"]
    pub kind: ::serde_json::Value,
    #[doc = "The build implementation for the job."]
    pub module: ModuleJobModule,
    #[serde(
        rename = "runtime-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub runtime_parameters: ::std::option::Option<NamedParameters>,
}
impl ::std::convert::From<&ModuleJob> for ModuleJob {
    fn from(value: &ModuleJob) -> Self {
        value.clone()
    }
}
impl ModuleJob {
    pub fn builder() -> builder::ModuleJob {
        Default::default()
    }
}
#[doc = "The build implementation for the job."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The build implementation for the job.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ModuleJobModule(::std::string::String);
impl ::std::ops::Deref for ModuleJobModule {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModuleJobModule> for ::std::string::String {
    fn from(value: ModuleJobModule) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ModuleJobModule> for ModuleJobModule {
    fn from(value: &ModuleJobModule) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ModuleJobModule {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModuleJobModule {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ModuleJobModule {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModuleJobModule {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModuleJobModule {
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
#[doc = "A number value that is the modulus of two other number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Modulus Two Values\","]
#[doc = "  \"description\": \"A number value that is the modulus of two other number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"modulus-number\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct ModulusTwoValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedNumberValue>,
    pub right: ::std::boxed::Box<ComputedNumberValue>,
    pub source: Source,
}
impl ::std::convert::From<&ModulusTwoValues> for ModulusTwoValues {
    fn from(value: &ModulusTwoValues) -> Self {
        value.clone()
    }
}
impl ModulusTwoValues {
    pub fn builder() -> builder::ModulusTwoValues {
        Default::default()
    }
}
#[doc = "A number value that is the product of two other number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Multiply Two Values\","]
#[doc = "  \"description\": \"A number value that is the product of two other number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"multiply-number\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct MultiplyTwoValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedNumberValue>,
    pub right: ::std::boxed::Box<ComputedNumberValue>,
    pub source: Source,
}
impl ::std::convert::From<&MultiplyTwoValues> for MultiplyTwoValues {
    fn from(value: &MultiplyTwoValues) -> Self {
        value.clone()
    }
}
impl MultiplyTwoValues {
    pub fn builder() -> builder::MultiplyTwoValues {
        Default::default()
    }
}
#[doc = "The parameters for the job."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Named Parameters\","]
#[doc = "  \"description\": \"The parameters for the job.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"title\": \"Action Parameter\","]
#[doc = "    \"description\": \"A parameter for the job. Must provide exactly one of a value or a value-array.\","]
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
#[doc = "A boolean value that is the result of a logical NAND operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Nand Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical NAND operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"nand-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct NandTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&NandTwoBooleanValues> for NandTwoBooleanValues {
    fn from(value: &NandTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl NandTwoBooleanValues {
    pub fn builder() -> builder::NandTwoBooleanValues {
        Default::default()
    }
}
#[doc = "The low-level script schema for a Native Shell.  It describes in the most basic way the elements necessary to build the code that can compile into the shell program.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Native Shell Low-Level Script Schema\","]
#[doc = "  \"description\": \"The low-level script schema for a Native Shell.  It describes in the most basic way the elements necessary to build the code that can compile into the shell program.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"jobs\","]
#[doc = "    \"meta\","]
#[doc = "    \"schema-version\","]
#[doc = "    \"threads\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"jobs\": {"]
#[doc = "      \"title\": \"Job Collection\","]
#[doc = "      \"description\": \"The set of all configured actions to take during execution.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxItems\": 10000,"]
#[doc = "      \"minItems\": 0,"]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/Job\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"meta\": {"]
#[doc = "      \"title\": \"Metadata\","]
#[doc = "      \"description\": \"Information about the script.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\","]
#[doc = "        \"source\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"authors\": {"]
#[doc = "          \"title\": \"Script Author List\","]
#[doc = "          \"description\": \"All authors of the script, if the authors desire to include it.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"Script Author\","]
#[doc = "            \"description\": \"An author of the script.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 200,"]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"maxItems\": 200,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        },"]
#[doc = "        \"license\": {"]
#[doc = "          \"title\": \"Script License\","]
#[doc = "          \"description\": \"The SPDX description of the script's license.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"spdx\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"copyright\": {"]
#[doc = "              \"title\": \"Copyright\","]
#[doc = "              \"description\": \"The copyright description that accompanies the license type.  This usually relates to the first line of the license text.\\n\""]
#[doc = "            },"]
#[doc = "            \"spdx-id\": {"]
#[doc = "              \"title\": \"SPDX ID\","]
#[doc = "              \"description\": \"The formal SPDX identifier string for this license.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"maxLength\": 1000,"]
#[doc = "              \"minLength\": 1"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"title\": \"Script Name\","]
#[doc = "          \"description\": \"The script name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 200,"]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"title\": \"Script Version\","]
#[doc = "          \"description\": \"The version of the script.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 100,"]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"schema-version\": {"]
#[doc = "      \"title\": \"Schema Version\","]
#[doc = "      \"description\": \"The version of the schema for this LLS.  This is used to ensure compatibility with the parser and runtime.\\n\","]
#[doc = "      \"const\": \"1.0.0\""]
#[doc = "    },"]
#[doc = "    \"threads\": {"]
#[doc = "      \"title\": \"Threads\","]
#[doc = "      \"description\": \"All sequential instructions for execution.  These provide a form of instruction set.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxItems\": 10000,"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"$ref\": \"#/$defs/Thread\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$spdx-license\": \"MIT\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NativeShellLowLevelScriptSchema {
    #[doc = "The set of all configured actions to take during execution."]
    pub jobs: ::std::collections::HashMap<NativeShellLowLevelScriptSchemaJobsKey, Job>,
    pub meta: Metadata,
    #[doc = "The version of the schema for this LLS.  This is used to ensure compatibility with the parser and runtime.\n"]
    #[serde(rename = "schema-version")]
    pub schema_version: ::serde_json::Value,
    #[doc = "All sequential instructions for execution.  These provide a form of instruction set.\n"]
    pub threads: ::std::collections::HashMap<NativeShellLowLevelScriptSchemaThreadsKey, Thread>,
}
impl ::std::convert::From<&NativeShellLowLevelScriptSchema> for NativeShellLowLevelScriptSchema {
    fn from(value: &NativeShellLowLevelScriptSchema) -> Self {
        value.clone()
    }
}
impl NativeShellLowLevelScriptSchema {
    pub fn builder() -> builder::NativeShellLowLevelScriptSchema {
        Default::default()
    }
}
#[doc = "`NativeShellLowLevelScriptSchemaJobsKey`"]
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
pub struct NativeShellLowLevelScriptSchemaJobsKey(::std::string::String);
impl ::std::ops::Deref for NativeShellLowLevelScriptSchemaJobsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellLowLevelScriptSchemaJobsKey> for ::std::string::String {
    fn from(value: NativeShellLowLevelScriptSchemaJobsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellLowLevelScriptSchemaJobsKey>
    for NativeShellLowLevelScriptSchemaJobsKey
{
    fn from(value: &NativeShellLowLevelScriptSchemaJobsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellLowLevelScriptSchemaJobsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellLowLevelScriptSchemaJobsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellLowLevelScriptSchemaJobsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellLowLevelScriptSchemaJobsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellLowLevelScriptSchemaJobsKey {
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
#[doc = "`NativeShellLowLevelScriptSchemaThreadsKey`"]
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
pub struct NativeShellLowLevelScriptSchemaThreadsKey(::std::string::String);
impl ::std::ops::Deref for NativeShellLowLevelScriptSchemaThreadsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NativeShellLowLevelScriptSchemaThreadsKey> for ::std::string::String {
    fn from(value: NativeShellLowLevelScriptSchemaThreadsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NativeShellLowLevelScriptSchemaThreadsKey>
    for NativeShellLowLevelScriptSchemaThreadsKey
{
    fn from(value: &NativeShellLowLevelScriptSchemaThreadsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NativeShellLowLevelScriptSchemaThreadsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NativeShellLowLevelScriptSchemaThreadsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NativeShellLowLevelScriptSchemaThreadsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NativeShellLowLevelScriptSchemaThreadsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NativeShellLowLevelScriptSchemaThreadsKey {
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
#[doc = "A boolean value that is the result of a logical NOR operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Nor Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical NOR operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"nor-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct NorTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&NorTwoBooleanValues> for NorTwoBooleanValues {
    fn from(value: &NorTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl NorTwoBooleanValues {
    pub fn builder() -> builder::NorTwoBooleanValues {
        Default::default()
    }
}
#[doc = "A boolean value that is the negation of another boolean value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Not Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the negation of another boolean value.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"not-boolean\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct NotBooleanValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ::std::boxed::Box<ComputedBooleanValue>,
}
impl ::std::convert::From<&NotBooleanValue> for NotBooleanValue {
    fn from(value: &NotBooleanValue) -> Self {
        value.clone()
    }
}
impl NotBooleanValue {
    pub fn builder() -> builder::NotBooleanValue {
        Default::default()
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"number-to-string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$comment\": \"We may want to expand the format into an explicit definition so it does not require additional parsing.  At the moment, this is the printf format string.\\n\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NumberToStringValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "Describes how to continue running the owning thread."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"On Exit Behavior\","]
#[doc = "  \"description\": \"Describes how to continue running the owning thread.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"by\","]
#[doc = "        \"kind\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"by\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maxValue\": 32767,"]
#[doc = "          \"minValue\": -32768"]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"jump\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"stop-thread\","]
#[doc = "            \"abort-script\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"stop-with\": {"]
#[doc = "          \"$ref\": \"#/$defs/ScriptExit\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum OnExitBehavior {
    Variant0 {
        by: i64,
        kind: ::serde_json::Value,
        source: Source,
    },
    Variant1 {
        kind: OnExitBehaviorVariant1Kind,
        source: Source,
        #[serde(
            rename = "stop-with",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        stop_with: ::std::option::Option<ScriptExit>,
    },
}
impl ::std::convert::From<&Self> for OnExitBehavior {
    fn from(value: &OnExitBehavior) -> Self {
        value.clone()
    }
}
#[doc = "`OnExitBehaviorVariant1Kind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"stop-thread\","]
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
pub enum OnExitBehaviorVariant1Kind {
    #[serde(rename = "stop-thread")]
    StopThread,
    #[serde(rename = "abort-script")]
    AbortScript,
}
impl ::std::convert::From<&Self> for OnExitBehaviorVariant1Kind {
    fn from(value: &OnExitBehaviorVariant1Kind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OnExitBehaviorVariant1Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StopThread => f.write_str("stop-thread"),
            Self::AbortScript => f.write_str("abort-script"),
        }
    }
}
impl ::std::str::FromStr for OnExitBehaviorVariant1Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop-thread" => Ok(Self::StopThread),
            "abort-script" => Ok(Self::AbortScript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OnExitBehaviorVariant1Kind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OnExitBehaviorVariant1Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OnExitBehaviorVariant1Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A boolean value that is the result of a logical OR operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Or Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical OR operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"or-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct OrTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&OrTwoBooleanValues> for OrTwoBooleanValues {
    fn from(value: &OrTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl OrTwoBooleanValues {
    pub fn builder() -> builder::OrTwoBooleanValues {
        Default::default()
    }
}
#[doc = "A reference to a parameter or state placeholder.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Parameter Name\","]
#[doc = "  \"description\": \"A reference to a parameter or state placeholder.\\n\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-zA-Z][a-zA-Z0-9$_]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ParamName(::std::string::String);
impl ::std::ops::Deref for ParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParamName> for ::std::string::String {
    fn from(value: ParamName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ParamName> for ParamName {
    fn from(value: &ParamName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[a-zA-Z][a-zA-Z0-9$_]*$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z][a-zA-Z0-9$_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParamName {
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
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"string\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"$ref\": \"#/$defs/ConstString\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Number\","]
#[doc = "          \"description\": \"A constant number value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"number\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Boolean\","]
#[doc = "          \"description\": \"A constant boolean (true/false) value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
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
#[doc = "            \"kind\","]
#[doc = "            \"source\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"null\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time String List\","]
#[doc = "          \"description\": \"A constant string list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"string-list\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/ConstString\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Number List\","]
#[doc = "          \"description\": \"A constant number list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"number-list\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time Boolean List\","]
#[doc = "          \"description\": \"A constant boolean list value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"boolean-list\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Compile-Time String Map\","]
#[doc = "          \"description\": \"A constant string map value.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"string-map\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0,"]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \".*\": {"]
#[doc = "                  \"$ref\": \"#/$defs/ConstString\""]
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
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"number-map\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0,"]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \".*\": {"]
#[doc = "                  \"$ref\": \"#/$defs/ConstInt\""]
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
#[doc = "            \"kind\","]
#[doc = "            \"source\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"boolean-map\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0,"]
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
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"$ref\": \"#/$defs/ConstString\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Number\","]
#[doc = "      \"description\": \"A constant number value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"number\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Boolean\","]
#[doc = "      \"description\": \"A constant boolean (true/false) value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
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
#[doc = "        \"kind\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"null\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time String List\","]
#[doc = "      \"description\": \"A constant string list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"string-list\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/ConstString\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Number List\","]
#[doc = "      \"description\": \"A constant number list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"number-list\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/ConstInt\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time Boolean List\","]
#[doc = "      \"description\": \"A constant boolean list value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"boolean-list\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Compile-Time String Map\","]
#[doc = "      \"description\": \"A constant string map value.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"string-map\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0,"]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"$ref\": \"#/$defs/ConstString\""]
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
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"number-map\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0,"]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \".*\": {"]
#[doc = "              \"$ref\": \"#/$defs/ConstInt\""]
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
#[doc = "        \"kind\","]
#[doc = "        \"source\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"boolean-map\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0,"]
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
#[serde(tag = "kind", deny_unknown_fields)]
pub enum ParameterValue {
    #[doc = "Compile-Time String\n\nA constant string value."]
    #[serde(rename = "string")]
    String { source: Source, value: ConstString },
    #[doc = "Compile-Time Number\n\nA constant number value."]
    #[serde(rename = "number")]
    Number { source: Source, value: ConstInt },
    #[doc = "Compile-Time Boolean\n\nA constant boolean (true/false) value."]
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
        value: ::std::vec::Vec<ConstString>,
    },
    #[doc = "Compile-Time Number List\n\nA constant number list value."]
    #[serde(rename = "number-list")]
    NumberList {
        source: Source,
        value: ::std::vec::Vec<ConstInt>,
    },
    #[doc = "Compile-Time Boolean List\n\nA constant boolean list value."]
    #[serde(rename = "boolean-list")]
    BooleanList {
        source: Source,
        value: ::std::vec::Vec<bool>,
    },
    #[doc = "Compile-Time String Map\n\nA constant string map value."]
    #[serde(rename = "string-map")]
    StringMap {
        source: Source,
        value: ::std::collections::HashMap<ParameterValueValueKey, ConstString>,
    },
    #[doc = "Compile-Time Number Map\n\nA constant number map value."]
    #[serde(rename = "number-map")]
    NumberMap {
        source: Source,
        value: ::std::collections::HashMap<ParameterValueValueKey, ConstInt>,
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
        if PATTERN.find(value).is_none() {
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
#[doc = "A number value that is the result of raising one number to the power of another."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Power Two Values\","]
#[doc = "  \"description\": \"A number value that is the result of raising one number to the power of another.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base\","]
#[doc = "    \"exponent\","]
#[doc = "    \"kind\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"exponent\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"power-number\""]
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
pub struct PowerTwoValues {
    pub base: ::std::boxed::Box<ComputedNumberValue>,
    pub exponent: ::std::boxed::Box<ComputedNumberValue>,
    pub kind: ::serde_json::Value,
    pub source: Source,
}
impl ::std::convert::From<&PowerTwoValues> for PowerTwoValues {
    fn from(value: &PowerTwoValues) -> Self {
        value.clone()
    }
}
impl PowerTwoValues {
    pub fn builder() -> builder::PowerTwoValues {
        Default::default()
    }
}
#[doc = "A number value that is the product of all numbers in a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Product Number List Value\","]
#[doc = "  \"description\": \"A number value that is the product of all numbers in a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"product-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct ProductNumberListValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&ProductNumberListValue> for ProductNumberListValue {
    fn from(value: &ProductNumberListValue) -> Self {
        value.clone()
    }
}
impl ProductNumberListValue {
    pub fn builder() -> builder::ProductNumberListValue {
        Default::default()
    }
}
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range Boolean List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"range-boolean-list\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeBooleanListValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    pub kind: ::serde_json::Value,
    pub list: ::std::boxed::Box<ComputedBooleanListValue>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
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
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list. If the 'start' field is not given, then it defaults to 0.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range Number List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list. If the 'start' field is not given, then it defaults to 0.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"range-number-list\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
    pub count: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    pub kind: ::serde_json::Value,
    pub list: ::std::boxed::Box<ComputedNumberListValue>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
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
#[doc = "Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Range String List Value\","]
#[doc = "  \"description\": \"Selects a sub-sequence of values from an array.  If the 'count' or 'end' fields are not given, then it defaults to the end of the list.  If the 'start' field is not given, then it defaults to 0.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"list\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"range-string-list\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringListValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeStringListValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    pub kind: ::serde_json::Value,
    pub list: ::std::boxed::Box<ComputedStringListValue>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
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
#[doc = "A number value that is the result of rounding another number to the nearest integer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Round Value\","]
#[doc = "  \"description\": \"A number value that is the result of rounding another number to the nearest integer.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"round-number\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RoundValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ::std::boxed::Box<ComputedNumberValue>,
}
impl ::std::convert::From<&RoundValue> for RoundValue {
    fn from(value: &RoundValue) -> Self {
        value.clone()
    }
}
impl RoundValue {
    pub fn builder() -> builder::RoundValue {
        Default::default()
    }
}
#[doc = "An author of the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Script Author\","]
#[doc = "  \"description\": \"An author of the script.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 200,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScriptAuthor(::std::string::String);
impl ::std::ops::Deref for ScriptAuthor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScriptAuthor> for ::std::string::String {
    fn from(value: ScriptAuthor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScriptAuthor> for ScriptAuthor {
    fn from(value: &ScriptAuthor) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ScriptAuthor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScriptAuthor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScriptAuthor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScriptAuthor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScriptAuthor {
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
#[doc = "The script exit code and optional message."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Script Exit\","]
#[doc = "  \"description\": \"The script exit code and optional message.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"$ref\": \"#/$defs/ExitCode\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A descriptive message for the reason of the exit.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1000,"]
#[doc = "      \"minLength\": 0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"requires\": ["]
#[doc = "    \"source\","]
#[doc = "    \"code\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScriptExit {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExitCode>,
    #[doc = "A descriptive message for the reason of the exit."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<ScriptExitMessage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<Source>,
}
impl ::std::convert::From<&ScriptExit> for ScriptExit {
    fn from(value: &ScriptExit) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ScriptExit {
    fn default() -> Self {
        Self {
            code: Default::default(),
            message: Default::default(),
            source: Default::default(),
        }
    }
}
impl ScriptExit {
    pub fn builder() -> builder::ScriptExit {
        Default::default()
    }
}
#[doc = "A descriptive message for the reason of the exit."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A descriptive message for the reason of the exit.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1000,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScriptExitMessage(::std::string::String);
impl ::std::ops::Deref for ScriptExitMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScriptExitMessage> for ::std::string::String {
    fn from(value: ScriptExitMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScriptExitMessage> for ScriptExitMessage {
    fn from(value: &ScriptExitMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ScriptExitMessage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScriptExitMessage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScriptExitMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScriptExitMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScriptExitMessage {
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
#[doc = "The SPDX description of the script's license."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Script License\","]
#[doc = "  \"description\": \"The SPDX description of the script's license.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"spdx\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"copyright\": {"]
#[doc = "      \"title\": \"Copyright\","]
#[doc = "      \"description\": \"The copyright description that accompanies the license type.  This usually relates to the first line of the license text.\\n\""]
#[doc = "    },"]
#[doc = "    \"spdx-id\": {"]
#[doc = "      \"title\": \"SPDX ID\","]
#[doc = "      \"description\": \"The formal SPDX identifier string for this license.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScriptLicense {
    #[doc = "The copyright description that accompanies the license type.  This usually relates to the first line of the license text.\n"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub copyright: ::std::option::Option<::serde_json::Value>,
    pub spdx: ::serde_json::Value,
    #[doc = "The formal SPDX identifier string for this license."]
    #[serde(
        rename = "spdx-id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spdx_id: ::std::option::Option<SpdxId>,
}
impl ::std::convert::From<&ScriptLicense> for ScriptLicense {
    fn from(value: &ScriptLicense) -> Self {
        value.clone()
    }
}
impl ScriptLicense {
    pub fn builder() -> builder::ScriptLicense {
        Default::default()
    }
}
#[doc = "The script name."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Script Name\","]
#[doc = "  \"description\": \"The script name.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 200,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScriptName(::std::string::String);
impl ::std::ops::Deref for ScriptName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScriptName> for ::std::string::String {
    fn from(value: ScriptName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScriptName> for ScriptName {
    fn from(value: &ScriptName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ScriptName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScriptName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScriptName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScriptName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScriptName {
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
#[doc = "The version of the script."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Script Version\","]
#[doc = "  \"description\": \"The version of the script.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScriptVersion(::std::string::String);
impl ::std::ops::Deref for ScriptVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScriptVersion> for ::std::string::String {
    fn from(value: ScriptVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScriptVersion> for ScriptVersion {
    fn from(value: &ScriptVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ScriptVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScriptVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScriptVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScriptVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScriptVersion {
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
#[doc = "`SendEventPayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendEventPayload {
    Message(EventPayloadMessage),
    Signal(EventPayloadSignal),
}
impl ::std::convert::From<&Self> for SendEventPayload {
    fn from(value: &SendEventPayload) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SendEventPayload {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Message(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Signal(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for SendEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SendEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SendEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for SendEventPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Message(x) => x.fmt(f),
            Self::Signal(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<EventPayloadMessage> for SendEventPayload {
    fn from(value: EventPayloadMessage) -> Self {
        Self::Message(value)
    }
}
impl ::std::convert::From<EventPayloadSignal> for SendEventPayload {
    fn from(value: EventPayloadSignal) -> Self {
        Self::Signal(value)
    }
}
#[doc = "The source from the script.  Every object should include one."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Source\","]
#[doc = "  \"description\": \"The source from the script.  Every object should include one.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"column\","]
#[doc = "    \"file\","]
#[doc = "    \"line\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"column\": {"]
#[doc = "      \"title\": \"Column\","]
#[doc = "      \"description\": \"The column number where the node is defined.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maxValue\": 4294967295,"]
#[doc = "      \"minValue\": 0"]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"title\": \"File\","]
#[doc = "      \"description\": \"The file where the node is defined.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 10000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"line\": {"]
#[doc = "      \"title\": \"Line\","]
#[doc = "      \"description\": \"The line number where the node is defined.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maxValue\": 4294967295,"]
#[doc = "      \"minValue\": 0"]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"title\": \"Text\","]
#[doc = "      \"description\": \"The script's code.  If present, allows better error reporting by showing the related script text.\\n\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 4294967295,"]
#[doc = "      \"minLength\": 0"]
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
    pub file: File,
    #[doc = "The line number where the node is defined."]
    pub line: i64,
    #[doc = "The script's code.  If present, allows better error reporting by showing the related script text.\n"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<Text>,
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
#[doc = "The formal SPDX identifier string for this license."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SPDX ID\","]
#[doc = "  \"description\": \"The formal SPDX identifier string for this license.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SpdxId(::std::string::String);
impl ::std::ops::Deref for SpdxId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SpdxId> for ::std::string::String {
    fn from(value: SpdxId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpdxId> for SpdxId {
    fn from(value: &SpdxId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SpdxId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SpdxId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SpdxId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpdxId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SpdxId {
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
#[doc = "    \"kind\","]
#[doc = "    \"separator\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"split-string\""]
#[doc = "    },"]
#[doc = "    \"maximumSplits\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"separator\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    #[serde(
        rename = "maximumSplits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maximum_splits: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    pub separator: ::std::boxed::Box<ComputedStringValue>,
    pub source: Source,
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
#[doc = "A job that only defines a state, usable by other jobs.  This kind of job does not run anything.  Commonly, scripts use this to store command-line arguments and to provide the user feedback on those arguments and possible environment variable usage.\nThe job generates a shared state by forming a union of the initial-parameters, runtime-parameters, and environment-parameters. When the job runs, it will construct its shared state by loading first the initial-parameters, then the environment-parameters, then the runtime-parameters.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"State Job\","]
#[doc = "  \"description\": \"A job that only defines a state, usable by other jobs.  This kind of job does not run anything.  Commonly, scripts use this to store command-line arguments and to provide the user feedback on those arguments and possible environment variable usage.\\nThe job generates a shared state by forming a union of the initial-parameters, runtime-parameters, and environment-parameters. When the job runs, it will construct its shared state by loading first the initial-parameters, then the environment-parameters, then the runtime-parameters.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"environment-parameters\": {"]
#[doc = "      \"title\": \"Environment Parameter List\","]
#[doc = "      \"description\": \"A list of parameters that come from the environment.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxItems\": 1000,"]
#[doc = "      \"minItems\": 0,"]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \".*\": {"]
#[doc = "          \"title\": \"Environment Parameter\","]
#[doc = "          \"description\": \"A description of a parameter that comes from the environment. This will grant jobs to reference the value from runtime parameters.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"source\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"aliases\": {"]
#[doc = "              \"title\": \"Alias List\","]
#[doc = "              \"description\": \"A list of aliases the user can give to fill in the value.  Useful for allowing an environment variable in the form THE_NAME and the CLI parameters '--the-name' and '-n'.\\n\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"maxLength\": 100,"]
#[doc = "                \"minLength\": 0"]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"title\": \"Description\","]
#[doc = "              \"description\": \"Help text for the parameter. Currently, only allows for a list of text for a single language.  Eventually may allow for multiple langauges.\\n\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"maxLength\": 10000,"]
#[doc = "                \"minLength\": 0"]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            },"]
#[doc = "            \"kind\": {"]
#[doc = "              \"title\": \"Value type\","]
#[doc = "              \"description\": \"Allowed type of value.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"string\","]
#[doc = "                \"string-list\","]
#[doc = "                \"int\","]
#[doc = "                \"int-list\","]
#[doc = "                \"flag\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initial-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/InitialParameters\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"state\""]
#[doc = "    },"]
#[doc = "    \"runtime-parameters\": {"]
#[doc = "      \"$ref\": \"#/$defs/NamedParameters\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StateJob {
    #[doc = "A list of parameters that come from the environment.\n"]
    #[serde(
        rename = "environment-parameters",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub environment_parameters:
        ::std::collections::HashMap<StateJobEnvironmentParametersKey, EnvironmentParameter>,
    #[serde(
        rename = "initial-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_parameters: ::std::option::Option<InitialParameters>,
    pub kind: ::serde_json::Value,
    #[serde(
        rename = "runtime-parameters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub runtime_parameters: ::std::option::Option<NamedParameters>,
}
impl ::std::convert::From<&StateJob> for StateJob {
    fn from(value: &StateJob) -> Self {
        value.clone()
    }
}
impl StateJob {
    pub fn builder() -> builder::StateJob {
        Default::default()
    }
}
#[doc = "`StateJobEnvironmentParametersKey`"]
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
pub struct StateJobEnvironmentParametersKey(::std::string::String);
impl ::std::ops::Deref for StateJobEnvironmentParametersKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StateJobEnvironmentParametersKey> for ::std::string::String {
    fn from(value: StateJobEnvironmentParametersKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StateJobEnvironmentParametersKey> for StateJobEnvironmentParametersKey {
    fn from(value: &StateJobEnvironmentParametersKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StateJobEnvironmentParametersKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new(".*").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StateJobEnvironmentParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StateJobEnvironmentParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StateJobEnvironmentParametersKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StateJobEnvironmentParametersKey {
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
#[doc = "A minimal unit of work to perform, which ties events and jobs together."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Step\","]
#[doc = "  \"description\": \"A minimal unit of work to perform, which ties events and jobs together.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"run\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the step.  Used only for debugging.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"run\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"title\": \"Send Event\","]
#[doc = "          \"description\": \"Send an event to the event broker.  It has either a string message, or an integer code.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"name\","]
#[doc = "            \"payload\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"send-event\""]
#[doc = "            },"]
#[doc = "            \"name\": {"]
#[doc = "              \"$ref\": \"#/$defs/EventRef\""]
#[doc = "            },"]
#[doc = "            \"payload\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Wait For Event\","]
#[doc = "          \"description\": \"Wait for something to send an event *after* this starts listening for events. It can optionally also wait for a matching payload.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"name\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"wait-for-event\""]
#[doc = "            },"]
#[doc = "            \"name\": {"]
#[doc = "              \"$ref\": \"#/$defs/EventRef\""]
#[doc = "            },"]
#[doc = "            \"payload\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Spawn Job\","]
#[doc = "          \"description\": \"Requests the parallel execution of a job, either start it or restart it.  If it's currently running, this will do nothing.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"job\","]
#[doc = "            \"kind\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"job\": {"]
#[doc = "              \"$ref\": \"#/$defs/JobRef\""]
#[doc = "            },"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"spawn-job\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Wait For Job\","]
#[doc = "          \"description\": \"Wait for a job to finish executing.  This will block until the job's execution exits.  If it has already completed, this will continue without waiting.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"exit-behavior\","]
#[doc = "            \"job\","]
#[doc = "            \"kind\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"exit-behavior\": {"]
#[doc = "              \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "            },"]
#[doc = "            \"job\": {"]
#[doc = "              \"$ref\": \"#/$defs/JobRef\""]
#[doc = "            },"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"wait-for-job\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Spawn Thread\","]
#[doc = "          \"description\": \"Requests the parallel execution of a thread, either start it or restart it.  If it's currently running, this will do nothing.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\","]
#[doc = "            \"thread\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"spawn-thread\""]
#[doc = "            },"]
#[doc = "            \"thread\": {"]
#[doc = "              \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Wait For Thread\","]
#[doc = "          \"description\": \"Wait for a thread to finish executing.  This will block until the thread's execution exits.  If it has already completed, this will continue without waiting.\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"exit-behavior\","]
#[doc = "            \"kind\","]
#[doc = "            \"thread\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"exit-behavior\": {"]
#[doc = "              \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "            },"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"wait-for-thread\""]
#[doc = "            },"]
#[doc = "            \"thread\": {"]
#[doc = "              \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Wait For All\","]
#[doc = "          \"description\": \"Wait for all the threads and jobs to finish.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"exit-behavior\","]
#[doc = "            \"jobs\","]
#[doc = "            \"kind\","]
#[doc = "            \"threads\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"exit-behavior\": {"]
#[doc = "              \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "            },"]
#[doc = "            \"jobs\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/JobRef\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            },"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"wait-for-thread\""]
#[doc = "            },"]
#[doc = "            \"threads\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 1000,"]
#[doc = "              \"minItems\": 0"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"title\": \"Do Nothing\","]
#[doc = "          \"description\": \"Perform no action.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"kind\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"kind\": {"]
#[doc = "              \"const\": \"noop\""]
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
pub struct Step {
    #[doc = "The name of the step.  Used only for debugging."]
    pub name: ::std::string::String,
    pub run: StepRun,
    pub source: Source,
}
impl ::std::convert::From<&Step> for Step {
    fn from(value: &Step) -> Self {
        value.clone()
    }
}
impl Step {
    pub fn builder() -> builder::Step {
        Default::default()
    }
}
#[doc = "`StepRun`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Send Event\","]
#[doc = "      \"description\": \"Send an event to the event broker.  It has either a string message, or an integer code.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"name\","]
#[doc = "        \"payload\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"send-event\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"$ref\": \"#/$defs/EventRef\""]
#[doc = "        },"]
#[doc = "        \"payload\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Wait For Event\","]
#[doc = "      \"description\": \"Wait for something to send an event *after* this starts listening for events. It can optionally also wait for a matching payload.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"wait-for-event\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"$ref\": \"#/$defs/EventRef\""]
#[doc = "        },"]
#[doc = "        \"payload\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Spawn Job\","]
#[doc = "      \"description\": \"Requests the parallel execution of a job, either start it or restart it.  If it's currently running, this will do nothing.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"job\","]
#[doc = "        \"kind\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"job\": {"]
#[doc = "          \"$ref\": \"#/$defs/JobRef\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"spawn-job\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Wait For Job\","]
#[doc = "      \"description\": \"Wait for a job to finish executing.  This will block until the job's execution exits.  If it has already completed, this will continue without waiting.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"exit-behavior\","]
#[doc = "        \"job\","]
#[doc = "        \"kind\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"exit-behavior\": {"]
#[doc = "          \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "        },"]
#[doc = "        \"job\": {"]
#[doc = "          \"$ref\": \"#/$defs/JobRef\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"wait-for-job\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Spawn Thread\","]
#[doc = "      \"description\": \"Requests the parallel execution of a thread, either start it or restart it.  If it's currently running, this will do nothing.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\","]
#[doc = "        \"thread\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"spawn-thread\""]
#[doc = "        },"]
#[doc = "        \"thread\": {"]
#[doc = "          \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Wait For Thread\","]
#[doc = "      \"description\": \"Wait for a thread to finish executing.  This will block until the thread's execution exits.  If it has already completed, this will continue without waiting.\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"exit-behavior\","]
#[doc = "        \"kind\","]
#[doc = "        \"thread\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"exit-behavior\": {"]
#[doc = "          \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"wait-for-thread\""]
#[doc = "        },"]
#[doc = "        \"thread\": {"]
#[doc = "          \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Wait For All\","]
#[doc = "      \"description\": \"Wait for all the threads and jobs to finish.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"exit-behavior\","]
#[doc = "        \"jobs\","]
#[doc = "        \"kind\","]
#[doc = "        \"threads\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"exit-behavior\": {"]
#[doc = "          \"$ref\": \"#/$defs/ExitBehavior\""]
#[doc = "        },"]
#[doc = "        \"jobs\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/JobRef\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"wait-for-thread\""]
#[doc = "        },"]
#[doc = "        \"threads\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/ThreadRef\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Do Nothing\","]
#[doc = "      \"description\": \"Perform no action.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"noop\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum StepRun {
    SendEvent {
        kind: ::serde_json::Value,
        name: EventRef,
        payload: SendEventPayload,
    },
    WaitForEvent {
        kind: ::serde_json::Value,
        name: EventRef,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        payload: ::std::option::Option<WaitForEventPayload>,
    },
    SpawnJob {
        job: JobRef,
        kind: ::serde_json::Value,
    },
    WaitForJob {
        #[serde(rename = "exit-behavior")]
        exit_behavior: ExitBehavior,
        job: JobRef,
        kind: ::serde_json::Value,
    },
    SpawnThread {
        kind: ::serde_json::Value,
        thread: ThreadRef,
    },
    WaitForThread {
        #[serde(rename = "exit-behavior")]
        exit_behavior: ExitBehavior,
        kind: ::serde_json::Value,
        thread: ThreadRef,
    },
    WaitForAll {
        #[serde(rename = "exit-behavior")]
        exit_behavior: ExitBehavior,
        jobs: ::std::vec::Vec<JobRef>,
        kind: ::serde_json::Value,
        threads: ::std::vec::Vec<ThreadRef>,
    },
    DoNothing {
        kind: ::serde_json::Value,
    },
}
impl ::std::convert::From<&Self> for StepRun {
    fn from(value: &StepRun) -> Self {
        value.clone()
    }
}
#[doc = "A job that builds a stream connection between two module jobs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Stream Job\","]
#[doc = "  \"description\": \"A job that builds a stream connection between two module jobs.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"from\","]
#[doc = "    \"kind\","]
#[doc = "    \"to\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"from\": {"]
#[doc = "      \"$ref\": \"#/$defs/StreamLocation\""]
#[doc = "    },"]
#[doc = "    \"to\": {"]
#[doc = "      \"$ref\": \"#/$defs/StreamLocation\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StreamJob {
    pub from: StreamLocation,
    pub kind: ::serde_json::Value,
    pub to: StreamLocation,
}
impl ::std::convert::From<&StreamJob> for StreamJob {
    fn from(value: &StreamJob) -> Self {
        value.clone()
    }
}
impl StreamJob {
    pub fn builder() -> builder::StreamJob {
        Default::default()
    }
}
#[doc = "`StreamLocation`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"File Descriptor Stream\","]
#[doc = "      \"description\": \"The file descriptor reference associated with the owning job.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"fd\","]
#[doc = "        \"job\","]
#[doc = "        \"kind\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"fd\": {"]
#[doc = "          \"$ref\": \"#/$defs/FileDescriptor\""]
#[doc = "        },"]
#[doc = "        \"job\": {"]
#[doc = "          \"$ref\": \"#/$defs/JobRef\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"title\": \"Kind\","]
#[doc = "          \"description\": \"Stream location distinguisher\","]
#[doc = "          \"const\": \"fd\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Named Stream\","]
#[doc = "      \"description\": \"The named stream's name in the owning job.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"job\","]
#[doc = "        \"kind\","]
#[doc = "        \"name\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"job\": {"]
#[doc = "          \"$ref\": \"#/$defs/JobRef\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"const\": \"named\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"$ref\": \"#/$defs/StreamName\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum StreamLocation {
    #[doc = "File Descriptor Stream\n\nThe file descriptor reference associated with the owning job."]
    #[serde(rename = "fd")]
    Fd {
        fd: FileDescriptor,
        job: JobRef,
        source: ::serde_json::Value,
    },
    #[doc = "Named Stream\n\nThe named stream's name in the owning job."]
    #[serde(rename = "named")]
    Named {
        job: JobRef,
        name: StreamName,
        source: Source,
    },
}
impl ::std::convert::From<&Self> for StreamLocation {
    fn from(value: &StreamLocation) -> Self {
        value.clone()
    }
}
#[doc = "A module's named stream identifier.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"StreamName\","]
#[doc = "  \"description\": \"A module's named stream identifier.\\n\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StreamName(::std::string::String);
impl ::std::ops::Deref for StreamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StreamName> for ::std::string::String {
    fn from(value: StreamName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StreamName> for StreamName {
    fn from(value: &StreamName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StreamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StreamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StreamName {
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
#[doc = "Extract an internal part of another string.  The 'count' field cannot be used with 'end'.  If 'start' is not given, then it defaults to 0, and if 'end' or 'count' are not given, then it defaults to the end of the string.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Sub-String Value\","]
#[doc = "  \"description\": \"Extract an internal part of another string.  The 'count' field cannot be used with 'end'.  If 'start' is not given, then it defaults to 0, and if 'end' or 'count' are not given, then it defaults to the end of the string.\\n\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"substring\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
    pub kind: ::serde_json::Value,
    pub source: Source,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::std::boxed::Box<ComputedNumberValue>>,
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
#[doc = "A number value that is the difference of two other number values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Subtract Two Values\","]
#[doc = "  \"description\": \"A number value that is the difference of two other number values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"subtract-number\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedNumberValue\""]
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
pub struct SubtractTwoValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedNumberValue>,
    pub right: ::std::boxed::Box<ComputedNumberValue>,
    pub source: Source,
}
impl ::std::convert::From<&SubtractTwoValues> for SubtractTwoValues {
    fn from(value: &SubtractTwoValues) -> Self {
        value.clone()
    }
}
impl SubtractTwoValues {
    pub fn builder() -> builder::SubtractTwoValues {
        Default::default()
    }
}
#[doc = "A number value that is the sum of all numbers in a list."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Sum Number List Value\","]
#[doc = "  \"description\": \"A number value that is the sum of all numbers in a list.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"sum-number-list\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
pub struct SumNumberListValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    pub value: ComputedNumberListValue,
}
impl ::std::convert::From<&SumNumberListValue> for SumNumberListValue {
    fn from(value: &SumNumberListValue) -> Self {
        value.clone()
    }
}
impl SumNumberListValue {
    pub fn builder() -> builder::SumNumberListValue {
        Default::default()
    }
}
#[doc = "The script's code.  If present, allows better error reporting by showing the related script text.\n"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Text\","]
#[doc = "  \"description\": \"The script's code.  If present, allows better error reporting by showing the related script text.\\n\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 4294967295,"]
#[doc = "  \"minLength\": 0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Text(::std::string::String);
impl ::std::ops::Deref for Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Text> for ::std::string::String {
    fn from(value: Text) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Text> for Text {
    fn from(value: &Text) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Text {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4294967295usize {
            return Err("longer than 4294967295 characters".into());
        }
        if value.chars().count() < 0usize {
            return Err("shorter than 0 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Text {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Text {
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
#[doc = "A sequence of steps.  The name comes from the property key in the owning object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Thread\","]
#[doc = "  \"description\": \"A sequence of steps.  The name comes from the property key in the owning object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"main\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {},"]
#[doc = "        {"]
#[doc = "          \"title\": \"Main\","]
#[doc = "          \"description\": \"A thread which the user can include in the list of threads to start when the script begins running.  The 'job' reference field indicates a job whose initial parameters will come from the\\n\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"job\","]
#[doc = "            \"source\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"default\": {"]
#[doc = "              \"title\": \"Default\","]
#[doc = "              \"description\": \"If true, then this will run as a main thread if the user does not declare a main thread to run.\\n\","]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"job\": {"]
#[doc = "              \"$ref\": \"#/$defs/JobRef\""]
#[doc = "            },"]
#[doc = "            \"source\": {"]
#[doc = "              \"$ref\": \"#/$defs/Source\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"steps\": {"]
#[doc = "      \"title\": \"Step List\","]
#[doc = "      \"description\": \"A list of ordered steps that run sequentially.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Step\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 10000,"]
#[doc = "      \"minItems\": 1"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Thread {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub main: ::std::option::Option<ThreadMain>,
    pub source: Source,
    #[doc = "A list of ordered steps that run sequentially."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub steps: ::std::vec::Vec<Step>,
}
impl ::std::convert::From<&Thread> for Thread {
    fn from(value: &Thread) -> Self {
        value.clone()
    }
}
impl Thread {
    pub fn builder() -> builder::Thread {
        Default::default()
    }
}
#[doc = "`ThreadMain`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {},"]
#[doc = "    {"]
#[doc = "      \"title\": \"Main\","]
#[doc = "      \"description\": \"A thread which the user can include in the list of threads to start when the script begins running.  The 'job' reference field indicates a job whose initial parameters will come from the\\n\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"job\","]
#[doc = "        \"source\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"default\": {"]
#[doc = "          \"title\": \"Default\","]
#[doc = "          \"description\": \"If true, then this will run as a main thread if the user does not declare a main thread to run.\\n\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"job\": {"]
#[doc = "          \"$ref\": \"#/$defs/JobRef\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/Source\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum ThreadMain {
    Variant0(::serde_json::Value),
    Variant1 {
        #[doc = "If true, then this will run as a main thread if the user does not declare a main thread to run.\n"]
        #[serde(default)]
        default: bool,
        job: JobRef,
        source: Source,
    },
}
impl ::std::convert::From<&Self> for ThreadMain {
    fn from(value: &ThreadMain) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for ThreadMain {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Reference to a thread.  It must reference one of the keys in the \"thread\" collection."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"JobRef\","]
#[doc = "  \"description\": \"Reference to a thread.  It must reference one of the keys in the \\\"thread\\\" collection.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ThreadRef(::std::string::String);
impl ::std::ops::Deref for ThreadRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ThreadRef> for ::std::string::String {
    fn from(value: ThreadRef) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ThreadRef> for ThreadRef {
    fn from(value: &ThreadRef) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ThreadRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ThreadRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ThreadRef {
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
#[doc = "Trims the surrounding whitespace or other characters from a string."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Trim String Value\","]
#[doc = "  \"description\": \"Trims the surrounding whitespace or other characters from a string.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"trim-string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
#[doc = "    },"]
#[doc = "    \"trim-chars\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedStringValue\""]
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
pub struct TrimStringValue {
    pub kind: ::serde_json::Value,
    pub source: Source,
    #[serde(
        rename = "trim-chars",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trim_chars: ::std::option::Option<::std::boxed::Box<ComputedStringValue>>,
    pub value: ::std::boxed::Box<ComputedStringValue>,
}
impl ::std::convert::From<&TrimStringValue> for TrimStringValue {
    fn from(value: &TrimStringValue) -> Self {
        value.clone()
    }
}
impl TrimStringValue {
    pub fn builder() -> builder::TrimStringValue {
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"union-boolean-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"union-number-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"union-string-map\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/Source\""]
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
    pub kind: ::serde_json::Value,
    pub source: Source,
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
#[doc = "Allowed type of value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Value type\","]
#[doc = "  \"description\": \"Allowed type of value.\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"string\","]
#[doc = "    \"string-list\","]
#[doc = "    \"int\","]
#[doc = "    \"int-list\","]
#[doc = "    \"flag\""]
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
pub enum ValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "string-list")]
    StringList,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "int-list")]
    IntList,
    #[serde(rename = "flag")]
    Flag,
}
impl ::std::convert::From<&Self> for ValueType {
    fn from(value: &ValueType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => f.write_str("string"),
            Self::StringList => f.write_str("string-list"),
            Self::Int => f.write_str("int"),
            Self::IntList => f.write_str("int-list"),
            Self::Flag => f.write_str("flag"),
        }
    }
}
impl ::std::str::FromStr for ValueType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            "string-list" => Ok(Self::StringList),
            "int" => Ok(Self::Int),
            "int-list" => Ok(Self::IntList),
            "flag" => Ok(Self::Flag),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ValueType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ValueType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WaitForEventPayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/EventPayloadMessage\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/EventPayloadSignal\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum WaitForEventPayload {
    Message(EventPayloadMessage),
    Signal(EventPayloadSignal),
}
impl ::std::convert::From<&Self> for WaitForEventPayload {
    fn from(value: &WaitForEventPayload) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for WaitForEventPayload {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Message(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Signal(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for WaitForEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WaitForEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WaitForEventPayload {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for WaitForEventPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Message(x) => x.fmt(f),
            Self::Signal(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<EventPayloadMessage> for WaitForEventPayload {
    fn from(value: EventPayloadMessage) -> Self {
        Self::Message(value)
    }
}
impl ::std::convert::From<EventPayloadSignal> for WaitForEventPayload {
    fn from(value: EventPayloadSignal) -> Self {
        Self::Signal(value)
    }
}
#[doc = "A boolean value that is the result of a logical XNOR operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Xnor Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical XNOR operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"xnor-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct XnorTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&XnorTwoBooleanValues> for XnorTwoBooleanValues {
    fn from(value: &XnorTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl XnorTwoBooleanValues {
    pub fn builder() -> builder::XnorTwoBooleanValues {
        Default::default()
    }
}
#[doc = "A boolean value that is the result of a logical exclusive OR operation on two boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Xor Boolean Value\","]
#[doc = "  \"description\": \"A boolean value that is the result of a logical exclusive OR operation on two boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"xor-boolean\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"$ref\": \"#/$defs/ComputedBooleanValue\""]
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
pub struct XorTwoBooleanValues {
    pub kind: ::serde_json::Value,
    pub left: ::std::boxed::Box<ComputedBooleanValue>,
    pub right: ::std::boxed::Box<ComputedBooleanValue>,
    pub source: Source,
}
impl ::std::convert::From<&XorTwoBooleanValues> for XorTwoBooleanValues {
    fn from(value: &XorTwoBooleanValues) -> Self {
        value.clone()
    }
}
impl XorTwoBooleanValues {
    pub fn builder() -> builder::XorTwoBooleanValues {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AbsValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberValue, ::std::string::String>,
    }
    impl ::std::default::Default for AbsValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AbsValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AbsValue> for super::AbsValue {
        type Error = super::error::ConversionError;
        fn try_from(value: AbsValue) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AbsValue> for AbsValue {
        fn from(value: super::AbsValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
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
    pub struct AddTwoValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for AddTwoValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl AddTwoValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<AddTwoValues> for super::AddTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::AddTwoValues> for AddTwoValues {
        fn from(value: super::AddTwoValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AndTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for AndTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl AndTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<AndTwoBooleanValues> for super::AndTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AndTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::AndTwoBooleanValues> for AndTwoBooleanValues {
        fn from(value: super::AndTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AverageNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for AverageNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AverageNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AverageNumberListValue> for super::AverageNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AverageNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AverageNumberListValue> for AverageNumberListValue {
        fn from(value: super::AverageNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        true_string: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        value: ::std::result::Result<super::ComputedBooleanValue, ::std::string::String>,
    }
    impl ::std::default::Default for BooleanToStringValue {
        fn default() -> Self {
            Self {
                false_string: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                true_string: Ok(Default::default()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                true_string: value.true_string?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::BooleanToStringValue> for BooleanToStringValue {
        fn from(value: super::BooleanToStringValue) -> Self {
            Self {
                false_string: Ok(value.false_string),
                kind: Ok(value.kind),
                source: Ok(value.source),
                true_string: Ok(value.true_string),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CeilValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CeilValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl CeilValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CeilValue> for super::CeilValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CeilValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::CeilValue> for CeilValue {
        fn from(value: super::CeilValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantBooleanListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantBooleanListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanListValue> for ConstantBooleanListValue {
        fn from(value: super::ConstantBooleanListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
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
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanMapValue> for ConstantBooleanMapValue {
        fn from(value: super::ConstantBooleanMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantBooleanValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantBooleanValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantBooleanValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantBooleanValue> for ConstantBooleanValue {
        fn from(value: super::ConstantBooleanValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantNumberListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberListValue> for ConstantNumberListValue {
        fn from(value: super::ConstantNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::collections::HashMap<
                super::ConstantNumberMapValueValueKey,
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantNumberMapValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ConstantNumberMapValueValueKey,
                    ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberMapValue> for ConstantNumberMapValue {
        fn from(value: super::ConstantNumberMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantNumberValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantNumberValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantNumberValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantNumberValue> for ConstantNumberValue {
        fn from(value: super::ConstantNumberValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::vec::Vec<super::ConstantStringListValueValueItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstantStringListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringListValue> for ConstantStringListValue {
        fn from(value: super::ConstantStringListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
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
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringMapValue> for ConstantStringMapValue {
        fn from(value: super::ConstantStringMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantStringValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ConstantStringValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantStringValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConstantStringValue> for ConstantStringValue {
        fn from(value: super::ConstantStringValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DivideTwoValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for DivideTwoValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl DivideTwoValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<DivideTwoValues> for super::DivideTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DivideTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::DivideTwoValues> for DivideTwoValues {
        fn from(value: super::DivideTwoValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnvironmentParameter {
        aliases:
            ::std::result::Result<::std::vec::Vec<super::AliasListItem>, ::std::string::String>,
        description:
            ::std::result::Result<::std::vec::Vec<super::DescriptionItem>, ::std::string::String>,
        kind: ::std::result::Result<super::ValueType, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for EnvironmentParameter {
        fn default() -> Self {
            Self {
                aliases: Ok(Default::default()),
                description: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl EnvironmentParameter {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AliasListItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DescriptionItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ValueType>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    impl ::std::convert::TryFrom<EnvironmentParameter> for super::EnvironmentParameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnvironmentParameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                description: value.description?,
                kind: value.kind?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::EnvironmentParameter> for EnvironmentParameter {
        fn from(value: super::EnvironmentParameter) -> Self {
            Self {
                aliases: Ok(value.aliases),
                description: Ok(value.description),
                kind: Ok(value.kind),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExitBehavior {
        default_behavior: ::std::result::Result<
            ::std::option::Option<super::OnExitBehavior>,
            ::std::string::String,
        >,
        exit_code_behaviors:
            ::std::result::Result<::std::vec::Vec<super::ExitCodeBehavior>, ::std::string::String>,
        never_started: ::std::result::Result<
            ::std::option::Option<super::OnExitBehavior>,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::option::Option<super::Source>, ::std::string::String>,
    }
    impl ::std::default::Default for ExitBehavior {
        fn default() -> Self {
            Self {
                default_behavior: Ok(Default::default()),
                exit_code_behaviors: Ok(Default::default()),
                never_started: Ok(Default::default()),
                source: Ok(Default::default()),
            }
        }
    }
    impl ExitBehavior {
        pub fn default_behavior<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OnExitBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_behavior = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for default_behavior: {}",
                    e
                )
            });
            self
        }
        pub fn exit_code_behaviors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExitCodeBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.exit_code_behaviors = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for exit_code_behaviors: {}",
                    e
                )
            });
            self
        }
        pub fn never_started<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OnExitBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.never_started = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for never_started: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Source>>,
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
                default_behavior: value.default_behavior?,
                exit_code_behaviors: value.exit_code_behaviors?,
                never_started: value.never_started?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ExitBehavior> for ExitBehavior {
        fn from(value: super::ExitBehavior) -> Self {
            Self {
                default_behavior: Ok(value.default_behavior),
                exit_code_behaviors: Ok(value.exit_code_behaviors),
                never_started: Ok(value.never_started),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExitCodeBehavior {
        behavior: ::std::result::Result<super::OnExitBehavior, ::std::string::String>,
        max_code:
            ::std::result::Result<::std::option::Option<super::ExitCode>, ::std::string::String>,
        min_code:
            ::std::result::Result<::std::option::Option<super::ExitCode>, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ExitCodeBehavior {
        fn default() -> Self {
            Self {
                behavior: Err("no value supplied for behavior".to_string()),
                max_code: Ok(Default::default()),
                min_code: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl ExitCodeBehavior {
        pub fn behavior<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OnExitBehavior>,
            T::Error: ::std::fmt::Display,
        {
            self.behavior = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for behavior: {}", e));
            self
        }
        pub fn max_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExitCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_code: {}", e));
            self
        }
        pub fn min_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExitCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_code: {}", e));
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
    impl ::std::convert::TryFrom<ExitCodeBehavior> for super::ExitCodeBehavior {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExitCodeBehavior,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                behavior: value.behavior?,
                max_code: value.max_code?,
                min_code: value.min_code?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ExitCodeBehavior> for ExitCodeBehavior {
        fn from(value: super::ExitCodeBehavior) -> Self {
            Self {
                behavior: Ok(value.behavior),
                max_code: Ok(value.max_code),
                min_code: Ok(value.min_code),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FloorValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FloorValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl FloorValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FloorValue> for super::FloorValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FloorValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FloorValue> for FloorValue {
        fn from(value: super::FloorValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineJob {
        code: ::std::result::Result<super::InlineJobCode, ::std::string::String>,
        initial_parameters: ::std::result::Result<
            ::std::option::Option<super::InitialParameters>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        runtime_parameters: ::std::result::Result<
            ::std::option::Option<super::NamedParameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InlineJob {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                initial_parameters: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                runtime_parameters: Ok(Default::default()),
            }
        }
    }
    impl InlineJob {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InlineJobCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn initial_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InitialParameters>>,
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn runtime_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NamedParameters>>,
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
    }
    impl ::std::convert::TryFrom<InlineJob> for super::InlineJob {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InlineJob,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                initial_parameters: value.initial_parameters?,
                kind: value.kind?,
                runtime_parameters: value.runtime_parameters?,
            })
        }
    }
    impl ::std::convert::From<super::InlineJob> for InlineJob {
        fn from(value: super::InlineJob) -> Self {
            Self {
                code: Ok(value.code),
                initial_parameters: Ok(value.initial_parameters),
                kind: Ok(value.kind),
                runtime_parameters: Ok(value.runtime_parameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Job {
        def: ::std::result::Result<super::JobDef, ::std::string::String>,
        rerunnable: ::std::result::Result<bool, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for Job {
        fn default() -> Self {
            Self {
                def: Err("no value supplied for def".to_string()),
                rerunnable: Ok(super::defaults::default_bool::<true>()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl Job {
        pub fn def<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobDef>,
            T::Error: ::std::fmt::Display,
        {
            self.def = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for def: {}", e));
            self
        }
        pub fn rerunnable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.rerunnable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rerunnable: {}", e));
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
    impl ::std::convert::TryFrom<Job> for super::Job {
        type Error = super::error::ConversionError;
        fn try_from(value: Job) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                def: value.def?,
                rerunnable: value.rerunnable?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::Job> for Job {
        fn from(value: super::Job) -> Self {
            Self {
                def: Ok(value.def),
                rerunnable: Ok(value.rerunnable),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<super::ComputedBooleanListValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexBooleanValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    }
    impl ::std::convert::TryFrom<ListIndexBooleanValue> for super::ListIndexBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexBooleanValue> for ListIndexBooleanValue {
        fn from(value: super::ListIndexBooleanValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexNumberValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
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
    }
    impl ::std::convert::TryFrom<ListIndexNumberValue> for super::ListIndexNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexNumberValue> for ListIndexNumberValue {
        fn from(value: super::ListIndexNumberValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<super::ComputedStringListValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ListIndexStringValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                index: Err("no value supplied for index".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    }
    impl ::std::convert::TryFrom<ListIndexStringValue> for super::ListIndexStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListIndexStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                index: value.index?,
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ListIndexStringValue> for ListIndexStringValue {
        fn from(value: super::ListIndexStringValue) -> Self {
            Self {
                default: Ok(value.default),
                index: Ok(value.index),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToStringValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        separator: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedStringListValue, ::std::string::String>,
    }
    impl ::std::default::Default for ListToStringValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                separator: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ListToStringValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
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
                kind: value.kind?,
                separator: value.separator?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ListToStringValue> for ListToStringValue {
        fn from(value: super::ListToStringValue) -> Self {
            Self {
                kind: Ok(value.kind),
                separator: Ok(value.separator),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanListValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanListValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupBooleanListValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupBooleanListValue> for super::LookupBooleanListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanListValue> for LookupBooleanListValue {
        fn from(value: super::LookupBooleanListValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanMapValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanMapValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupBooleanMapValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupBooleanMapValue> for super::LookupBooleanMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanMapValue> for LookupBooleanMapValue {
        fn from(value: super::LookupBooleanMapValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupBooleanValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupBooleanValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupBooleanValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupBooleanValue> for super::LookupBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupBooleanValue> for LookupBooleanValue {
        fn from(value: super::LookupBooleanValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberListValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberListValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupNumberListValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupNumberListValue> for super::LookupNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberListValue> for LookupNumberListValue {
        fn from(value: super::LookupNumberListValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberMapValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberMapValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupNumberMapValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupNumberMapValue> for super::LookupNumberMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberMapValue> for LookupNumberMapValue {
        fn from(value: super::LookupNumberMapValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupNumberValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupNumberValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupNumberValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupNumberValue> for super::LookupNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupNumberValue> for LookupNumberValue {
        fn from(value: super::LookupNumberValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStringListValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStringListValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupStringListValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupStringListValue> for super::LookupStringListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringListValue> for LookupStringListValue {
        fn from(value: super::LookupStringListValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStringMapValue {
        job: ::std::result::Result<super::JobRef, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStringMapValue {
        fn default() -> Self {
            Self {
                job: Err("no value supplied for job".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupStringMapValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobRef>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
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
    impl ::std::convert::TryFrom<LookupStringMapValue> for super::LookupStringMapValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringMapValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringMapValue> for LookupStringMapValue {
        fn from(value: super::LookupStringMapValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LookupStringValue {
        job: ::std::result::Result<::std::option::Option<super::JobRef>, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        name: ::std::result::Result<super::ParamName, ::std::string::String>,
        node: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for LookupStringValue {
        fn default() -> Self {
            Self {
                job: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                node: Err("no value supplied for node".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl LookupStringValue {
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JobRef>>,
            T::Error: ::std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ParamName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
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
    }
    impl ::std::convert::TryFrom<LookupStringValue> for super::LookupStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LookupStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                job: value.job?,
                kind: value.kind?,
                name: value.name?,
                node: value.node?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::LookupStringValue> for LookupStringValue {
        fn from(value: super::LookupStringValue) -> Self {
            Self {
                job: Ok(value.job),
                kind: Ok(value.kind),
                name: Ok(value.name),
                node: Ok(value.node),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MacroJob {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        macro_: ::std::result::Result<super::MacroJobMacro, ::std::string::String>,
    }
    impl ::std::default::Default for MacroJob {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                macro_: Err("no value supplied for macro_".to_string()),
            }
        }
    }
    impl MacroJob {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn macro_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MacroJobMacro>,
            T::Error: ::std::fmt::Display,
        {
            self.macro_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for macro_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MacroJob> for super::MacroJob {
        type Error = super::error::ConversionError;
        fn try_from(value: MacroJob) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                macro_: value.macro_?,
            })
        }
    }
    impl ::std::convert::From<super::MacroJob> for MacroJob {
        fn from(value: super::MacroJob) -> Self {
            Self {
                kind: Ok(value.kind),
                macro_: Ok(value.macro_),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        map: ::std::result::Result<super::ComputedBooleanMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyBooleanValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    }
    impl ::std::convert::TryFrom<MapKeyBooleanValue> for super::MapKeyBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                kind: value.kind?,
                map: value.map?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyBooleanValue> for MapKeyBooleanValue {
        fn from(value: super::MapKeyBooleanValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                kind: Ok(value.kind),
                map: Ok(value.map),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        map: ::std::result::Result<super::ComputedNumberMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyNumberValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    }
    impl ::std::convert::TryFrom<MapKeyNumberValue> for super::MapKeyNumberValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyNumberValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                kind: value.kind?,
                map: value.map?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyNumberValue> for MapKeyNumberValue {
        fn from(value: super::MapKeyNumberValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                kind: Ok(value.kind),
                map: Ok(value.map),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        map: ::std::result::Result<super::ComputedStringMapValue, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for MapKeyStringValue {
        fn default() -> Self {
            Self {
                default: Err("no value supplied for default".to_string()),
                key: Err("no value supplied for key".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                map: Err("no value supplied for map".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    }
    impl ::std::convert::TryFrom<MapKeyStringValue> for super::MapKeyStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapKeyStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                key: value.key?,
                kind: value.kind?,
                map: value.map?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::MapKeyStringValue> for MapKeyStringValue {
        fn from(value: super::MapKeyStringValue) -> Self {
            Self {
                default: Ok(value.default),
                key: Ok(value.key),
                kind: Ok(value.kind),
                map: Ok(value.map),
                source: Ok(value.source),
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedStringMapValue, ::std::string::String>,
    }
    impl ::std::default::Default for MapToStringValue {
        fn default() -> Self {
            Self {
                item_separator: Ok(Default::default()),
                key_separator: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MapToStringValue> for MapToStringValue {
        fn from(value: super::MapToStringValue) -> Self {
            Self {
                item_separator: Ok(value.item_separator),
                key_separator: Ok(value.key_separator),
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MaxNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for MaxNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl MaxNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MaxNumberListValue> for super::MaxNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MaxNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MaxNumberListValue> for MaxNumberListValue {
        fn from(value: super::MaxNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Metadata {
        authors: ::std::result::Result<::std::vec::Vec<super::ScriptAuthor>, ::std::string::String>,
        license: ::std::result::Result<
            ::std::option::Option<super::ScriptLicense>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::ScriptName, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        version: ::std::result::Result<super::ScriptVersion, ::std::string::String>,
    }
    impl ::std::default::Default for Metadata {
        fn default() -> Self {
            Self {
                authors: Ok(Default::default()),
                license: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Metadata {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ScriptAuthor>>,
            T::Error: ::std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authors: {}", e));
            self
        }
        pub fn license<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ScriptLicense>>,
            T::Error: ::std::fmt::Display,
        {
            self.license = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for license: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScriptName>,
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
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScriptVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Metadata> for super::Metadata {
        type Error = super::error::ConversionError;
        fn try_from(value: Metadata) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authors: value.authors?,
                license: value.license?,
                name: value.name?,
                source: value.source?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Metadata> for Metadata {
        fn from(value: super::Metadata) -> Self {
            Self {
                authors: Ok(value.authors),
                license: Ok(value.license),
                name: Ok(value.name),
                source: Ok(value.source),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MinNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for MinNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl MinNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MinNumberListValue> for super::MinNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MinNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MinNumberListValue> for MinNumberListValue {
        fn from(value: super::MinNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModuleJob {
        initial_parameters: ::std::result::Result<
            ::std::option::Option<super::InitialParameters>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        module: ::std::result::Result<super::ModuleJobModule, ::std::string::String>,
        runtime_parameters: ::std::result::Result<
            ::std::option::Option<super::NamedParameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModuleJob {
        fn default() -> Self {
            Self {
                initial_parameters: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                module: Err("no value supplied for module".to_string()),
                runtime_parameters: Ok(Default::default()),
            }
        }
    }
    impl ModuleJob {
        pub fn initial_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InitialParameters>>,
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn module<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModuleJobModule>,
            T::Error: ::std::fmt::Display,
        {
            self.module = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for module: {}", e));
            self
        }
        pub fn runtime_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NamedParameters>>,
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
    }
    impl ::std::convert::TryFrom<ModuleJob> for super::ModuleJob {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModuleJob,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                initial_parameters: value.initial_parameters?,
                kind: value.kind?,
                module: value.module?,
                runtime_parameters: value.runtime_parameters?,
            })
        }
    }
    impl ::std::convert::From<super::ModuleJob> for ModuleJob {
        fn from(value: super::ModuleJob) -> Self {
            Self {
                initial_parameters: Ok(value.initial_parameters),
                kind: Ok(value.kind),
                module: Ok(value.module),
                runtime_parameters: Ok(value.runtime_parameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModulusTwoValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for ModulusTwoValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl ModulusTwoValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<ModulusTwoValues> for super::ModulusTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModulusTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ModulusTwoValues> for ModulusTwoValues {
        fn from(value: super::ModulusTwoValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MultiplyTwoValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for MultiplyTwoValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl MultiplyTwoValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<MultiplyTwoValues> for super::MultiplyTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MultiplyTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::MultiplyTwoValues> for MultiplyTwoValues {
        fn from(value: super::MultiplyTwoValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NandTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for NandTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl NandTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<NandTwoBooleanValues> for super::NandTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NandTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::NandTwoBooleanValues> for NandTwoBooleanValues {
        fn from(value: super::NandTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NativeShellLowLevelScriptSchema {
        jobs: ::std::result::Result<
            ::std::collections::HashMap<super::NativeShellLowLevelScriptSchemaJobsKey, super::Job>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::Metadata, ::std::string::String>,
        schema_version: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        threads: ::std::result::Result<
            ::std::collections::HashMap<
                super::NativeShellLowLevelScriptSchemaThreadsKey,
                super::Thread,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NativeShellLowLevelScriptSchema {
        fn default() -> Self {
            Self {
                jobs: Err("no value supplied for jobs".to_string()),
                meta: Err("no value supplied for meta".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                threads: Err("no value supplied for threads".to_string()),
            }
        }
    }
    impl NativeShellLowLevelScriptSchema {
        pub fn jobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::NativeShellLowLevelScriptSchemaJobsKey,
                    super::Job,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.jobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jobs: {}", e));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Metadata>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {}", e));
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
        pub fn threads<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::NativeShellLowLevelScriptSchemaThreadsKey,
                    super::Thread,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.threads = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threads: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NativeShellLowLevelScriptSchema>
        for super::NativeShellLowLevelScriptSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NativeShellLowLevelScriptSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jobs: value.jobs?,
                meta: value.meta?,
                schema_version: value.schema_version?,
                threads: value.threads?,
            })
        }
    }
    impl ::std::convert::From<super::NativeShellLowLevelScriptSchema>
        for NativeShellLowLevelScriptSchema
    {
        fn from(value: super::NativeShellLowLevelScriptSchema) -> Self {
            Self {
                jobs: Ok(value.jobs),
                meta: Ok(value.meta),
                schema_version: Ok(value.schema_version),
                threads: Ok(value.threads),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NorTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for NorTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl NorTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<NorTwoBooleanValues> for super::NorTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NorTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::NorTwoBooleanValues> for NorTwoBooleanValues {
        fn from(value: super::NorTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotBooleanValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NotBooleanValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl NotBooleanValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NotBooleanValue> for super::NotBooleanValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotBooleanValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::NotBooleanValue> for NotBooleanValue {
        fn from(value: super::NotBooleanValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NumberToStringValue {
        format: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NumberToStringValue {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::NumberToStringValue> for NumberToStringValue {
        fn from(value: super::NumberToStringValue) -> Self {
            Self {
                format: Ok(value.format),
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for OrTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl OrTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<OrTwoBooleanValues> for super::OrTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::OrTwoBooleanValues> for OrTwoBooleanValues {
        fn from(value: super::OrTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
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
    pub struct PowerTwoValues {
        base: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        exponent: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for PowerTwoValues {
        fn default() -> Self {
            Self {
                base: Err("no value supplied for base".to_string()),
                exponent: Err("no value supplied for exponent".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl PowerTwoValues {
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn exponent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.exponent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exponent: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
    impl ::std::convert::TryFrom<PowerTwoValues> for super::PowerTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PowerTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base: value.base?,
                exponent: value.exponent?,
                kind: value.kind?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::PowerTwoValues> for PowerTwoValues {
        fn from(value: super::PowerTwoValues) -> Self {
            Self {
                base: Ok(value.base),
                exponent: Ok(value.exponent),
                kind: Ok(value.kind),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for ProductNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ProductNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProductNumberListValue> for super::ProductNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProductNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ProductNumberListValue> for ProductNumberListValue {
        fn from(value: super::ProductNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RangeBooleanListValue {
        count: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RangeBooleanListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
            }
        }
    }
    impl RangeBooleanListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
            })
        }
    }
    impl ::std::convert::From<super::RangeBooleanListValue> for RangeBooleanListValue {
        fn from(value: super::RangeBooleanListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RangeNumberListValue {
        count: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RangeNumberListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
            }
        }
    }
    impl RangeNumberListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
            })
        }
    }
    impl ::std::convert::From<super::RangeNumberListValue> for RangeNumberListValue {
        fn from(value: super::RangeNumberListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RangeStringListValue {
        count: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        list: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringListValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RangeStringListValue {
        fn default() -> Self {
            Self {
                count: Ok(Default::default()),
                end: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                list: Err("no value supplied for list".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
            }
        }
    }
    impl RangeStringListValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
                kind: value.kind?,
                list: value.list?,
                source: value.source?,
                start: value.start?,
            })
        }
    }
    impl ::std::convert::From<super::RangeStringListValue> for RangeStringListValue {
        fn from(value: super::RangeStringListValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                kind: Ok(value.kind),
                list: Ok(value.list),
                source: Ok(value.source),
                start: Ok(value.start),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RoundValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RoundValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl RoundValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RoundValue> for super::RoundValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RoundValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::RoundValue> for RoundValue {
        fn from(value: super::RoundValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScriptExit {
        code: ::std::result::Result<::std::option::Option<super::ExitCode>, ::std::string::String>,
        message: ::std::result::Result<
            ::std::option::Option<super::ScriptExitMessage>,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::option::Option<super::Source>, ::std::string::String>,
    }
    impl ::std::default::Default for ScriptExit {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                message: Ok(Default::default()),
                source: Ok(Default::default()),
            }
        }
    }
    impl ScriptExit {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExitCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ScriptExitMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Source>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ScriptExit> for super::ScriptExit {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScriptExit,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ScriptExit> for ScriptExit {
        fn from(value: super::ScriptExit) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScriptLicense {
        copyright: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        spdx: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        spdx_id: ::std::result::Result<::std::option::Option<super::SpdxId>, ::std::string::String>,
    }
    impl ::std::default::Default for ScriptLicense {
        fn default() -> Self {
            Self {
                copyright: Ok(Default::default()),
                spdx: Err("no value supplied for spdx".to_string()),
                spdx_id: Ok(Default::default()),
            }
        }
    }
    impl ScriptLicense {
        pub fn copyright<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.copyright = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for copyright: {}", e));
            self
        }
        pub fn spdx<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.spdx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spdx: {}", e));
            self
        }
        pub fn spdx_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SpdxId>>,
            T::Error: ::std::fmt::Display,
        {
            self.spdx_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spdx_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ScriptLicense> for super::ScriptLicense {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScriptLicense,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                copyright: value.copyright?,
                spdx: value.spdx?,
                spdx_id: value.spdx_id?,
            })
        }
    }
    impl ::std::convert::From<super::ScriptLicense> for ScriptLicense {
        fn from(value: super::ScriptLicense) -> Self {
            Self {
                copyright: Ok(value.copyright),
                spdx: Ok(value.spdx),
                spdx_id: Ok(value.spdx_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Source {
        column: ::std::result::Result<i64, ::std::string::String>,
        file: ::std::result::Result<super::File, ::std::string::String>,
        line: ::std::result::Result<i64, ::std::string::String>,
        text: ::std::result::Result<::std::option::Option<super::Text>, ::std::string::String>,
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
            T: ::std::convert::TryInto<super::File>,
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
            T: ::std::convert::TryInto<::std::option::Option<super::Text>>,
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
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        maximum_splits: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        separator: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SplitStringValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                maximum_splits: Ok(Default::default()),
                separator: Err("no value supplied for separator".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SplitStringValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn maximum_splits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_splits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum_splits: {}", e));
            self
        }
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
                kind: value.kind?,
                maximum_splits: value.maximum_splits?,
                separator: value.separator?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SplitStringValue> for SplitStringValue {
        fn from(value: super::SplitStringValue) -> Self {
            Self {
                kind: Ok(value.kind),
                maximum_splits: Ok(value.maximum_splits),
                separator: Ok(value.separator),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StateJob {
        environment_parameters: ::std::result::Result<
            ::std::collections::HashMap<
                super::StateJobEnvironmentParametersKey,
                super::EnvironmentParameter,
            >,
            ::std::string::String,
        >,
        initial_parameters: ::std::result::Result<
            ::std::option::Option<super::InitialParameters>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        runtime_parameters: ::std::result::Result<
            ::std::option::Option<super::NamedParameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StateJob {
        fn default() -> Self {
            Self {
                environment_parameters: Ok(Default::default()),
                initial_parameters: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                runtime_parameters: Ok(Default::default()),
            }
        }
    }
    impl StateJob {
        pub fn environment_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::StateJobEnvironmentParametersKey,
                    super::EnvironmentParameter,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.environment_parameters = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for environment_parameters: {}",
                    e
                )
            });
            self
        }
        pub fn initial_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InitialParameters>>,
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn runtime_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NamedParameters>>,
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
    }
    impl ::std::convert::TryFrom<StateJob> for super::StateJob {
        type Error = super::error::ConversionError;
        fn try_from(value: StateJob) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                environment_parameters: value.environment_parameters?,
                initial_parameters: value.initial_parameters?,
                kind: value.kind?,
                runtime_parameters: value.runtime_parameters?,
            })
        }
    }
    impl ::std::convert::From<super::StateJob> for StateJob {
        fn from(value: super::StateJob) -> Self {
            Self {
                environment_parameters: Ok(value.environment_parameters),
                initial_parameters: Ok(value.initial_parameters),
                kind: Ok(value.kind),
                runtime_parameters: Ok(value.runtime_parameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Step {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        run: ::std::result::Result<super::StepRun, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for Step {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                run: Err("no value supplied for run".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl Step {
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
            T: ::std::convert::TryInto<super::StepRun>,
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
    impl ::std::convert::TryFrom<Step> for super::Step {
        type Error = super::error::ConversionError;
        fn try_from(value: Step) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                run: value.run?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::Step> for Step {
        fn from(value: super::Step) -> Self {
            Self {
                name: Ok(value.name),
                run: Ok(value.run),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamJob {
        from: ::std::result::Result<super::StreamLocation, ::std::string::String>,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        to: ::std::result::Result<super::StreamLocation, ::std::string::String>,
    }
    impl ::std::default::Default for StreamJob {
        fn default() -> Self {
            Self {
                from: Err("no value supplied for from".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                to: Err("no value supplied for to".to_string()),
            }
        }
    }
    impl StreamJob {
        pub fn from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamLocation>,
            T::Error: ::std::fmt::Display,
        {
            self.from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamLocation>,
            T::Error: ::std::fmt::Display,
        {
            self.to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StreamJob> for super::StreamJob {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamJob,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                from: value.from?,
                kind: value.kind?,
                to: value.to?,
            })
        }
    }
    impl ::std::convert::From<super::StreamJob> for StreamJob {
        fn from(value: super::StreamJob) -> Self {
            Self {
                from: Ok(value.from),
                kind: Ok(value.kind),
                to: Ok(value.to),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubStringValue {
        count: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        end: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        start: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            ::std::string::String,
        >,
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
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                start: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SubStringValue {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                ::std::option::Option<::std::boxed::Box<super::ComputedNumberValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                start: value.start?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SubStringValue> for SubStringValue {
        fn from(value: super::SubStringValue) -> Self {
            Self {
                count: Ok(value.count),
                end: Ok(value.end),
                kind: Ok(value.kind),
                source: Ok(value.source),
                start: Ok(value.start),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubtractTwoValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedNumberValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for SubtractTwoValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl SubtractTwoValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedNumberValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<SubtractTwoValues> for super::SubtractTwoValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubtractTwoValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::SubtractTwoValues> for SubtractTwoValues {
        fn from(value: super::SubtractTwoValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SumNumberListValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        value: ::std::result::Result<super::ComputedNumberListValue, ::std::string::String>,
    }
    impl ::std::default::Default for SumNumberListValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SumNumberListValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
            T: ::std::convert::TryInto<super::ComputedNumberListValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SumNumberListValue> for super::SumNumberListValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SumNumberListValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SumNumberListValue> for SumNumberListValue {
        fn from(value: super::SumNumberListValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Thread {
        main:
            ::std::result::Result<::std::option::Option<super::ThreadMain>, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        steps: ::std::result::Result<::std::vec::Vec<super::Step>, ::std::string::String>,
    }
    impl ::std::default::Default for Thread {
        fn default() -> Self {
            Self {
                main: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                steps: Ok(Default::default()),
            }
        }
    }
    impl Thread {
        pub fn main<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ThreadMain>>,
            T::Error: ::std::fmt::Display,
        {
            self.main = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for main: {}", e));
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
        pub fn steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Step>>,
            T::Error: ::std::fmt::Display,
        {
            self.steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for steps: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Thread> for super::Thread {
        type Error = super::error::ConversionError;
        fn try_from(value: Thread) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                main: value.main?,
                source: value.source?,
                steps: value.steps?,
            })
        }
    }
    impl ::std::convert::From<super::Thread> for Thread {
        fn from(value: super::Thread) -> Self {
            Self {
                main: Ok(value.main),
                source: Ok(value.source),
                steps: Ok(value.steps),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrimStringValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        trim_chars: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::boxed::Box<super::ComputedStringValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TrimStringValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                trim_chars: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl TrimStringValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
        pub fn trim_chars<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::boxed::Box<super::ComputedStringValue>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.trim_chars = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trim_chars: {}", e));
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
    impl ::std::convert::TryFrom<TrimStringValue> for super::TrimStringValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrimStringValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                source: value.source?,
                trim_chars: value.trim_chars?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::TrimStringValue> for TrimStringValue {
        fn from(value: super::TrimStringValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                trim_chars: Ok(value.trim_chars),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionBooleanMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedBooleanMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionBooleanMapValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionBooleanMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionBooleanMapValue> for UnionBooleanMapValue {
        fn from(value: super::UnionBooleanMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionNumberMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedNumberMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionNumberMapValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionNumberMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionNumberMapValue> for UnionNumberMapValue {
        fn from(value: super::UnionNumberMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionStringMapValue {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        source: ::std::result::Result<super::Source, ::std::string::String>,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ComputedStringMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UnionStringMapValue {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl UnionStringMapValue {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
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
                kind: value.kind?,
                source: value.source?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::UnionStringMapValue> for UnionStringMapValue {
        fn from(value: super::UnionStringMapValue) -> Self {
            Self {
                kind: Ok(value.kind),
                source: Ok(value.source),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct XnorTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for XnorTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl XnorTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<XnorTwoBooleanValues> for super::XnorTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: XnorTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::XnorTwoBooleanValues> for XnorTwoBooleanValues {
        fn from(value: super::XnorTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct XorTwoBooleanValues {
        kind: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        left: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        right: ::std::result::Result<
            ::std::boxed::Box<super::ComputedBooleanValue>,
            ::std::string::String,
        >,
        source: ::std::result::Result<super::Source, ::std::string::String>,
    }
    impl ::std::default::Default for XorTwoBooleanValues {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                left: Err("no value supplied for left".to_string()),
                right: Err("no value supplied for right".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl XorTwoBooleanValues {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {}", e));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<super::ComputedBooleanValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {}", e));
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
    impl ::std::convert::TryFrom<XorTwoBooleanValues> for super::XorTwoBooleanValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: XorTwoBooleanValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                left: value.left?,
                right: value.right?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::XorTwoBooleanValues> for XorTwoBooleanValues {
        fn from(value: super::XorTwoBooleanValues) -> Self {
            Self {
                kind: Ok(value.kind),
                left: Ok(value.left),
                right: Ok(value.right),
                source: Ok(value.source),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
}
