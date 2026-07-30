//SPDX::MIT
//

use crate::shell_lib::structure;

use super::model;

pub fn get_value_type(value: &model::ComputedValue) -> Option<structure::meta::ValueType> {
    match value {
        model::ComputedValue::ComputedNullValue(_) => None,
        model::ComputedValue::LookupStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::ListIndexStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::MapKeyStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::SubStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::TrimStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::NumberToStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::BooleanToStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::ListToStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::MapToStringValue(_) => Some(structure::meta::ValueType::String),
        model::ComputedValue::ConstantStringValue(_) => Some(structure::meta::ValueType::String),

        model::ComputedValue::LookupNumberValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::AddTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::SubtractTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::MultiplyTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::DivideTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::ModulusTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::PowerTwoValues(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::RoundValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::FloorValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::CeilValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::AbsValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::SumNumberListValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::ProductNumberListValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::AverageNumberListValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::MinNumberListValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::MaxNumberListValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::ListIndexNumberValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::MapKeyNumberValue(_) => Some(structure::meta::ValueType::Float),
        model::ComputedValue::CollectionSizeNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::StringLeftIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::StringRightIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::StringListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::NumberListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::BooleanListIndexNumberValue(_) => {
            Some(structure::meta::ValueType::Float)
        }
        model::ComputedValue::ConstantNumberValue(_) => Some(structure::meta::ValueType::Float),

        model::ComputedValue::LookupBooleanValue(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::AndTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::OrTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::NotBooleanValue(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::XorTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::NandTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::NorTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::XnorTwoBooleanValues(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::ListIndexBooleanValue(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::MapKeyBooleanValue(_) => Some(structure::meta::ValueType::Boolean),
        model::ComputedValue::MapContainsKeyBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        model::ComputedValue::ListContainsIndexBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        model::ComputedValue::StringEqualBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        model::ComputedValue::NumberEqualBooleanValue(_) => {
            Some(structure::meta::ValueType::Boolean)
        }
        model::ComputedValue::ConstantBooleanValue(_) => Some(structure::meta::ValueType::Boolean),

        model::ComputedValue::LookupStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        model::ComputedValue::SplitStringValue(_) => Some(structure::meta::ValueType::StringList),
        model::ComputedValue::RangeStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        model::ComputedValue::StringListMapKeyValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        model::ComputedValue::MapKeysStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }
        model::ComputedValue::ConstantStringListValue(_) => {
            Some(structure::meta::ValueType::StringList)
        }

        model::ComputedValue::LookupNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }
        model::ComputedValue::RangeNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }
        model::ComputedValue::ConstantNumberListValue(_) => {
            Some(structure::meta::ValueType::FloatList)
        }

        model::ComputedValue::LookupBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }
        model::ComputedValue::RangeBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }
        model::ComputedValue::ConstantBooleanListValue(_) => {
            Some(structure::meta::ValueType::BooleanList)
        }

        model::ComputedValue::LookupStringMapValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }
        model::ComputedValue::UnionStringMapValue(_) => Some(structure::meta::ValueType::StringMap),
        model::ComputedValue::StringMapListIndexValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }
        model::ComputedValue::ConstantStringMapValue(_) => {
            Some(structure::meta::ValueType::StringMap)
        }

        model::ComputedValue::LookupNumberMapValue(_) => Some(structure::meta::ValueType::FloatMap),
        model::ComputedValue::UnionNumberMapValue(_) => Some(structure::meta::ValueType::FloatMap),
        model::ComputedValue::ConstantNumberMapValue(_) => {
            Some(structure::meta::ValueType::FloatMap)
        }

        model::ComputedValue::LookupBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }
        model::ComputedValue::UnionBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }
        model::ComputedValue::ConstantBooleanMapValue(_) => {
            Some(structure::meta::ValueType::BooleanMap)
        }

        model::ComputedValue::LookupStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }
        model::ComputedValue::UnionStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }
        model::ComputedValue::ConstantStringListMapValue(_) => {
            Some(structure::meta::ValueType::StringListMap)
        }

        model::ComputedValue::LookupStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
        model::ComputedValue::RangeStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
        model::ComputedValue::ConstantStringMapListValue(_) => {
            Some(structure::meta::ValueType::StringMapList)
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedStringValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedStringValue::LookupStringValue(lookup_string_value) => {
                model::ComputedValue::LookupStringValue(lookup_string_value.clone())
            }
            model::ComputedStringValue::ListIndexStringValue(list_index_string_value) => {
                model::ComputedValue::ListIndexStringValue(list_index_string_value.clone())
            }
            model::ComputedStringValue::MapKeyStringValue(map_key_string_value) => {
                model::ComputedValue::MapKeyStringValue(map_key_string_value.clone())
            }
            model::ComputedStringValue::SubStringValue(sub_string_value) => {
                model::ComputedValue::SubStringValue(sub_string_value.clone())
            }
            model::ComputedStringValue::TrimStringValue(trim_string_value) => {
                model::ComputedValue::TrimStringValue(trim_string_value.clone())
            }
            model::ComputedStringValue::NumberToStringValue(number_to_string_value) => {
                model::ComputedValue::NumberToStringValue(number_to_string_value.clone())
            }
            model::ComputedStringValue::BooleanToStringValue(boolean_to_string_value) => {
                model::ComputedValue::BooleanToStringValue(boolean_to_string_value.clone())
            }
            model::ComputedStringValue::ListToStringValue(list_to_string_value) => {
                model::ComputedValue::ListToStringValue(list_to_string_value.clone())
            }
            model::ComputedStringValue::MapToStringValue(map_to_string_value) => {
                model::ComputedValue::MapToStringValue(map_to_string_value.clone())
            }
            model::ComputedStringValue::ConstantStringValue(constant_string_value) => {
                model::ComputedValue::ConstantStringValue(constant_string_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedNumberValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedNumberValue::LookupNumberValue(lookup_number_value) => {
                model::ComputedValue::LookupNumberValue(lookup_number_value.clone())
            }
            model::ComputedNumberValue::AddTwoValues(add_two_values) => {
                model::ComputedValue::AddTwoValues(add_two_values.clone())
            }
            model::ComputedNumberValue::SubtractTwoValues(subtract_two_values) => {
                model::ComputedValue::SubtractTwoValues(subtract_two_values.clone())
            }
            model::ComputedNumberValue::MultiplyTwoValues(multiply_two_values) => {
                model::ComputedValue::MultiplyTwoValues(multiply_two_values.clone())
            }
            model::ComputedNumberValue::DivideTwoValues(divide_two_values) => {
                model::ComputedValue::DivideTwoValues(divide_two_values.clone())
            }
            model::ComputedNumberValue::ModulusTwoValues(modulus_two_values) => {
                model::ComputedValue::ModulusTwoValues(modulus_two_values.clone())
            }
            model::ComputedNumberValue::PowerTwoValues(power_two_values) => {
                model::ComputedValue::PowerTwoValues(power_two_values.clone())
            }
            model::ComputedNumberValue::RoundValue(round_value) => {
                model::ComputedValue::RoundValue(round_value.clone())
            }
            model::ComputedNumberValue::FloorValue(floor_value) => {
                model::ComputedValue::FloorValue(floor_value.clone())
            }
            model::ComputedNumberValue::CeilValue(ceil_value) => {
                model::ComputedValue::CeilValue(ceil_value.clone())
            }
            model::ComputedNumberValue::AbsValue(abs_value) => {
                model::ComputedValue::AbsValue(abs_value.as_ref().clone())
            }
            model::ComputedNumberValue::SumNumberListValue(sum_number_list_value) => {
                model::ComputedValue::SumNumberListValue(sum_number_list_value.clone())
            }
            model::ComputedNumberValue::ProductNumberListValue(product_number_list_value) => {
                model::ComputedValue::ProductNumberListValue(product_number_list_value.clone())
            }
            model::ComputedNumberValue::AverageNumberListValue(average_number_list_value) => {
                model::ComputedValue::AverageNumberListValue(average_number_list_value.clone())
            }
            model::ComputedNumberValue::MinNumberListValue(min_number_list_value) => {
                model::ComputedValue::MinNumberListValue(min_number_list_value.clone())
            }
            model::ComputedNumberValue::MaxNumberListValue(max_number_list_value) => {
                model::ComputedValue::MaxNumberListValue(max_number_list_value.clone())
            }
            model::ComputedNumberValue::ListIndexNumberValue(list_index_number_value) => {
                model::ComputedValue::ListIndexNumberValue(list_index_number_value.clone())
            }
            model::ComputedNumberValue::MapKeyNumberValue(map_key_number_value) => {
                model::ComputedValue::MapKeyNumberValue(map_key_number_value.clone())
            }
            model::ComputedNumberValue::CollectionSizeNumberValue(collection_size_number_value) => {
                model::ComputedValue::CollectionSizeNumberValue(
                    collection_size_number_value.clone(),
                )
            }
            model::ComputedNumberValue::StringLeftIndexNumberValue(
                string_left_index_number_value,
            ) => model::ComputedValue::StringLeftIndexNumberValue(
                string_left_index_number_value.clone(),
            ),
            model::ComputedNumberValue::StringRightIndexNumberValue(
                string_right_index_number_value,
            ) => model::ComputedValue::StringRightIndexNumberValue(
                string_right_index_number_value.clone(),
            ),
            model::ComputedNumberValue::StringListIndexNumberValue(
                string_list_index_number_value,
            ) => model::ComputedValue::StringListIndexNumberValue(
                string_list_index_number_value.clone(),
            ),
            model::ComputedNumberValue::NumberListIndexNumberValue(
                number_list_index_number_value,
            ) => model::ComputedValue::NumberListIndexNumberValue(
                number_list_index_number_value.clone(),
            ),
            model::ComputedNumberValue::BooleanListIndexNumberValue(
                boolean_list_index_number_value,
            ) => model::ComputedValue::BooleanListIndexNumberValue(
                boolean_list_index_number_value.clone(),
            ),
            model::ComputedNumberValue::ConstantNumberValue(constant_number_value) => {
                model::ComputedValue::ConstantNumberValue(constant_number_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedStringListValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedStringListValue::LookupStringListValue(lookup_string_list_value) => {
                model::ComputedValue::LookupStringListValue(lookup_string_list_value.clone())
            }
            model::ComputedStringListValue::SplitStringValue(split_string_value) => {
                model::ComputedValue::SplitStringValue(split_string_value.clone())
            }
            model::ComputedStringListValue::RangeStringListValue(range_string_list_value) => {
                model::ComputedValue::RangeStringListValue(range_string_list_value.clone())
            }
            model::ComputedStringListValue::StringListMapKeyValue(string_list_map_key_value) => {
                model::ComputedValue::StringListMapKeyValue(string_list_map_key_value.clone())
            }
            model::ComputedStringListValue::MapKeysStringListValue(map_keys_string_list_value) => {
                model::ComputedValue::MapKeysStringListValue(map_keys_string_list_value.clone())
            }
            model::ComputedStringListValue::ConstantStringListValue(constant_string_list_value) => {
                model::ComputedValue::ConstantStringListValue(constant_string_list_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedStringMapValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedStringMapValue::LookupStringMapValue(lookup_string_map_value) => {
                model::ComputedValue::LookupStringMapValue(lookup_string_map_value.clone())
            }
            model::ComputedStringMapValue::UnionStringMapValue(union_string_map_value) => {
                model::ComputedValue::UnionStringMapValue(union_string_map_value.clone())
            }
            model::ComputedStringMapValue::StringMapListIndexValue(string_map_list_index_value) => {
                model::ComputedValue::StringMapListIndexValue(
                    string_map_list_index_value.as_ref().clone(),
                )
            }
            model::ComputedStringMapValue::ConstantStringMapValue(constant_string_map_value) => {
                model::ComputedValue::ConstantStringMapValue(constant_string_map_value.clone())
            }
        }
    }
}
