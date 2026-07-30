//SPDX:MIT

use crate::shell_lib::structure;

/// Does the expected type match the discovered kind?
/// Though not required, the actual value may be passed in to perform extra validation checks.
pub fn is_type_match(
    expected: structure::meta::ValueType,
    optional: bool,
    kind: Option<structure::meta::ValueType>,
    value: Option<String>,
) -> bool {
    match kind {
        None if optional => true,
        None => false, // !optional
        Some(kind) => match kind {
            structure::meta::ValueType::String => {
                // Strings might be enum types.
                if let structure::meta::ValueType::Enum(choices) = expected {
                    match value {
                        Some(value) => choices.contains(&value),

                        // Should be an "indeterminante" value, because this can't make a solid call.
                        // But it looks close enough, and later things like compilers may take over the
                        // checking.
                        None => true,
                    }
                } else {
                    expected == kind
                }
            }

            // Not applicable: value types are never enums; only fields are.
            structure::meta::ValueType::Enum(_) => panic!("bad state: values can't be enum types"),

            _ => kind == expected,
        },
    }
}
