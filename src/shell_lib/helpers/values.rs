//! Some helpers for working with the input data maps.

use std::collections::HashMap;

/// Merges multiple maps into a single map.
/// If a key exists in multiple maps, the last non-None value is kept.
/// If a value is None, the key is removed from the result map.
pub fn union_maps<V: Clone>(maps: &Vec<HashMap<String, Option<V>>>) -> HashMap<String, Option<V>> {
    let mut result: HashMap<String, Option<V>> = HashMap::new();
    for map in maps {
        for (key, value) in map {
            match value {
                None => {
                    // Remove the key if the value is None.
                    result.remove(key);
                }
                v => {
                    // If the key already exists, we keep the first non-None value.
                    result.insert(key.clone(), v.clone());
                }
            }
        }
    }
    result
}

/// Finalizes a map by converting Option<V> to V.
/// If a key has a None value, it is not included in the final map.
pub fn finalize_map<V: Clone>(map: &HashMap<String, Option<V>>) -> HashMap<String, V> {
    let mut result: HashMap<String, V> = HashMap::new();
    // Convert Option<V> to V.
    for (key, value) in map.iter() {
        if let Some(v) = value {
            result.insert(key.clone(), v.clone());
        }
    }
    result
}

pub fn finalize_map_union<V: Clone>(maps: &Vec<HashMap<String, Option<V>>>) -> HashMap<String, V> {
    let unioned_map = union_maps(maps);
    finalize_map(&unioned_map)
}

/// Convert a boolean value to a string based on the provided true and false values.
pub fn map_bool_to_string(value: bool, true_val: &String, false_val: &String) -> String {
    if value {
        true_val.clone()
    } else {
        false_val.clone()
    }
}

/// Splits a string by a separator and returns a vector of strings.
pub fn split_string(value: &str, separator: &str, max_splits: f64) -> Vec<String> {
    if max_splits <= 0.0 {
        return vec![value.to_string()];
    }
    let values: Vec<String> = value.split(separator).map(|s| s.to_string()).collect();
    if values.len() > max_splits as usize {
        values.into_iter().take(max_splits as usize).collect()
    } else {
        values
    }
}

/// Trims a string by removing specified characters from both ends.
pub fn trim_string(value: &str, trim_chars: Option<&str>) -> String {
    if let Some(chars) = trim_chars {
        value.trim_matches(|v| chars.contains(v)).to_string()
    } else {
        value.trim().to_string()
    }
}

/// Removes the first 'start' characters from the string.
pub fn sub_string_start(value: &str, start: usize) -> String {
    value.chars().skip(start).collect()
}

/// Returns the first characters up to index 'end' in the string.
/// If 'end' is < 0, then this indicates the index from the right.
pub fn sub_string_end(value: &str, end: i64) -> String {
    let mut count = end;
    if count < 0 {
        count = value.len() as i64 + count;
    }
    if count <= 0 {
        String::new()
    } else {
        value.chars().take(count as usize).collect()
    }
}

/// Returns the string at positions from 'start' up to and including 'end'.
/// If 'end' is < 0, then this indicates the index from the right.
pub fn sub_string_start_end(value: &str, start: usize, end: usize) -> String {
    let mut end = end as i64;
    if end < 0 {
        end = value.len() as i64 + end;
    }
    let count = end - start as i64;
    if count <= 0 {
        String::new()
    } else {
        value.chars().skip(start).take(count as usize).collect()
    }
}

/// Returns the 'count' characters after the 'start' position.
pub fn sub_string_start_count(value: &str, start: usize, count: usize) -> String {
    value.chars().skip(start).take(count).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_string_start() {
        assert_eq!(sub_string_start("01234", 2), "234".to_string());
        assert_eq!(sub_string_start("01234", 5), "".to_string());
        assert_eq!(sub_string_start("01234", 30), "".to_string());
    }

    #[test]
    fn test_sub_string_end() {
        assert_eq!(sub_string_end("01234", 2), "01".to_string());
        assert_eq!(sub_string_end("01234", 5), "01234".to_string());
        assert_eq!(sub_string_end("01234", 30), "01234".to_string());
        assert_eq!(sub_string_end("01234", -2), "012".to_string());
        assert_eq!(sub_string_end("01234", -5), "".to_string());
        assert_eq!(sub_string_end("01234", -30), "".to_string());
    }
}
