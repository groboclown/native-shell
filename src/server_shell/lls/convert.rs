//SPDX::MIT
//

use crate::shell_lib::structure;

use super::model;

pub fn get_value_type(value: &model::ComputedValue) -> structure::meta::ValueType {
    match value {
        model::ComputedValue::LookupStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::ListIndexStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::MapKeyStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::SubStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::TrimStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::NumberToStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::BooleanToStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::ListToStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::MapToStringValue(_) => structure::meta::ValueType::String,
        model::ComputedValue::ConstantStringValue(_) => structure::meta::ValueType::String,

        model::ComputedValue::LookupNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::AddTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::SubtractTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::MultiplyTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::DivideTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::ModulusTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::PowerTwoValues(_) => structure::meta::ValueType::Float,
        model::ComputedValue::RoundValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::FloorValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::CeilValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::AbsValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::SumNumberListValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::ProductNumberListValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::AverageNumberListValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::MinNumberListValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::MaxNumberListValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::ListIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::MapKeyNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::CollectionSizeNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::StringLeftIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::StringRightIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::StringListIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::NumberListIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::BooleanListIndexNumberValue(_) => structure::meta::ValueType::Float,
        model::ComputedValue::ConstantNumberValue(_) => structure::meta::ValueType::Float,

        model::ComputedValue::LookupBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::AndTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::OrTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::NotBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::XorTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::NandTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::NorTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::XnorTwoBooleanValues(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::ListIndexBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::MapKeyBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::MapContainsKeyBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::ListContainsIndexBooleanValue(_) => {
            structure::meta::ValueType::Boolean
        }
        model::ComputedValue::StringEqualBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::NumberEqualBooleanValue(_) => structure::meta::ValueType::Boolean,
        model::ComputedValue::ConstantBooleanValue(_) => structure::meta::ValueType::Boolean,

        model::ComputedValue::LookupStringListValue(_) => structure::meta::ValueType::StringList,
        model::ComputedValue::SplitStringValue(_) => structure::meta::ValueType::StringList,
        model::ComputedValue::RangeStringListValue(_) => structure::meta::ValueType::StringList,
        model::ComputedValue::StringListMapKeyValue(_) => structure::meta::ValueType::StringList,
        model::ComputedValue::MapKeysStringListValue(_) => structure::meta::ValueType::StringList,
        model::ComputedValue::ConstantStringListValue(_) => structure::meta::ValueType::StringList,

        model::ComputedValue::LookupNumberListValue(_) => structure::meta::ValueType::FloatList,
        model::ComputedValue::RangeNumberListValue(_) => structure::meta::ValueType::FloatList,
        model::ComputedValue::ConstantNumberListValue(_) => structure::meta::ValueType::FloatList,

        model::ComputedValue::LookupBooleanListValue(_) => structure::meta::ValueType::BooleanList,
        model::ComputedValue::RangeBooleanListValue(_) => structure::meta::ValueType::BooleanList,
        model::ComputedValue::ConstantBooleanListValue(_) => {
            structure::meta::ValueType::BooleanList
        }

        model::ComputedValue::LookupStringMapValue(_) => structure::meta::ValueType::StringMap,
        model::ComputedValue::UnionStringMapValue(_) => structure::meta::ValueType::StringMap,
        model::ComputedValue::StringMapListIndexValue(_) => structure::meta::ValueType::StringMap,
        model::ComputedValue::ConstantStringMapValue(_) => structure::meta::ValueType::StringMap,

        model::ComputedValue::LookupNumberMapValue(_) => structure::meta::ValueType::FloatMap,
        model::ComputedValue::UnionNumberMapValue(_) => structure::meta::ValueType::FloatMap,
        model::ComputedValue::ConstantNumberMapValue(_) => structure::meta::ValueType::FloatMap,

        model::ComputedValue::LookupBooleanMapValue(_) => structure::meta::ValueType::BooleanMap,
        model::ComputedValue::UnionBooleanMapValue(_) => structure::meta::ValueType::BooleanMap,
        model::ComputedValue::ConstantBooleanMapValue(_) => structure::meta::ValueType::BooleanMap,

        model::ComputedValue::LookupStringListMapValue(_) => {
            structure::meta::ValueType::StringListMap
        }
        model::ComputedValue::UnionStringListMapValue(_) => {
            structure::meta::ValueType::StringListMap
        }
        model::ComputedValue::ConstantStringListMapValue(_) => {
            structure::meta::ValueType::StringListMap
        }

        model::ComputedValue::LookupStringMapListValue(_) => {
            structure::meta::ValueType::StringMapList
        }
        model::ComputedValue::RangeStringMapListValue(_) => {
            structure::meta::ValueType::StringMapList
        }
        model::ComputedValue::ConstantStringMapListValue(_) => {
            structure::meta::ValueType::StringMapList
        }
    }
}

/// Return 'true' if the string list value item references a list.
pub fn is_string_list(value: &model::ConstantStringListValueValueItem) -> bool {
    match value {
        model::ConstantStringListValueValueItem::LookupStringValue(_) => true,
        model::ConstantStringListValueValueItem::ListIndexStringValue(_) => true,
        model::ConstantStringListValueValueItem::MapKeyStringValue(_) => true,
        model::ConstantStringListValueValueItem::SubStringValue(_) => true,
        model::ConstantStringListValueValueItem::TrimStringValue(_) => true,
        model::ConstantStringListValueValueItem::NumberToStringValue(_) => true,
        model::ConstantStringListValueValueItem::BooleanToStringValue(_) => true,
        model::ConstantStringListValueValueItem::ListToStringValue(_) => true,
        model::ConstantStringListValueValueItem::MapToStringValue(_) => true,
        model::ConstantStringListValueValueItem::ConstantStringValue(_) => true,

        model::ConstantStringListValueValueItem::LookupStringListValue(_) => false,
        model::ConstantStringListValueValueItem::SplitStringValue(_) => false,
        model::ConstantStringListValueValueItem::RangeStringListValue(_) => false,
        model::ConstantStringListValueValueItem::StringListMapKeyValue(_) => false,
        model::ConstantStringListValueValueItem::MapKeysStringListValue(_) => false,
        model::ConstantStringListValueValueItem::ConstantStringListValue(_) => false,
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

impl Into<model::ComputedValue> for &model::ComputedBooleanValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedBooleanValue::LookupBooleanValue(lookup_boolean_value) => {
                model::ComputedValue::LookupBooleanValue(lookup_boolean_value.clone())
            }
            model::ComputedBooleanValue::AndTwoBooleanValues(and_two_boolean_values) => {
                model::ComputedValue::AndTwoBooleanValues(and_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::OrTwoBooleanValues(or_two_boolean_values) => {
                model::ComputedValue::OrTwoBooleanValues(or_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::NotBooleanValue(not_boolean_value) => {
                model::ComputedValue::NotBooleanValue(not_boolean_value.clone())
            }
            model::ComputedBooleanValue::XorTwoBooleanValues(xor_two_boolean_values) => {
                model::ComputedValue::XorTwoBooleanValues(xor_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::NandTwoBooleanValues(nand_two_boolean_values) => {
                model::ComputedValue::NandTwoBooleanValues(nand_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::NorTwoBooleanValues(nor_two_boolean_values) => {
                model::ComputedValue::NorTwoBooleanValues(nor_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::XnorTwoBooleanValues(xnor_two_boolean_values) => {
                model::ComputedValue::XnorTwoBooleanValues(xnor_two_boolean_values.clone())
            }
            model::ComputedBooleanValue::ListIndexBooleanValue(list_index_boolean_value) => {
                model::ComputedValue::ListIndexBooleanValue(list_index_boolean_value.clone())
            }
            model::ComputedBooleanValue::MapKeyBooleanValue(map_key_boolean_value) => {
                model::ComputedValue::MapKeyBooleanValue(map_key_boolean_value.clone())
            }
            model::ComputedBooleanValue::MapContainsKeyBooleanValue(
                map_contains_key_boolean_value,
            ) => model::ComputedValue::MapContainsKeyBooleanValue(
                map_contains_key_boolean_value.clone(),
            ),
            model::ComputedBooleanValue::ListContainsIndexBooleanValue(
                list_contains_index_boolean_value,
            ) => model::ComputedValue::ListContainsIndexBooleanValue(
                list_contains_index_boolean_value.clone(),
            ),
            model::ComputedBooleanValue::StringEqualBooleanValue(string_equal_boolean_value) => {
                model::ComputedValue::StringEqualBooleanValue(string_equal_boolean_value.clone())
            }
            model::ComputedBooleanValue::NumberEqualBooleanValue(number_equal_boolean_value) => {
                model::ComputedValue::NumberEqualBooleanValue(number_equal_boolean_value.clone())
            }
            model::ComputedBooleanValue::ConstantBooleanValue(constant_boolean_value) => {
                model::ComputedValue::ConstantBooleanValue(constant_boolean_value.clone())
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

impl Into<model::ComputedValue> for &model::ComputedNumberListValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedNumberListValue::LookupNumberListValue(lookup_number_list_value) => {
                model::ComputedValue::LookupNumberListValue(lookup_number_list_value.clone())
            }
            model::ComputedNumberListValue::RangeNumberListValue(range_number_list_value) => {
                model::ComputedValue::RangeNumberListValue(range_number_list_value.clone())
            }
            model::ComputedNumberListValue::ConstantNumberListValue(constant_number_list_value) => {
                model::ComputedValue::ConstantNumberListValue(constant_number_list_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedBooleanListValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedBooleanListValue::LookupBooleanListValue(lookup_boolean_list_value) => {
                model::ComputedValue::LookupBooleanListValue(lookup_boolean_list_value.clone())
            }
            model::ComputedBooleanListValue::RangeBooleanListValue(range_boolean_list_value) => {
                model::ComputedValue::RangeBooleanListValue(
                    range_boolean_list_value.as_ref().clone(),
                )
            }
            model::ComputedBooleanListValue::ConstantBooleanListValue(
                constant_boolean_list_value,
            ) => {
                model::ComputedValue::ConstantBooleanListValue(constant_boolean_list_value.clone())
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

impl Into<model::ComputedValue> for &model::ComputedNumberMapValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedNumberMapValue::LookupNumberMapValue(lookup_number_map_value) => {
                model::ComputedValue::LookupNumberMapValue(lookup_number_map_value.clone())
            }
            model::ComputedNumberMapValue::UnionNumberMapValue(union_number_map_value) => {
                model::ComputedValue::UnionNumberMapValue(union_number_map_value.clone())
            }
            model::ComputedNumberMapValue::ConstantNumberMapValue(constant_number_map_value) => {
                model::ComputedValue::ConstantNumberMapValue(constant_number_map_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedBooleanMapValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedBooleanMapValue::LookupBooleanMapValue(lookup_boolean_map_value) => {
                model::ComputedValue::LookupBooleanMapValue(lookup_boolean_map_value.clone())
            }
            model::ComputedBooleanMapValue::UnionBooleanMapValue(union_boolean_map_value) => {
                model::ComputedValue::UnionBooleanMapValue(union_boolean_map_value.clone())
            }
            model::ComputedBooleanMapValue::ConstantBooleanMapValue(constant_boolean_map_value) => {
                model::ComputedValue::ConstantBooleanMapValue(constant_boolean_map_value.clone())
            }
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedStringListMapValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedStringListMapValue::LookupStringListMapValue(
                lookup_string_list_map_value,
            ) => {
                model::ComputedValue::LookupStringListMapValue(lookup_string_list_map_value.clone())
            }
            model::ComputedStringListMapValue::UnionStringListMapValue(
                union_string_list_map_value,
            ) => model::ComputedValue::UnionStringListMapValue(union_string_list_map_value.clone()),
            model::ComputedStringListMapValue::ConstantStringListMapValue(
                constant_string_list_map_value,
            ) => model::ComputedValue::ConstantStringListMapValue(
                constant_string_list_map_value.clone(),
            ),
        }
    }
}

impl Into<model::ComputedValue> for &model::ComputedStringMapListValue {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ComputedStringMapListValue::LookupStringMapListValue(
                lookup_string_map_list_value,
            ) => {
                model::ComputedValue::LookupStringMapListValue(lookup_string_map_list_value.clone())
            }
            model::ComputedStringMapListValue::RangeStringMapListValue(
                range_string_map_list_value,
            ) => model::ComputedValue::RangeStringMapListValue(range_string_map_list_value.clone()),
            model::ComputedStringMapListValue::ConstantStringMapListValue(
                constant_string_map_list_value,
            ) => model::ComputedValue::ConstantStringMapListValue(
                constant_string_map_list_value.clone(),
            ),
        }
    }
}

impl Into<model::ComputedValue> for &model::ConstantStringListValueValueItem {
    fn into(self) -> model::ComputedValue {
        match self {
            model::ConstantStringListValueValueItem::LookupStringValue(lookup_string_value) => {
                model::ComputedValue::LookupStringValue(lookup_string_value.clone())
            }
            model::ConstantStringListValueValueItem::ListIndexStringValue(
                list_index_string_value,
            ) => model::ComputedValue::ListIndexStringValue(list_index_string_value.clone()),
            model::ConstantStringListValueValueItem::MapKeyStringValue(map_key_string_value) => {
                model::ComputedValue::MapKeyStringValue(map_key_string_value.clone())
            }
            model::ConstantStringListValueValueItem::SubStringValue(sub_string_value) => {
                model::ComputedValue::SubStringValue(sub_string_value.clone())
            }
            model::ConstantStringListValueValueItem::TrimStringValue(trim_string_value) => {
                model::ComputedValue::TrimStringValue(trim_string_value.clone())
            }
            model::ConstantStringListValueValueItem::NumberToStringValue(
                number_to_string_value,
            ) => model::ComputedValue::NumberToStringValue(number_to_string_value.clone()),
            model::ConstantStringListValueValueItem::BooleanToStringValue(
                boolean_to_string_value,
            ) => model::ComputedValue::BooleanToStringValue(boolean_to_string_value.clone()),
            model::ConstantStringListValueValueItem::ListToStringValue(list_to_string_value) => {
                model::ComputedValue::ListToStringValue(list_to_string_value.clone())
            }
            model::ConstantStringListValueValueItem::MapToStringValue(map_to_string_value) => {
                model::ComputedValue::MapToStringValue(map_to_string_value.clone())
            }
            model::ConstantStringListValueValueItem::ConstantStringValue(constant_string_value) => {
                model::ComputedValue::ConstantStringValue(constant_string_value.clone())
            }
            model::ConstantStringListValueValueItem::LookupStringListValue(
                lookup_string_list_value,
            ) => model::ComputedValue::LookupStringListValue(lookup_string_list_value.clone()),
            model::ConstantStringListValueValueItem::SplitStringValue(split_string_value) => {
                model::ComputedValue::SplitStringValue(split_string_value.clone())
            }
            model::ConstantStringListValueValueItem::RangeStringListValue(
                range_string_list_value,
            ) => model::ComputedValue::RangeStringListValue(range_string_list_value.clone()),
            model::ConstantStringListValueValueItem::StringListMapKeyValue(
                string_list_map_key_value,
            ) => model::ComputedValue::StringListMapKeyValue(string_list_map_key_value.clone()),
            model::ConstantStringListValueValueItem::MapKeysStringListValue(
                map_keys_string_list_value,
            ) => model::ComputedValue::MapKeysStringListValue(map_keys_string_list_value.clone()),
            model::ConstantStringListValueValueItem::ConstantStringListValue(
                constant_string_list_value,
            ) => model::ComputedValue::ConstantStringListValue(constant_string_list_value.clone()),
        }
    }
}
