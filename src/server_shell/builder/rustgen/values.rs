//! Handle turning the model's computed values into Rust code.

use std::{collections::HashMap, ops::Deref};

use super::{helpers, lookup};
use crate::{
    server_shell::{
        builder::{collect, errors},
        lls::{self, convert::*},
    },
    shell_lib::structure,
};

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

enum ConvAP {
    StrBit(&'static str),
    StringBit(String),
    Computed(
        (
            Option<structure::meta::ValueType>, // Expected kind
            // This copies the value.  A reference would be better here,
            // but there exist some places where we need to construct this,
            // which makes management of the value difficult.
            lls::model::ComputedValue,
        ),
    ),
}

/// Create the runtime (action) parameter value.
/// This happens within the `run()` function of the JobRunner instance.
/// At this point, the '&self' reference contains a 'self.runtime'
/// (Arc<Runtime>) value for looking up state field values.
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
    let mut stack: Vec<ConvAP> = vec![ConvAP::Computed((
        Some(ret_type.clone()),
        value.value.clone(),
    ))];

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
            ConvAP::StringBit(s) => {
                // Append the bit of text.
                ret_str.push_str(s.as_str());
            }
            ConvAP::Computed((exp_type, val)) => match val {
                lls::model::ComputedValue::ComputedNullValue(_) => {
                    // Receivers allow this value for list values and map values
                    // and optional top-level values.
                    // TODO encode optional into the expected type.
                    ret_str.push_str("None");
                }

                // ----------------------------------------------------
                // String
                // TODO have all of these generate a String object, rather than a &str.
                //      That will force conformity in how the code uses the generated value.
                lls::model::ComputedValue::LookupStringValue(lookup_string_value) => {
                    // TODO ensure exp_type is a string value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_string_value.source,
                            &"self.runtime".to_string(),
                            &lookup_string_value.job,
                            &lookup_string_value.name,
                            &structure::meta::ValueType::String,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
                lls::model::ComputedValue::ListIndexStringValue(list_index_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // This needs to take the format: (<list string>).get((<index value>).trunc()).or(<default>)
                    stack.push(ConvAP::StrBit(")"));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::String),
                        list_index_string_value.default.as_ref().into(),
                    )));
                    stack.push(ConvAP::StrBit(").trunc()).or("));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::Float),
                        list_index_string_value.index.as_ref().into(),
                    )));
                    stack.push(ConvAP::StrBit(").get(("));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::StringList),
                        list_index_string_value.list.as_ref().into(),
                    )));
                    ret_str.push('(');
                }
                lls::model::ComputedValue::MapKeyStringValue(map_key_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // This takes the format: (<string map>).get(<string>).or(<default>)
                    stack.push(ConvAP::StrBit(")"));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::String),
                        map_key_string_value.default.as_ref().into(),
                    )));
                    stack.push(ConvAP::StrBit(").or("));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::String),
                        map_key_string_value.key.as_ref().into(),
                    )));
                    stack.push(ConvAP::StrBit(").get("));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::StringMap),
                        (&map_key_string_value.map).into(),
                    )));
                    ret_str.push('(');
                }
                lls::model::ComputedValue::SubStringValue(sub_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // This uses the substring helper function to do the right behavior.
                    if let Some(end) = sub_string_value.end {
                        // It includes an 'end' section, which is the last index to consume.
                        if sub_string_value.count.is_some() {
                            issues.add_err(errors::BuilderError::InvalidLLS(
                                errors::ErrorDetails {
                                    source: sub_string_value.source.into(),
                                    message: "cannot specify both 'end' and 'count'".into(),
                                    related: Vec::new(),
                                },
                            ));
                            ret_str.push_str("panic!(\"end + count\")");
                            continue;
                        }
                        match sub_string_value.start {
                            Some(start) => {
                                // Call out:
                                // crate::shell_lib::helpers::values::sub_string_start_end(
                                //   (<value>).to_string(), (<start>).trunc() as usize, (<end>).trunc() as i64)
                                stack.push(ConvAP::StrBit(").trunc() as i64)"));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    end.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").trunc() as usize, ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    start.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").to_string(), ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::String),
                                    sub_string_value.value.as_ref().into(),
                                )));
                                ret_str.push_str(
                                    "crate::shell_lib::helpers::values::sub_string_start_end((",
                                );
                            }
                            None => {
                                // Call out:
                                // crate::shell_lib::helpers::values::sub_string_end(
                                //   (<value>).to_string(), (<end>).trunc() as i64)
                                stack.push(ConvAP::StrBit(").trunc() as i64)"));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    end.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").to_string(), ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::String),
                                    sub_string_value.value.as_ref().into(),
                                )));
                                ret_str.push_str(
                                    "crate::shell_lib::helpers::values::sub_string_end((",
                                );
                            }
                        }
                    } else if let Some(count) = sub_string_value.count {
                        match sub_string_value.start {
                            Some(start) => {
                                // Call out:
                                // crate::shell_lib::helpers::values::sub_string_start_count(
                                //   (<value>).to_string(), (<start>) as usize, (<count>) as i64)
                                stack.push(ConvAP::StrBit(").trunc() as i64)"));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    count.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").trunc() as usize, ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    start.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").to_string(), ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::String),
                                    sub_string_value.value.as_ref().into(),
                                )));
                                ret_str.push_str(
                                    "crate::shell_lib::helpers::values::sub_string_start_count((",
                                );
                            }
                            None => {
                                // Call out:
                                // crate::shell_lib::helpers::values::sub_string_end(
                                //   (<value>).to_string(), (<count>).trunc() as i64)
                                // (in this case, end acts the same as count)
                                stack.push(ConvAP::StrBit(").trunc() as i64)"));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    count.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").to_string(), ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::String),
                                    sub_string_value.value.as_ref().into(),
                                )));
                                ret_str.push_str(
                                    "crate::shell_lib::helpers::values::sub_string_end((",
                                );
                            }
                        }
                    } else {
                        match sub_string_value.start {
                            Some(start) => {
                                // Call out:
                                // crate::shell_lib::helpers::values::sub_string_start(
                                //   (<value>).to_string(), (<start>).trunc() as usize)
                                stack.push(ConvAP::StrBit(").trunc() as usize)"));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::Float),
                                    start.as_ref().into(),
                                )));
                                stack.push(ConvAP::StrBit(").to_string(), ("));
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::String),
                                    sub_string_value.value.as_ref().into(),
                                )));
                                ret_str.push_str(
                                    "crate::shell_lib::helpers::values::sub_string_start((",
                                );
                            }
                            None => {
                                // Just use the embedded string.
                                stack.push(ConvAP::Computed((
                                    Some(structure::meta::ValueType::StringMap),
                                    sub_string_value.value.as_ref().into(),
                                )));
                            }
                        }
                    }
                }
                lls::model::ComputedValue::TrimStringValue(trim_string_value) => {
                    // TODO ensure exp_type is a string value.
                    match trim_string_value.trim_chars {
                        Some(chars) => {
                            // Format with:
                            // (<value>).to_string().trim_matches(
                            //   (<chars>).to_string().chars().collect::<Vec<char>>().as_slice())
                            stack.push(ConvAP::StrBit(
                                ").to_string().chars().collect::<Vec<char>>().as_slice())",
                            ));
                            stack.push(ConvAP::Computed((
                                Some(structure::meta::ValueType::String),
                                chars.as_ref().into(),
                            )));
                            stack.push(ConvAP::StrBit(").to_string().trim_matches("));
                            stack.push(ConvAP::Computed((
                                Some(structure::meta::ValueType::String),
                                trim_string_value.value.as_ref().into(),
                            )));
                            ret_str.push('(');
                        }
                        None => {
                            // Whitespace.
                            // Format: (<value>).to_string().trim()
                            stack.push(ConvAP::StrBit(").to_string().trim()"));
                            stack.push(ConvAP::Computed((
                                Some(structure::meta::ValueType::String),
                                trim_string_value.value.as_ref().into(),
                            )));
                            ret_str.push('(');
                        }
                    }
                }
                lls::model::ComputedValue::NumberToStringValue(number_to_string_value) => {
                    // TODO ensure exp_type is a string value.
                    match number_to_string_value.format {
                        Some(format) => {
                            // Format: format!(<format string>, <value>)
                            stack.push(ConvAP::StrBit(")"));
                            stack.push(ConvAP::Computed((
                                Some(structure::meta::ValueType::Float),
                                number_to_string_value.value.as_ref().into(),
                            )));
                            stack.push(ConvAP::StringBit(format!(
                                "{}, ",
                                helpers::as_rust_str(&format.value)
                            )));
                            ret_str.push_str("format!(");
                        }
                        None => {
                            // Format: (<value>).to_string()
                            stack.push(ConvAP::StrBit(").to_string()"));
                            stack.push(ConvAP::Computed((
                                Some(structure::meta::ValueType::Float),
                                number_to_string_value.value.as_ref().into(),
                            )));
                            ret_str.push('(');
                        }
                    }
                }
                lls::model::ComputedValue::BooleanToStringValue(boolean_to_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // format: match <value> { true => "true" or <true>, false => "false" or <false> }
                    stack.push(ConvAP::StrBit(",}"));
                    stack.push(match boolean_to_string_value.false_string {
                        Some(s) => ConvAP::Computed((
                            Some(structure::meta::ValueType::String),
                            s.as_ref().into(),
                        )),
                        None => ConvAP::StringBit(helpers::as_rust_str(&"false".to_string())),
                    });
                    stack.push(ConvAP::StrBit(", false => "));
                    stack.push(match boolean_to_string_value.true_string {
                        Some(s) => ConvAP::Computed((
                            Some(structure::meta::ValueType::String),
                            s.as_ref().into(),
                        )),
                        None => ConvAP::StringBit(helpers::as_rust_str(&"true".to_string())),
                    });
                    stack.push(ConvAP::StrBit(" { true => "));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::Boolean),
                        boolean_to_string_value.value.as_ref().into(),
                    )));
                    ret_str.push_str("match ");
                }
                lls::model::ComputedValue::ListToStringValue(list_to_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // Format:
                    //   (<list>).join(&(<separator>))
                    stack.push(ConvAP::StrBit("))"));
                    stack.push(match list_to_string_value.separator {
                        Some(s) => ConvAP::Computed((
                            Some(structure::meta::ValueType::String),
                            s.as_ref().into(),
                        )),
                        None => ConvAP::StringBit(helpers::as_rust_string(&", ".to_string())),
                    });
                    stack.push(ConvAP::StrBit(").join(&("));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::StringList),
                        list_to_string_value.value.as_ref().into(),
                    )));
                    ret_str.push('(');
                }
                lls::model::ComputedValue::MapToStringValue(map_to_string_value) => {
                    // TODO ensure exp_type is a string value.
                    // Format:
                    //
                    // let s: String = (HashMap::<String, String>::new())
                    //    .iter()
                    //    .map(|(k, v)| format!("{}{}{}", k, key_sep, v))
                    //    .collect::<Vec<String>>()
                    //    .join(&(item_sep));
                    stack.push(ConvAP::StrBit("))"));
                    stack.push(match map_to_string_value.item_separator {
                        Some(s) => ConvAP::Computed((
                            Some(structure::meta::ValueType::String),
                            s.as_ref().into(),
                        )),
                        None => ConvAP::StringBit(helpers::as_rust_string(&", ".to_string())),
                    });
                    stack.push(ConvAP::StrBit(", v)).collect::<Vec<String>>().join(&("));
                    stack.push(match map_to_string_value.key_separator {
                        Some(s) => ConvAP::Computed((
                            Some(structure::meta::ValueType::String),
                            s.as_ref().into(),
                        )),
                        None => ConvAP::StringBit(helpers::as_rust_string(&"=".to_string())),
                    });
                    stack.push(ConvAP::StrBit(
                        ").iter().map(|(k, v)| format!(\"{}{}{}\", k, ",
                    ));
                    stack.push(ConvAP::Computed((
                        Some(structure::meta::ValueType::StringMap),
                        (&map_to_string_value.value).into(),
                    )));
                    ret_str.push('(');
                }
                lls::model::ComputedValue::ConstantStringValue(constant_string_value) => {
                    // TODO ensure exp_type is a string value.
                    ret_str.push_str(&helpers::as_rust_string(&constant_string_value.value));
                }

                // ----------------------------------------------------
                // Number Values
                // These must all treat the number as a f64.
                lls::model::ComputedValue::LookupNumberValue(lookup_number_value) => {
                    // TODO ensure exp_type is a number value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_number_value.source,
                            &"self.runtime".to_string(),
                            &lookup_number_value.job,
                            &lookup_number_value.name,
                            &structure::meta::ValueType::Float,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
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
                lls::model::ComputedValue::ConstantNumberValue(constant_number_value) => {
                    // TODO ensure exp_type is a number value.
                    ret_str.push_str(constant_number_value.value.to_string().as_str());
                }

                // ----------------------------------------------------
                // Boolean values
                lls::model::ComputedValue::LookupBooleanValue(lookup_boolean_value) => {
                    // TODO ensure exp_type is a boolean value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_boolean_value.source,
                            &"self.runtime".to_string(),
                            &lookup_boolean_value.job,
                            &lookup_boolean_value.name,
                            &structure::meta::ValueType::Boolean,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
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
                lls::model::ComputedValue::ConstantBooleanValue(constant_boolean_value) => {
                    ret_str.push_str(helpers::as_rust_bool(constant_boolean_value.value));
                }

                // ----------------------------------------------------
                // String List (Vec<String>) Values
                lls::model::ComputedValue::LookupStringListValue(lookup_string_list_value) => {
                    // TODO ensure exp_type is a string list value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_string_list_value.source,
                            &"self.runtime".to_string(),
                            &lookup_string_list_value.job,
                            &lookup_string_list_value.name,
                            &structure::meta::ValueType::StringList,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
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
                    // TODO ensure exp_type is a string list value.
                    // This is a series of 'join' statements for flattened lists.
                    //   (vec![values, ...] | list_ref).join(vec![values, ...] | list_ref) ...
                    stack.push(ConvAP::StrBit(")"));
                    let mut first = false;
                    let mut in_vec = false;
                    for val in &constant_string_list_value.value {
                        let str_list = is_string_list(&val);
                        stack.push(ConvAP::Computed((
                            Some(match str_list {
                                true => structure::meta::ValueType::StringList,
                                false => structure::meta::ValueType::String,
                            }),
                            val.into(),
                        )));
                        // General wrapper handler.
                        if str_list {
                            if first {
                                // Nothing to do...
                            } else if in_vec {
                                stack.push(ConvAP::StrBit("]).join("));
                                in_vec = false;
                            } else {
                                stack.push(ConvAP::StrBit(").join("));
                            }
                        } else if first {
                            // First item is a non-list.
                            stack.push(ConvAP::StrBit("vec!["));
                            in_vec = true;
                        } else if !in_vec {
                            // Join in a list of non-list items.
                            stack.push(ConvAP::StrBit(").join(vec!["));
                            in_vec = true;
                        } else {
                            // In a vector, and not a string list to append.
                            stack.push(ConvAP::StrBit(", "))
                        }
                        first = false;
                    }
                    ret_str.push('(');
                }

                // ----------------------------------------------------
                // Number List (Vec<f64>) Values
                lls::model::ComputedValue::LookupNumberListValue(lookup_number_list_value) => {
                    // TODO ensure exp_type is a number list value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_number_list_value.source,
                            &"self.runtime".to_string(),
                            &lookup_number_list_value.job,
                            &lookup_number_list_value.name,
                            &structure::meta::ValueType::FloatList,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
                lls::model::ComputedValue::RangeNumberListValue(range_number_list_value) => todo!(),
                lls::model::ComputedValue::ConstantNumberListValue(constant_number_list_value) => {
                    todo!()
                }

                // ----------------------------------------------------
                // Boolean list (Vec<bool>) Values
                lls::model::ComputedValue::LookupBooleanListValue(lookup_boolean_list_value) => {
                    // TODO ensure exp_type is a boolean list value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_boolean_list_value.source,
                            &"self.runtime".to_string(),
                            &lookup_boolean_list_value.job,
                            &lookup_boolean_list_value.name,
                            &structure::meta::ValueType::BooleanList,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
                lls::model::ComputedValue::RangeBooleanListValue(range_boolean_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantBooleanListValue(
                    constant_boolean_list_value,
                ) => todo!(),

                // ----------------------------------------------------
                // String Map (HashMap<String, String>) Values
                lls::model::ComputedValue::LookupStringMapValue(lookup_string_map_value) => {
                    // TODO ensure exp_type is a string map value.
                    ret_str.push_str(
                        lookup::generate_lookup(
                            &lookup_string_map_value.source,
                            &"self.runtime".to_string(),
                            &lookup_string_map_value.job,
                            &lookup_string_map_value.name,
                            &structure::meta::ValueType::StringMap,
                            false,
                            issues,
                            col,
                        )
                        .as_str(),
                    );
                }
                lls::model::ComputedValue::UnionStringMapValue(union_string_map_value) => todo!(),
                lls::model::ComputedValue::StringMapListIndexValue(string_map_list_index_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringMapValue(constant_string_map_value) => {
                    todo!()
                }

                // ----------------------------------------------------
                // Number Map (HashMap<String, f64>) Values
                lls::model::ComputedValue::LookupNumberMapValue(lookup_number_map_value) => todo!(),
                lls::model::ComputedValue::UnionNumberMapValue(union_number_map_value) => todo!(),
                lls::model::ComputedValue::ConstantNumberMapValue(constant_number_map_value) => {
                    todo!()
                }

                // ----------------------------------------------------
                // Boolean Map (HashMap<String, bool>) Values
                // (could also use HashSet<String>)
                lls::model::ComputedValue::LookupBooleanMapValue(lookup_boolean_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::UnionBooleanMapValue(union_boolean_map_value) => todo!(),
                lls::model::ComputedValue::ConstantBooleanMapValue(constant_boolean_map_value) => {
                    todo!()
                }

                // ----------------------------------------------------
                // String List Map (HashMap<String, Vec<String>>) Values
                lls::model::ComputedValue::LookupStringListMapValue(
                    lookup_string_list_map_value,
                ) => todo!(),
                lls::model::ComputedValue::UnionStringListMapValue(union_string_list_map_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringListMapValue(
                    constant_string_list_map_value,
                ) => todo!(),

                // ----------------------------------------------------
                // String Map List (Vec<HashMap<String, String>>) Values
                lls::model::ComputedValue::LookupStringMapListValue(
                    lookup_string_map_list_value,
                ) => todo!(),
                lls::model::ComputedValue::RangeStringMapListValue(range_string_map_list_value) => {
                    todo!()
                }
                lls::model::ComputedValue::ConstantStringMapListValue(
                    constant_string_map_list_value,
                ) => todo!(),
            },
        }
    }
}
