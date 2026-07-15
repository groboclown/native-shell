//! Handle turning the model's computed values into Rust code.

use std::collections::{HashMap, HashSet};

use crate::{
    server_shell::{
        builder::{
            errors::{BuilderError, ErrorDetails},
            assemble,
        },
        lls::model,
    },
    shell_lib::structure::meta,
};

/// Constructing a value requires pulling in states from nodes and lookup values before
/// constructing the values.  This recursive structure requires tracking the state and lookup values.
pub struct ConstructValueState<'a, SG: super::sequence::SequenceGen<'a>> {
    pulled_states: std::collections::HashSet<String>,
    indent: String,
    sgen: &'a SG,
}

impl<'a, SG: super::sequence::SequenceGen<'a>> ConstructValueState<'a, SG> {
    pub fn new(sgen: &'a SG, indent: &str) -> Self {
        Self {
            pulled_states: std::collections::HashSet::new(),
            indent: indent.to_string(),
            sgen,
        }
    }

    fn mark_value_visited(
        &mut self,
        source: &model::Source,
        model_type: &model::ComputedValue,
        node: &String,
        field: &String,
        needs_clone: bool,
    ) -> Result<String, BuilderError> {
        let node_id = self.node_id(source, node)?;
        self.ensure_state_type(source, node, field, model_type)?;
        self.pulled_states.insert(node_id.clone());
        if needs_clone {
            Ok(format!("state_{}.{}.clone()", node_id, field))
        } else {
            Ok(format!("state_{}.{}", node_id, field))
        }
    }

    pub fn state_values(&self) -> String {
        let mut ret = String::new();
        for state in &self.pulled_states {
            ret.push_str(&format!(
                "{}let state_{} = self.runtime.{}.state();\n",
                self.indent, state, state
            ));
        }
        ret
    }

    /// Constructs a value from a computed value, pulling in any necessary states or lookup values.
    /// This can recursively pull in other states or values.
    pub fn construct_value(
        &mut self,
        value: &model::ComputedValue,
    ) -> Result<String, BuilderError> {
        let mut visiting = std::collections::HashSet::new();
        match value {
            model::ComputedValue::StringMapValue(_) => Ok(format!(
                "crate::shell_lib::helpers::values::finalize_map({})",
                self.inner_construct_value(value, &mut visiting)?
            )),
            model::ComputedValue::NumberMapValue(_) => Ok(format!(
                "crate::shell_lib::helpers::values::finalize_map({})",
                self.inner_construct_value(value, &mut visiting)?
            )),
            model::ComputedValue::BooleanMapValue(_) => Ok(format!(
                "crate::shell_lib::helpers::values::finalize_map({})",
                self.inner_construct_value(value, &mut visiting)?
            )),
            _ => self.inner_construct_value(value, &mut visiting),
        }
    }

    fn get_node_named(
        &self,
        source: &model::Source,
        name: &String,
    ) -> Result<&'a assemble::ModuleNode, BuilderError> {
        self.sgen.get_node_named(source, name)
    }

    fn node_id(&self, source: &model::Source, name: &String) -> Result<String, BuilderError> {
        Ok(self.get_node_named(source, name)?.node_id.clone())
    }

    fn ensure_state_type(
        &self,
        source: &model::Source,
        node_name: &String,
        field: &String,
        field_type: &model::ComputedValue,
    ) -> Result<(), BuilderError> {
        let node = self.get_node_named(source, node_name)?;
        let state = node.module.state_struct.as_ref().ok_or_else(|| {
            BuilderError::NoStateForModule(ErrorDetails {
                message: format!(
                    "Node '{}' module '{}' does not have a state struct.",
                    node_name, node.module.name
                ),
                source: source.clone(),
                related: vec![],
            })
        })?;
        for meta_field in &state.fields {
            if meta_field.name == *field {
                ensure_value_type(
                    source,
                    &node.module.name,
                    &meta_field.name,
                    &meta_field.value_type,
                    field_type,
                )?;
                return Ok(());
            }
        }
        Err(BuilderError::NoSuchField(ErrorDetails {
            message: format!(
                "Node '{}' module '{}' does not have a state field named '{}'.",
                node_name, node.module.name, field
            ),
            source: source.clone(),
            related: vec![],
        }))
    }

    fn inner_construct_value(
        &mut self,
        value: &model::ComputedValue,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<String, BuilderError> {
        match value {
            model::ComputedValue::StringValue(computed_string_value) => match computed_string_value
            {
                model::ComputedStringValue::LookupStringValue(lookup_string_value) => Ok(self
                    .mark_value_visited(
                        &lookup_string_value.source,
                        value,
                        &lookup_string_value.node,
                        &lookup_string_value.name,
                        true,
                    )?),
                model::ComputedStringValue::ListIndexStringValue(list_index_string_value) => {
                    let index = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*list_index_string_value.index.clone()),
                        visiting,
                    )?;
                    let list = self.inner_construct_value(
                        &model::ComputedValue::StringListValue(
                            list_index_string_value.list.clone(),
                        ),
                        visiting,
                    )?;
                    let default = self.inner_construct_value(
                        &model::ComputedValue::StringValue(
                            *list_index_string_value.default.clone(),
                        ),
                        visiting,
                    )?;
                    Ok(format!("({}).get({}).unwrap_or({})", list, index, default))
                }
                model::ComputedStringValue::MapKeyStringValue(map_key_string_value) => {
                    let key = self.inner_construct_value(
                        &model::ComputedValue::StringValue(*map_key_string_value.key.clone()),
                        visiting,
                    )?;
                    let map = self.inner_construct_value(
                        &model::ComputedValue::StringMapValue(map_key_string_value.map.clone()),
                        visiting,
                    )?;
                    let default = self.inner_construct_value(
                        &model::ComputedValue::StringValue(*map_key_string_value.default.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).get({}).unwrap_or({})", map, key, default))
                }
                model::ComputedStringValue::SubStringValue(sub_string_value) => {
                    let string = self.inner_construct_value(
                        &model::ComputedValue::StringValue(*sub_string_value.value.clone()),
                        visiting,
                    )?;
                    let start = sub_string_value.start.clone().map(|v| {
                        self.inner_construct_value(
                            &model::ComputedValue::NumberValue(*v.clone()),
                            visiting,
                        )
                    });
                    let end = sub_string_value.end.clone().map(|v| {
                        self.inner_construct_value(
                            &model::ComputedValue::NumberValue(*v.clone()),
                            visiting,
                        )
                    });
                    let mut sub = String::new();
                    if let Some(start) = start {
                        sub.push_str(start?.as_str());
                    }
                    sub.push_str("..");
                    if let Some(end) = end {
                        sub.push_str(end?.as_str());
                    }
                    // There's probably better ways to do this.
                    Ok(format!("({}).as_str()[{}].to_string()", string, sub))
                }
                model::ComputedStringValue::TrimStringValue(trim_string_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::StringValue(*trim_string_value.value.clone()),
                        visiting,
                    )?;
                    let trim_chars = match &trim_string_value.trim_chars {
                        Some(chars) => format!(
                            "Some({})",
                            self.inner_construct_value(
                                &model::ComputedValue::StringValue(*chars.clone()),
                                visiting,
                            )?
                        ),
                        None => "None".to_string(),
                    };
                    Ok(format!(
                        "crate::shell_lib::helpers::values::trim_string({}, {})",
                        value, trim_chars
                    ))
                }
                model::ComputedStringValue::NumberToStringValue(number_to_string_value) => {
                    Ok(format!(
                        "{}.to_string()",
                        self.inner_construct_value(
                            &model::ComputedValue::NumberValue(
                                *number_to_string_value.value.clone()
                            ),
                            visiting,
                        )?
                    ))
                }
                model::ComputedStringValue::BooleanToStringValue(boolean_to_string_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::BooleanValue(boolean_to_string_value.value.clone()),
                        visiting,
                    )?;
                    if value == "true" {
                        if let Some(res) = &boolean_to_string_value.true_string {
                            self.inner_construct_value(
                                &model::ComputedValue::StringValue(*res.clone()),
                                visiting,
                            )
                        } else {
                            // Default string value.
                            Ok("\"true\".to_string()".to_string())
                        }
                    } else if value == "false" {
                        if let Some(res) = &boolean_to_string_value.false_string {
                            self.inner_construct_value(
                                &model::ComputedValue::StringValue(*res.clone()),
                                visiting,
                            )
                        } else {
                            // Default string value.
                            Ok("\"false\".to_string()".to_string())
                        }
                    } else {
                        // It's an evaluated value.
                        let true_str = boolean_to_string_value
                            .true_string
                            .as_ref()
                            .map(|s| {
                                self.inner_construct_value(
                                    &model::ComputedValue::StringValue(*s.clone()),
                                    visiting,
                                )
                            })
                            .transpose()?
                            .unwrap_or("&\"true\".to_string()".to_string());
                        let false_str = boolean_to_string_value
                            .false_string
                            .as_ref()
                            .map(|s| {
                                self.inner_construct_value(
                                    &model::ComputedValue::StringValue(*s.clone()),
                                    visiting,
                                )
                            })
                            .transpose()?
                            .unwrap_or("&\"false\".to_string()".to_string());
                        Ok(format!(
                            "crate::shell_lib::helpers::values::map_bool_to_string({}, {}, {})",
                            value, true_str, false_str
                        ))
                    }
                }
                model::ComputedStringValue::ListToStringValue(list_to_string_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::StringListValue(list_to_string_value.value.clone()),
                        visiting,
                    )?;
                    let separator = match &list_to_string_value.separator {
                        Some(separator) => self.inner_construct_value(
                            &model::ComputedValue::StringValue(*separator.clone()),
                            visiting,
                        )?,
                        None => "\"\"".to_string(),
                    };
                    Ok(format!("({}).join({})", list, separator))
                }
                model::ComputedStringValue::MapToStringValue(map_to_string_value) => {
                    let map = self.inner_construct_value(
                        &model::ComputedValue::StringMapValue(map_to_string_value.value.clone()),
                        visiting,
                    )?;
                    let key_separator = match &map_to_string_value.key_separator {
                        Some(separator) => self.inner_construct_value(
                            &model::ComputedValue::StringValue(*separator.clone()),
                            visiting,
                        )?,
                        None => "\"\"".to_string(),
                    };
                    let item_separator = match &map_to_string_value.item_separator {
                        Some(separator) => self.inner_construct_value(
                            &model::ComputedValue::StringValue(*separator.clone()),
                            visiting,
                        )?,
                        None => "\"\"".to_string(),
                    };

                    Ok(format!(
                        "{}.iter().map(|(k, v)| format!(\"{{}}{{}}{{}}\", k, {}, v)).collect::<Vec<_>>().join({})",
                        map, key_separator, item_separator,
                    ))
                }
                model::ComputedStringValue::ConstantStringValue(constant_string_value) => {
                    Ok(super::helpers::as_rust_string(&constant_string_value.value))
                }
            },
            model::ComputedValue::NumberValue(computed_number_value) => match computed_number_value
            {
                model::ComputedNumberValue::LookupNumberValue(lookup_number_value) => Ok(self
                    .mark_value_visited(
                        &lookup_number_value.source,
                        value,
                        &lookup_number_value.node,
                        &lookup_number_value.name,
                        false,
                    )?),
                model::ComputedNumberValue::AddTwoValues(add_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*add_two_values.left.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*add_two_values.right.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}) + ({})", left, right))
                }
                model::ComputedNumberValue::SubtractTwoValues(subtract_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*subtract_two_values.left.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*subtract_two_values.right.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}) - ({})", left, right))
                }
                model::ComputedNumberValue::MultiplyTwoValues(multiply_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*multiply_two_values.left.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*multiply_two_values.right.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}) * ({})", left, right))
                }
                model::ComputedNumberValue::DivideTwoValues(divide_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*divide_two_values.left.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*divide_two_values.right.clone()),
                        visiting,
                    )?;
                    // If we're really good, we'd do some validation here to ensure that the right value is not constant zero.
                    if right == "0" || right == "0.0" {
                        return Err(BuilderError::FieldTypeMismatch(ErrorDetails {
                            message: "Division by zero is not allowed.".to_string(),
                            source: divide_two_values.source.clone(),
                            related: vec![],
                        }));
                    }
                    Ok(format!("({}) / ({})", left, right))
                }
                model::ComputedNumberValue::ModulusTwoValues(modulus_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*modulus_two_values.left.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*modulus_two_values.right.clone()),
                        visiting,
                    )?;
                    // If we're really good, we'd do some validation here to ensure that the right value is not constant zero.
                    Ok(format!("({}) % ({})", left, right))
                }
                model::ComputedNumberValue::PowerTwoValues(power_two_values) => {
                    let left = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*power_two_values.base.clone()),
                        visiting,
                    )?;
                    let right = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*power_two_values.exponent.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).pow({})", left, right))
                }
                model::ComputedNumberValue::RoundValue(round_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*round_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).round()", value))
                }
                model::ComputedNumberValue::FloorValue(floor_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*floor_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).floor()", value))
                }
                model::ComputedNumberValue::CeilValue(ceil_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*ceil_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).ceil()", value))
                }
                model::ComputedNumberValue::AbsValue(abs_value) => {
                    let value = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(abs_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).abs()", value))
                }
                model::ComputedNumberValue::SumNumberListValue(sum_number_list_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(sum_number_list_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).iter().sum::<f64>()", list))
                }
                model::ComputedNumberValue::ProductNumberListValue(product_number_list_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(
                            product_number_list_value.value.clone(),
                        ),
                        visiting,
                    )?;
                    Ok(format!("({}).iter().product::<f64>()", list))
                }
                model::ComputedNumberValue::AverageNumberListValue(average_number_list_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(
                            average_number_list_value.value.clone(),
                        ),
                        visiting,
                    )?;
                    // If we were really good, we'd check that the list is not empty.
                    Ok(format!(
                        "({}).iter().sum::<f64>() / {}.len() as f64",
                        list, list
                    ))
                }
                model::ComputedNumberValue::MinNumberListValue(min_number_list_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(min_number_list_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!(
                        "({}).iter().cloned().fold(f64::INFINITY, f64::min)",
                        list
                    ))
                }
                model::ComputedNumberValue::MaxNumberListValue(max_number_list_value) => {
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(max_number_list_value.value.clone()),
                        visiting,
                    )?;
                    Ok(format!(
                        "({}).iter().cloned().fold(f64::NEG_INFINITY, f64::max)",
                        list
                    ))
                }
                model::ComputedNumberValue::ListIndexNumberValue(list_index_number_value) => {
                    let index = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*list_index_number_value.index.clone()),
                        visiting,
                    )?;
                    let list = self.inner_construct_value(
                        &model::ComputedValue::NumberListValue(
                            list_index_number_value.list.clone(),
                        ),
                        visiting,
                    )?;
                    let default = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(
                            *list_index_number_value.default.clone(),
                        ),
                        visiting,
                    )?;
                    Ok(format!("({}).get({}).unwrap_or({})", list, index, default))
                }
                model::ComputedNumberValue::MapKeyNumberValue(map_key_number_value) => {
                    let key = self.inner_construct_value(
                        &model::ComputedValue::StringValue(map_key_number_value.key.clone()),
                        visiting,
                    )?;
                    let map = self.inner_construct_value(
                        &model::ComputedValue::NumberMapValue(map_key_number_value.map.clone()),
                        visiting,
                    )?;
                    let default = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*map_key_number_value.default.clone()),
                        visiting,
                    )?;
                    Ok(format!("({}).get({}).unwrap_or({})", map, key, default))
                }
                model::ComputedNumberValue::ConstantNumberValue(constant_number_value) => {
                    Ok(constant_number_value.value.to_string())
                }
            },
            model::ComputedValue::BooleanValue(computed_boolean_value) => {
                match computed_boolean_value {
                    model::ComputedBooleanValue::LookupBooleanValue(lookup_boolean_value) => {
                        Ok(self.mark_value_visited(
                            &lookup_boolean_value.source,
                            value,
                            &lookup_boolean_value.node,
                            &lookup_boolean_value.name,
                            false,
                        )?)
                    }
                    model::ComputedBooleanValue::AndTwoBooleanValues(and_two_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(*and_two_values.left.clone()),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(*and_two_values.right.clone()),
                            visiting,
                        )?;
                        Ok(format!("({}) && ({})", left, right))
                    }
                    model::ComputedBooleanValue::OrTwoBooleanValues(or_two_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(*or_two_values.left.clone()),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(*or_two_values.right.clone()),
                            visiting,
                        )?;
                        Ok(format!("({}) || ({})", left, right))
                    }
                    model::ComputedBooleanValue::NotBooleanValue(not_value) => {
                        let value = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(*not_value.value.clone()),
                            visiting,
                        )?;
                        Ok(format!("!({})", value))
                    }
                    model::ComputedBooleanValue::XorTwoBooleanValues(xor_two_boolean_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *xor_two_boolean_values.left.clone(),
                            ),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *xor_two_boolean_values.right.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("({}) ^ ({})", left, right))
                    }
                    model::ComputedBooleanValue::NandTwoBooleanValues(nand_two_boolean_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *nand_two_boolean_values.left.clone(),
                            ),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *nand_two_boolean_values.right.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("!(({}) && ({}))", left, right))
                    }
                    model::ComputedBooleanValue::NorTwoBooleanValues(nor_two_boolean_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *nor_two_boolean_values.left.clone(),
                            ),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *nor_two_boolean_values.right.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("!(({}) || ({}))", left, right))
                    }
                    model::ComputedBooleanValue::XnorTwoBooleanValues(xnor_two_boolean_values) => {
                        let left = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *xnor_two_boolean_values.left.clone(),
                            ),
                            visiting,
                        )?;
                        let right = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *xnor_two_boolean_values.right.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("!(({}) ^ ({}))", left, right))
                    }
                    model::ComputedBooleanValue::ListIndexBooleanValue(
                        list_index_boolean_value,
                    ) => {
                        let index = self.inner_construct_value(
                            &model::ComputedValue::NumberValue(
                                *list_index_boolean_value.index.clone(),
                            ),
                            visiting,
                        )?;
                        let list = self.inner_construct_value(
                            &model::ComputedValue::BooleanListValue(
                                list_index_boolean_value.list.clone(),
                            ),
                            visiting,
                        )?;
                        let default = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *list_index_boolean_value.default.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("({}).get({}).unwrap_or({})", list, index, default))
                    }
                    model::ComputedBooleanValue::MapKeyBooleanValue(map_key_boolean_value) => {
                        let key = self.inner_construct_value(
                            &model::ComputedValue::StringValue(*map_key_boolean_value.key.clone()),
                            visiting,
                        )?;
                        let map = self.inner_construct_value(
                            &model::ComputedValue::BooleanMapValue(
                                map_key_boolean_value.map.clone(),
                            ),
                            visiting,
                        )?;
                        let default = self.inner_construct_value(
                            &model::ComputedValue::BooleanValue(
                                *map_key_boolean_value.default.clone(),
                            ),
                            visiting,
                        )?;
                        Ok(format!("({}).get({}).unwrap_or({})", map, key, default))
                    }
                    model::ComputedBooleanValue::ConstantBooleanValue(constant_boolean_value) => {
                        Ok(constant_boolean_value.value.to_string())
                    }
                }
            }
            model::ComputedValue::StringListValue(computed_string_list_value) => Ok(format!(
                "vec![{}]",
                self.str_list(computed_string_list_value, visiting)?
            )),
            model::ComputedValue::NumberListValue(computed_number_list_value) => Ok(format!(
                "vec![{}]",
                self.number_list(computed_number_list_value, visiting)?
            )),
            model::ComputedValue::BooleanListValue(computed_boolean_list_value) => Ok(format!(
                "vec![{}]",
                self.boolean_list(computed_boolean_list_value, visiting)?
            )),
            model::ComputedValue::StringMapValue(computed_string_map_value) => {
                match computed_string_map_value {
                    model::ComputedStringMapValue::LookupStringMapValue(
                        lookup_string_map_value,
                    ) => Ok(self.mark_value_visited(
                        &lookup_string_map_value.source,
                        value,
                        &lookup_string_map_value.node,
                        &lookup_string_map_value.name,
                        true,
                    )?),
                    model::ComputedStringMapValue::UnionStringMapValue(union_string_map_value) => {
                        let mut values = String::new();
                        let mut first = true;
                        for map in &union_string_map_value.values {
                            if !first {
                                values.push_str(", ");
                            }
                            first = false;
                            values.push_str(&self.inner_construct_value(
                                &model::ComputedValue::StringMapValue(map.clone()),
                                visiting,
                            )?);
                        }
                        Ok(format!(
                            "crate::shell_lib::helpers::union_maps(vec![{}])",
                            values
                        ))
                    }
                    model::ComputedStringMapValue::ConstantStringMapValue(
                        constant_string_map_value,
                    ) => {
                        let mut ret = "std::collections::HashMap::from([".to_string();
                        for (key, value) in &constant_string_map_value.value {
                            let key = key.as_str();
                            let value = match value {
                                Some(v) => format!(
                                    "Some({})",
                                    self.inner_construct_value(
                                        &model::ComputedValue::StringValue(*v.clone()),
                                        visiting,
                                    )?
                                ),
                                None => "None".to_string(),
                            };
                            ret.push_str(&format!("({key}, {value}), "));
                        }
                        ret.push_str("])");
                        Ok(ret)
                    }
                }
            }
            model::ComputedValue::NumberMapValue(computed_number_map_value) => {
                match computed_number_map_value {
                    model::ComputedNumberMapValue::LookupNumberMapValue(
                        lookup_number_map_value,
                    ) => Ok(self.mark_value_visited(
                        &lookup_number_map_value.source,
                        value,
                        &lookup_number_map_value.node,
                        &lookup_number_map_value.name,
                        true,
                    )?),
                    model::ComputedNumberMapValue::UnionNumberMapValue(union_number_map_value) => {
                        let mut values = String::new();
                        let mut first = true;
                        for map in &union_number_map_value.values {
                            if !first {
                                values.push_str(", ");
                            }
                            first = false;
                            values.push_str(&self.inner_construct_value(
                                &model::ComputedValue::NumberMapValue(map.clone()),
                                visiting,
                            )?);
                        }
                        Ok(format!(
                            "crate::shell_lib::helpers::union_maps(vec![{}])",
                            values
                        ))
                    }
                    model::ComputedNumberMapValue::ConstantNumberMapValue(
                        constant_number_map_value,
                    ) => {
                        let mut ret = "std::collections::HashMap::from([".to_string();
                        for (key, value) in &constant_number_map_value.value {
                            let key = key.as_str();
                            let value = match value {
                                Some(v) => format!(
                                    "Some({})",
                                    self.inner_construct_value(
                                        &model::ComputedValue::NumberValue(*v.clone()),
                                        visiting,
                                    )?
                                ),
                                None => "None".to_string(),
                            };
                            ret.push_str(&format!("({key}, {value}), "));
                        }
                        ret.push_str("])");
                        Ok(ret)
                    }
                }
            }
            model::ComputedValue::BooleanMapValue(computed_boolean_map_value) => {
                match computed_boolean_map_value {
                    model::ComputedBooleanMapValue::LookupBooleanMapValue(
                        lookup_boolean_map_value,
                    ) => Ok(self.mark_value_visited(
                        &lookup_boolean_map_value.source,
                        value,
                        &lookup_boolean_map_value.node,
                        &lookup_boolean_map_value.name,
                        true,
                    )?),
                    model::ComputedBooleanMapValue::UnionBooleanMapValue(
                        union_boolean_map_value,
                    ) => {
                        let mut values = String::new();
                        let mut first = true;
                        for map in &union_boolean_map_value.values {
                            if !first {
                                values.push_str(", ");
                            }
                            first = false;
                            values.push_str(&self.inner_construct_value(
                                &model::ComputedValue::BooleanMapValue(map.clone()),
                                visiting,
                            )?);
                        }
                        Ok(format!(
                            "crate::shell_lib::helpers::union_maps(vec![{}])",
                            values
                        ))
                    }
                    model::ComputedBooleanMapValue::ConstantBooleanMapValue(
                        constant_boolean_map_value,
                    ) => {
                        let mut ret = "std::collections::HashMap::from([".to_string();
                        for (key, value) in &constant_boolean_map_value.value {
                            let key = key.as_str();
                            let value = match value {
                                Some(v) => format!(
                                    "Some({})",
                                    self.inner_construct_value(
                                        &model::ComputedValue::BooleanValue(v.clone()),
                                        visiting,
                                    )?
                                ),
                                None => "None".to_string(),
                            };
                            ret.push_str(&format!("({key}, {value}), "));
                        }
                        ret.push_str("])");
                        Ok(ret)
                    }
                }
            }
        }
    }

    fn str_list(
        &mut self,
        value: &model::ComputedStringListValue,
        visiting: &mut HashSet<String>,
    ) -> Result<String, BuilderError> {
        match value {
            model::ComputedStringListValue::LookupStringListValue(lookup_string_list_value) => {
                Ok(self.mark_value_visited(
                    &lookup_string_list_value.source,
                    &model::ComputedValue::StringListValue(value.clone()),
                    &lookup_string_list_value.node,
                    &lookup_string_list_value.name,
                    true,
                )?)
            }
            model::ComputedStringListValue::SplitStringValue(split_string_value) => {
                let value = self.inner_construct_value(
                    &model::ComputedValue::StringValue(*split_string_value.value.clone()),
                    visiting,
                )?;
                let separator = self.inner_construct_value(
                    &model::ComputedValue::StringValue(*split_string_value.separator.clone()),
                    visiting,
                )?;
                let maximum_splits = match &split_string_value.maximum_splits {
                    Some(maximum_splits) => self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*maximum_splits.clone()),
                        visiting,
                    )?,
                    None => "f64::MAX".to_string(),
                };
                Ok(format!(
                    "crate::shell_lib::helpers::values::split_string({}, {}, {})",
                    value, separator, maximum_splits
                ))
            }
            model::ComputedStringListValue::RangeStringListValue(range_string_list_value) => {
                let start = match &range_string_list_value.start {
                    Some(start) => self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*start.clone()),
                        visiting,
                    )?,
                    None => String::new(),
                };
                let list = self.inner_construct_value(
                    &model::ComputedValue::StringListValue(*range_string_list_value.list.clone()),
                    visiting,
                )?;
                if let Some(end) = &range_string_list_value.end {
                    if range_string_list_value.count.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_string_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let end = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*end.clone()),
                        visiting,
                    )?;
                    Ok(format!("({})[({})..({})].to_vec()", list, start, end))
                } else if let Some(count) = &range_string_list_value.count {
                    if range_string_list_value.end.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_string_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let count = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*count.clone()),
                        visiting,
                    )?;
                    Ok(format!(
                        "({})[({})..(({})+({}))].to_vec()",
                        list, start, start, count
                    ))
                } else if start.is_empty() {
                    Ok(list)
                } else {
                    Ok(format!("({})[({})..].to_vec()", list, start))
                }
            }
            model::ComputedStringListValue::ConstantStringListValue(constant_string_list_value) => {
                let mut s = String::new();
                for item in &constant_string_list_value.value {
                    match item {
                        model::ConstantStringListValueValueItem::Value(computed_string_value) => {
                            s.push_str(
                                self.inner_construct_value(
                                    &model::ComputedValue::StringValue(
                                        computed_string_value.clone(),
                                    ),
                                    visiting,
                                )?
                                .as_str(),
                            );
                            s.push_str(", ");
                        }
                        model::ConstantStringListValueValueItem::ListValue(
                            computed_string_list_value,
                        ) => {
                            s.push_str(
                                self.str_list(computed_string_list_value, visiting)?
                                    .as_str(),
                            );
                        }
                    }
                }
                Ok(s)
            }
        }
    }

    fn number_list(
        &mut self,
        value: &model::ComputedNumberListValue,
        visiting: &mut HashSet<String>,
    ) -> Result<String, BuilderError> {
        match value {
            model::ComputedNumberListValue::LookupNumberListValue(lookup_number_list_value) => {
                Ok(self.mark_value_visited(
                    &lookup_number_list_value.source,
                    &model::ComputedValue::NumberListValue(value.clone()),
                    &lookup_number_list_value.node,
                    &lookup_number_list_value.name,
                    true,
                )?)
            }
            model::ComputedNumberListValue::RangeNumberListValue(range_number_list_value) => {
                let start = match &range_number_list_value.start {
                    Some(start) => self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*start.clone()),
                        visiting,
                    )?,
                    None => String::new(),
                };
                let list = self.inner_construct_value(
                    &model::ComputedValue::NumberListValue(*range_number_list_value.list.clone()),
                    visiting,
                )?;
                if let Some(end) = &range_number_list_value.end {
                    if range_number_list_value.count.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_number_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let end = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*end.clone()),
                        visiting,
                    )?;
                    Ok(format!("({})[({})..({})].to_vec()", list, start, end))
                } else if let Some(count) = &range_number_list_value.count {
                    if range_number_list_value.end.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_number_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let count = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*count.clone()),
                        visiting,
                    )?;
                    Ok(format!(
                        "({})[({})..(({})+({}))].to_vec()",
                        list, start, start, count
                    ))
                } else if start.is_empty() {
                    Ok(list)
                } else {
                    Ok(format!("({})[({})..].to_vec()", list, start))
                }
            }
            model::ComputedNumberListValue::ConstantNumberListValue(constant_number_list_value) => {
                let mut s = String::new();
                for item in &constant_number_list_value.value {
                    match item {
                        model::ConstantNumberListValueValueItem::Value(computed_number_value) => {
                            s.push_str(
                                self.inner_construct_value(
                                    &model::ComputedValue::NumberValue(
                                        computed_number_value.clone(),
                                    ),
                                    visiting,
                                )?
                                .as_str(),
                            );
                            s.push_str(", ");
                        }
                        model::ConstantNumberListValueValueItem::ListValue(
                            computed_number_list_value,
                        ) => {
                            s.push_str(
                                self.number_list(computed_number_list_value, visiting)?
                                    .as_str(),
                            );
                        }
                    }
                }
                Ok(s)
            }
        }
    }

    fn boolean_list(
        &mut self,
        value: &model::ComputedBooleanListValue,
        visiting: &mut HashSet<String>,
    ) -> Result<String, BuilderError> {
        match value {
            model::ComputedBooleanListValue::LookupBooleanListValue(lookup_boolean_list_value) => {
                Ok(self.mark_value_visited(
                    &lookup_boolean_list_value.source,
                    &model::ComputedValue::BooleanListValue(value.clone()),
                    &lookup_boolean_list_value.node,
                    &lookup_boolean_list_value.name,
                    true,
                )?)
            }
            model::ComputedBooleanListValue::RangeBooleanListValue(range_boolean_list_value) => {
                let start = match &range_boolean_list_value.start {
                    Some(start) => self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*start.clone()),
                        visiting,
                    )?,
                    None => String::new(),
                };
                let list = self.inner_construct_value(
                    &model::ComputedValue::BooleanListValue(*range_boolean_list_value.list.clone()),
                    visiting,
                )?;
                if let Some(end) = &range_boolean_list_value.end {
                    if range_boolean_list_value.count.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_boolean_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let end = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*end.clone()),
                        visiting,
                    )?;
                    Ok(format!("({})[({})..({})].to_vec()", list, start, end))
                } else if let Some(count) = &range_boolean_list_value.count {
                    if range_boolean_list_value.end.is_some() {
                        return Err(BuilderError::InvalidRange(ErrorDetails {
                            message: "Cannot specify both end and count in a range.".to_string(),
                            source: range_boolean_list_value.source.clone(),
                            related: vec![],
                        }));
                    }
                    let count = self.inner_construct_value(
                        &model::ComputedValue::NumberValue(*count.clone()),
                        visiting,
                    )?;
                    Ok(format!(
                        "({})[({})..(({})+({}))].to_vec()",
                        list, start, start, count
                    ))
                } else if start.is_empty() {
                    Ok(list)
                } else {
                    Ok(format!("({})[({})..].to_vec()", list, start))
                }
            }
            model::ComputedBooleanListValue::ConstantBooleanListValue(
                constant_boolean_list_value,
            ) => {
                let mut s = String::new();
                for item in &constant_boolean_list_value.value {
                    match item {
                        model::ConstantBooleanListValueValueItem::Value(computed_boolean_value) => {
                            s.push_str(
                                self.inner_construct_value(
                                    &model::ComputedValue::BooleanValue(
                                        computed_boolean_value.clone(),
                                    ),
                                    visiting,
                                )?
                                .as_str(),
                            );
                            s.push_str(", ");
                        }
                        model::ConstantBooleanListValueValueItem::ListValue(
                            computed_boolean_list_value,
                        ) => {
                            s.push_str(
                                self.boolean_list(computed_boolean_list_value, visiting)?
                                    .as_str(),
                            );
                        }
                    }
                }
                Ok(s)
            }
        }
    }
}

fn match_value_type(meta_type: &meta::ValueType, model_type: &model::ComputedValue) -> bool {
    match meta_type {
        meta::ValueType::String => matches!(model_type, model::ComputedValue::StringValue(_)),
        meta::ValueType::Float => matches!(model_type, model::ComputedValue::NumberValue(_)),
        meta::ValueType::Boolean => matches!(model_type, model::ComputedValue::BooleanValue(_)),
        meta::ValueType::StringList => {
            matches!(model_type, model::ComputedValue::StringListValue(_))
        }
        meta::ValueType::FloatList => {
            matches!(model_type, model::ComputedValue::NumberListValue(_))
        }
        meta::ValueType::BooleanList => {
            matches!(model_type, model::ComputedValue::BooleanListValue(_))
        }
        meta::ValueType::StringMap => matches!(model_type, model::ComputedValue::StringMapValue(_)),
        meta::ValueType::FloatMap => matches!(model_type, model::ComputedValue::NumberMapValue(_)),
        meta::ValueType::BooleanMap => {
            matches!(model_type, model::ComputedValue::BooleanMapValue(_))
        }

        // This one has no direct corollary.
        meta::ValueType::Enum(_) => matches!(model_type, model::ComputedValue::StringValue(_)),
    }
}

fn ensure_value_type(
    source: &model::Source,
    module_name: &String,
    field_name: &String,
    meta_type: &meta::ValueType,
    model_type: &model::ComputedValue,
) -> Result<(), BuilderError> {
    if !match_value_type(meta_type, model_type) {
        return Err(BuilderError::FieldTypeMismatch(ErrorDetails {
            message: format!(
                "Expected value type '{:?}' for field {} in module {}, found '{:?}'.",
                meta_type, field_name, module_name, model_type
            ),
            source: source.clone(),
            related: vec![],
        }));
    }
    Ok(())
}

pub fn construct_parameter_values(
    source: &model::Source,
    values: &HashMap<model::NodeInitialParametersKey, model::Parameter>,
    module: &meta::ModuleStructure,
) -> Result<HashMap<String, String>, BuilderError> {
    let mut errors = vec![];
    let mut ret = HashMap::new();
    let mut param_values: HashMap<String, &model::Parameter> = HashMap::new();
    let mut unused: HashSet<String> = HashSet::new();
    for (k, f) in values {
        let key: String = k.clone().into();
        param_values.insert(key.clone(), f);
        unused.insert(key);
    }

    for field in &module.fields {
        let param = param_values.get(&field.name);
        if param.is_none() {
            if !field.optional {
                errors.push(BuilderError::RequiredFieldMissing(ErrorDetails {
                    message: format!(
                        "Missing required field {} in module {}.",
                        field.name, module.name
                    ),
                    source: source.clone(),
                    related: vec![],
                }));
            }
            continue;
        }
        let mut opt1 = "";
        let mut opt2 = "";
        if field.optional {
            opt1 = "Some(";
            opt2 = ")";
        }
        let param = param.unwrap();
        match &field.value_type {
            meta::ValueType::String => match &param.value {
                model::ParameterValue::String {
                    source: _source,
                    value,
                } => {
                    ret.insert(
                        field.name.clone(),
                        format!("{}{}{}", opt1, super::helpers::as_rust_string(&value), opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected string value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::Float => match &param.value {
                model::ParameterValue::Number {
                    source: _source,
                    value,
                } => {
                    ret.insert(
                        field.name.clone(),
                        format!("{}{}{}", opt1, value.to_string(), opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected number value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::Boolean => match &param.value {
                model::ParameterValue::Boolean {
                    source: _source,
                    value,
                } => {
                    ret.insert(
                        field.name.clone(),
                        format!("{}{}{}", opt1, value.to_string(), opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected boolean value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::StringList => match &param.value {
                model::ParameterValue::StringList {
                    source: _source,
                    value,
                } => {
                    let mut list = String::new();
                    let mut first = true;
                    for item in value {
                        if !first {
                            list.push_str(", ");
                        }
                        first = false;
                        list.push_str(&super::helpers::as_rust_string(item));
                        list.push_str(".to_string()");
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}vec![{}]{}", opt1, list, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected string list value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::FloatList => match &param.value {
                model::ParameterValue::NumberList {
                    source: _source,
                    value,
                } => {
                    let mut list = String::new();
                    let mut first = true;
                    for item in value {
                        if !first {
                            list.push_str(", ");
                        }
                        first = false;
                        list.push_str(item.to_string().as_str());
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}vec![{}]{}", opt1, list, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected number list value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::BooleanList => match &param.value {
                model::ParameterValue::BooleanList {
                    source: _source,
                    value,
                } => {
                    let mut list = String::new();
                    let mut first = true;
                    for item in value {
                        if !first {
                            list.push_str(", ");
                        }
                        first = false;
                        list.push_str(item.to_string().as_str());
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}vec![{}]{}", opt1, list, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected boolean list value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::StringMap => match &param.value {
                model::ParameterValue::StringMap {
                    source: _source,
                    value,
                } => {
                    let mut map = String::new();
                    let mut first = true;
                    for (key, value) in value {
                        if !first {
                            map.push_str(", ");
                        }
                        first = false;
                        map.push_str(&format!(
                            "({}.to_string(), {}.to_string())",
                            super::helpers::as_rust_string(key),
                            super::helpers::as_rust_string(value)
                        ));
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}std::collections::HashMap::from([{}]){}", opt1, map, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected string map value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::FloatMap => match &param.value {
                model::ParameterValue::NumberMap {
                    source: _source,
                    value,
                } => {
                    let mut map = String::new();
                    let mut first = true;
                    for (key, value) in value {
                        if !first {
                            map.push_str(", ");
                        }
                        first = false;
                        map.push_str(&format!(
                            "({}.to_string(), {})",
                            super::helpers::as_rust_string(key),
                            value
                        ));
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}std::collections::HashMap::from([{}]){}", opt1, map, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected number map value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::BooleanMap => match &param.value {
                model::ParameterValue::BooleanMap {
                    source: _source,
                    value,
                } => {
                    let mut map = String::new();
                    let mut first = true;
                    for (key, value) in value {
                        if !first {
                            map.push_str(", ");
                        }
                        first = false;
                        map.push_str(&format!(
                            "({}.to_string(), {})",
                            super::helpers::as_rust_string(key),
                            value
                        ));
                    }
                    ret.insert(
                        field.name.clone(),
                        format!("{}std::collections::HashMap::from([{}]){}", opt1, map, opt2),
                    );
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected boolean map value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
            meta::ValueType::Enum(items) => match &param.value {
                model::ParameterValue::String {
                    source: _source,
                    value,
                } => {
                    if items.contains(value) {
                        ret.insert(
                            field.name.clone(),
                            format!("{}{}{}", opt1, super::helpers::as_rust_string(value), opt2),
                        );
                    } else {
                        errors.push(BuilderError::FieldTypeMismatch(
                            ErrorDetails {
                                message: format!(
                                    "Expected enum value for field {} in module {}, found '{}', expected one of {:?}.",
                                    field.name, module.name, value, items
                                ),
                                source: source.clone(),
                                related: vec![],
                            },
                        ));
                    }
                }
                _ => {
                    errors.push(BuilderError::FieldTypeMismatch(ErrorDetails {
                        message: format!(
                            "Expected enum value for field {} in module {}, found {:?}.",
                            field.name, module.name, param.value
                        ),
                        source: source.clone(),
                        related: vec![],
                    }));
                }
            },
        }
    }
    if errors.is_empty() {
        return Ok(ret);
    } else {
        return Err(BuilderError::Collection(errors));
    }
}
