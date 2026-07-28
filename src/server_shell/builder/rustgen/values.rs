//! Handle turning the model's computed values into Rust code.

use std::ops::Deref;

use super::helpers;
use crate::{
    server_shell::{
        builder::{collect, errors},
        lls,
    },
    shell_lib::structure,
};

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

pub fn conv_initial_parameter(
    _issues: &errors::ScriptIssues,
    _col: &collect::Collector,
    value: &lls::model::Parameter,
) -> Result<Option<(structure::meta::ValueType, String)>, ()> {
    match &value.value {
        lls::model::InitialParameterValue::String { source: _, value } => Ok(Some((
            structure::meta::ValueType::String,
            helpers::as_rust_string(&value.clone().into()),
        ))),
        lls::model::InitialParameterValue::Number { source: _, value } => {
            // Numeric conversion: just use to_string.
            Ok(Some((
                structure::meta::ValueType::Float,
                helpers::as_rust_float(*value.deref()),
            )))
        }
        lls::model::InitialParameterValue::Boolean { source: _, value } => Ok(Some((
            structure::meta::ValueType::Boolean,
            helpers::as_rust_bool(*value).to_string(),
        ))),
        lls::model::InitialParameterValue::Null { source: _ } => Ok(None),
        lls::model::InitialParameterValue::StringList { source: _, value } => Ok(Some((
            structure::meta::ValueType::StringList,
            helpers::as_rust_string_list(value),
        ))),
        lls::model::InitialParameterValue::NumberList { source: _, value } => Ok(Some((
            structure::meta::ValueType::FloatList,
            helpers::as_rust_float_list(value),
        ))),
        lls::model::InitialParameterValue::BooleanList { source: _, value } => Ok(Some((
            structure::meta::ValueType::BooleanList,
            helpers::as_rust_bool_list(value),
        ))),
        lls::model::InitialParameterValue::StringMap { source: _, value } => Ok(Some((
            structure::meta::ValueType::StringMap,
            helpers::as_rust_map(value, |v| helpers::as_rust_string(v)),
        ))),
        lls::model::InitialParameterValue::NumberMap { source: _, value } => Ok(Some((
            structure::meta::ValueType::FloatMap,
            helpers::as_rust_map(value, |v| helpers::as_rust_float(*v.deref())),
        ))),
        lls::model::InitialParameterValue::BooleanMap { source: _, value } => Ok(Some((
            structure::meta::ValueType::BooleanMap,
            helpers::as_rust_map(value, |v| helpers::as_rust_bool(*v).to_string()),
        ))),
        lls::model::InitialParameterValue::StringListMap { source: _, value } => Ok(Some((
            structure::meta::ValueType::StringListMap,
            helpers::as_rust_map(value, |v| helpers::as_rust_string_list(v)),
        ))),
        lls::model::InitialParameterValue::StringMapList { source: _, value } => Ok(Some((
            structure::meta::ValueType::StringMapList,
            helpers::as_rust_list(value, |v| {
                helpers::as_rust_map(v, |s| helpers::as_rust_string(s))
            }),
        ))),
    }
}

enum ConvAP<'a> {
    StrBit(&'static str),
    Computed(
        (
            Option<structure::meta::ValueType>, // Expected kind
            &'a lls::model::ComputedValue,
        ),
    ),
}

pub fn conv_action_parameter<'a, 'b, 'c>(
    issues: &'a errors::ScriptIssues,
    col: &'b collect::Collector,
    value: &'c lls::model::ActionParameter,
) -> Result<Option<(structure::meta::ValueType, String)>, ()> {
    let mut ret_type = match get_value_type(&value.value) {
        Some(t) => t,
        None => {
            return Ok(None);
        }
    };
    let mut ret_str = String::new();
    let mut stack: Vec<ConvAP<'c>> = vec![ConvAP::Computed((Some(ret_type.clone()), &value.value))];

    loop {
        let next_v = match stack.pop() {
            Some(v) => v,
            None => {
                return Ok(Some((ret_type.clone(), ret_str)));
            }
        };
        match next_v {
            ConvAP::StrBit(s) => {
                // Append the bit of text.
                ret_str.push_str(s);
            }
            ConvAP::Computed((exp_type, val)) => match val {
                lls::model::ComputedValue::LookupStringValue(lookup_string_value) => {
                    // TODO may not want early exit on error here.
                    let (job_ref, field_type) = issues.consume(col.get_job_state_field(
                        &lookup_string_value.source,
                        &lookup_string_value.job,
                        &lookup_string_value.name,
                        &exp_type,
                    ))?;
                    match field_type {
                        None => {
                            // The action parameter can't be decided yet.
                            // Assume it's valid.
                        }
                        Some(t) => {}
                    }
                    ret_str.push_str(
                        format!(
                            "self.runtime.{}.{}",
                            super::names::x(job_ref),
                            lookup_string_value.name
                        )
                        .as_str(),
                    );
                }
                lls::model::ComputedValue::ListIndexStringValue(list_index_string_value) => todo!(),
                lls::model::ComputedValue::MapKeyStringValue(map_key_string_value) => todo!(),
                lls::model::ComputedValue::SubStringValue(sub_string_value) => todo!(),
                lls::model::ComputedValue::TrimStringValue(trim_string_value) => todo!(),
                lls::model::ComputedValue::NumberToStringValue(number_to_string_value) => todo!(),
                lls::model::ComputedValue::BooleanToStringValue(boolean_to_string_value) => todo!(),
                lls::model::ComputedValue::ListToStringValue(list_to_string_value) => todo!(),
                lls::model::ComputedValue::MapToStringValue(map_to_string_value) => todo!(),
                lls::model::ComputedValue::ConstantStringValue(constant_string_value) => todo!(),

                lls::model::ComputedValue::LookupNumberValue(lookup_number_value) => todo!(),
                lls::model::ComputedValue::AddTwoValues(add_two_values) => todo!(),
                lls::model::ComputedValue::SubtractTwoValues(subtract_two_values) => todo!(),
                lls::model::ComputedValue::MultiplyTwoValues(multiply_two_values) => todo!(),
                lls::model::ComputedValue::DivideTwoValues(divide_two_values) => todo!(),
                lls::model::ComputedValue::ModulusTwoValues(modulus_two_values) => todo!(),
                lls::model::ComputedValue::PowerTwoValues(power_two_values) => todo!(),
                lls::model::ComputedValue::RoundValue(round_value) => todo!(),
                lls::model::ComputedValue::FloorValue(floor_value) => todo!(),
                lls::model::ComputedValue::CeilValue(ceil_value) => todo!(),
                lls::model::ComputedValue::AbsValue(abs_value) => todo!(),
                lls::model::ComputedValue::SumNumberListValue(sum_number_list_value) => todo!(),
                lls::model::ComputedValue::ProductNumberListValue(product_number_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::AverageNumberListValue(average_number_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::MinNumberListValue(min_number_list_value) => todo!(),
                lls::model::ComputedValue::MaxNumberListValue(max_number_list_value) => todo!(),
                lls::model::ComputedValue::ListIndexNumberValue(list_index_number_value) => todo!(),
                lls::model::ComputedValue::MapKeyNumberValue(map_key_number_value) => todo!(),
                lls::model::ComputedValue::CollectionSizeNumberValue(
                    collection_size_number_value,
                ) => todo!(),
                lls::model::ComputedValue::StringLeftIndexNumberValue(
                    string_left_index_number_value,
                ) => todo!(),
                lls::model::ComputedValue::StringRightIndexNumberValue(
                    string_right_index_number_value,
                ) => todo!(),
                lls::model::ComputedValue::StringListIndexNumberValue(
                    string_list_index_number_value,
                ) => todo!(),
                lls::model::ComputedValue::NumberListIndexNumberValue(
                    number_list_index_number_value,
                ) => todo!(),
                lls::model::ComputedValue::BooleanListIndexNumberValue(
                    boolean_list_index_number_value,
                ) => todo!(),
                lls::model::ComputedValue::ConstantNumberValue(constant_number_value) => todo!(),
                lls::model::ComputedValue::LookupBooleanValue(lookup_boolean_value) => todo!(),
                lls::model::ComputedValue::AndTwoBooleanValues(and_two_boolean_values) => todo!(),
                lls::model::ComputedValue::OrTwoBooleanValues(or_two_boolean_values) => todo!(),
                lls::model::ComputedValue::NotBooleanValue(not_boolean_value) => todo!(),
                lls::model::ComputedValue::XorTwoBooleanValues(xor_two_boolean_values) => todo!(),
                lls::model::ComputedValue::NandTwoBooleanValues(nand_two_boolean_values) => todo!(),
                lls::model::ComputedValue::NorTwoBooleanValues(nor_two_boolean_values) => todo!(),
                lls::model::ComputedValue::XnorTwoBooleanValues(xnor_two_boolean_values) => todo!(),
                lls::model::ComputedValue::ListIndexBooleanValue(list_index_boolean_value) => {
                    todo!()
                }
                lls::model::ComputedValue::MapKeyBooleanValue(map_key_boolean_value) => todo!(),
                lls::model::ComputedValue::MapContainsKeyBooleanValue(
                    map_contains_key_boolean_value,
                ) => todo!(),
                lls::model::ComputedValue::ListContainsIndexBooleanValue(
                    list_contains_index_boolean_value,
                ) => todo!(),
                lls::model::ComputedValue::StringEqualBooleanValue(string_equal_boolean_value) => {
                    todo!()
                }
                lls::model::ComputedValue::NumberEqualBooleanValue(number_equal_boolean_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantBooleanValue(constant_boolean_value) => todo!(),
                lls::model::ComputedValue::LookupStringListValue(lookup_string_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::SplitStringValue(split_string_value) => todo!(),
                lls::model::ComputedValue::RangeStringListValue(range_string_list_value) => todo!(),
                lls::model::ComputedValue::StringListMapKeyValue(string_list_map_key_value) => {
                    todo!()
                }
                lls::model::ComputedValue::MapKeysStringListValue(map_keys_string_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringListValue(constant_string_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::LookupNumberListValue(lookup_number_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::RangeNumberListValue(range_number_list_value) => todo!(),
                lls::model::ComputedValue::ConstantNumberListValue(constant_number_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::LookupBooleanListValue(lookup_boolean_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::RangeBooleanListValue(range_boolean_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantBooleanListValue(
                    constant_boolean_list_value,
                ) => todo!(),
                lls::model::ComputedValue::LookupStringMapValue(lookup_string_map_value) => todo!(),
                lls::model::ComputedValue::UnionStringMapValue(union_string_map_value) => todo!(),
                lls::model::ComputedValue::StringMapListIndexValue(string_map_list_index_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringMapValue(constant_string_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::LookupNumberMapValue(lookup_number_map_value) => todo!(),
                lls::model::ComputedValue::UnionNumberMapValue(union_number_map_value) => todo!(),
                lls::model::ComputedValue::ConstantNumberMapValue(constant_number_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::LookupBooleanMapValue(lookup_boolean_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::UnionBooleanMapValue(union_boolean_map_value) => todo!(),
                lls::model::ComputedValue::ConstantBooleanMapValue(constant_boolean_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::LookupStringListMapValue(
                    lookup_string_list_map_value,
                ) => todo!(),
                lls::model::ComputedValue::UnionStringListMapValue(union_string_list_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringListMapValue(
                    constant_string_list_map_value,
                ) => todo!(),
                lls::model::ComputedValue::LookupStringMapListValue(
                    lookup_string_map_list_value,
                ) => todo!(),
                lls::model::ComputedValue::RangeStringMapListValue(range_string_map_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringMapListValue(
                    constant_string_map_list_value,
                ) => todo!(),
                lls::model::ComputedValue::ComputedNullValue(computed_null_value) => todo!(),
            },
        }
    }
}

pub fn get_value_type(value: &lls::model::ComputedValue) -> Option<structure::meta::ValueType> {
    match value {
        lls::model::ComputedValue::ComputedNullValue(_) => None,
        lls::model::ComputedValue::LookupStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::ListIndexStringValue(_) => {
            Some(structure::meta::ValueType::String)
        }
        lls::model::ComputedValue::MapKeyStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::SubStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::TrimStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::NumberToStringValue(_) => {
            Some(structure::meta::ValueType::String)
        }
        lls::model::ComputedValue::BooleanToStringValue(_) => {
            Some(structure::meta::ValueType::String)
        }
        lls::model::ComputedValue::ListToStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::MapToStringValue(_) => Some(structure::meta::ValueType::String),
        lls::model::ComputedValue::ConstantStringValue(_) => {
            Some(structure::meta::ValueType::String)
        }

        lls::model::ComputedValue::LookupNumberValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::AddTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::SubtractTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::MultiplyTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::DivideTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::ModulusTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::PowerTwoValues(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::RoundValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::FloorValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::CeilValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::AbsValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::SumNumberListValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::ProductNumberListValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::AverageNumberListValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::MinNumberListValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::MaxNumberListValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::ListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::MapKeyNumberValue(_) => Some(structure::meta::ValueType::Float),
        lls::model::ComputedValue::CollectionSizeNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::StringLeftIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::StringRightIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::StringListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::NumberListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::BooleanListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        lls::model::ComputedValue::ConstantNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }

        lls::model::ComputedValue::LookupBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::AndTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::OrTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::NotBooleanValue(_) => Some(structure::meta::ValueType::Boolean),
        lls::model::ComputedValue::XorTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::NandTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::NorTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::XnorTwoBooleanValues(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::ListIndexBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::MapKeyBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::MapContainsKeyBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::ListContainsIndexBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::StringEqualBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::NumberEqualBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        lls::model::ComputedValue::ConstantBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }

        lls::model::ComputedValue::LookupStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        lls::model::ComputedValue::SplitStringValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        lls::model::ComputedValue::RangeStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        lls::model::ComputedValue::StringListMapKeyValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        lls::model::ComputedValue::MapKeysStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        lls::model::ComputedValue::ConstantStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }

        lls::model::ComputedValue::LookupNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }
        lls::model::ComputedValue::RangeNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }
        lls::model::ComputedValue::ConstantNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }

        lls::model::ComputedValue::LookupBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }
        lls::model::ComputedValue::RangeBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }
        lls::model::ComputedValue::ConstantBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }

        lls::model::ComputedValue::LookupStringMapValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }
        lls::model::ComputedValue::UnionStringMapValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }
        lls::model::ComputedValue::StringMapListIndexValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }
        lls::model::ComputedValue::ConstantStringMapValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }

        lls::model::ComputedValue::LookupNumberMapValue(_) => {
            Some(structure::meta::ValueType::FloatMap)
        }
        lls::model::ComputedValue::UnionNumberMapValue(_) => {
            Some(structure::meta::ValueType::FloatMap)
        }
        lls::model::ComputedValue::ConstantNumberMapValue(_) => {
            Some(structure::meta::ValueType::FloatMap)
        }

        lls::model::ComputedValue::LookupBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }
        lls::model::ComputedValue::UnionBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }
        lls::model::ComputedValue::ConstantBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }

        lls::model::ComputedValue::LookupStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }
        lls::model::ComputedValue::UnionStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }
        lls::model::ComputedValue::ConstantStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }

        lls::model::ComputedValue::LookupStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
        lls::model::ComputedValue::RangeStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
        lls::model::ComputedValue::ConstantStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
    }
}
