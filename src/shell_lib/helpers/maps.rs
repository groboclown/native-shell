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
pub fn finalize_map<V: Clone>(map: &HashMap<String, Option<V>>) -> HashMap<String, V>{
    let mut result: HashMap<String, V> = HashMap::new();
    // Convert Option<V> to V.
    for (key, value) in map.iter() {
        if let Some(v) = value {
            result.insert(key.clone(), v.clone());
        }
    }
    result
}

pub fn map_bool_to_string(value: bool, true_val: &String, false_val: &String) -> String {
    if value {
        true_val.clone()
    } else {
        false_val.clone()
    }
}
