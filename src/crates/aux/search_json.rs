use serde_json::Value;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread;

///! It finds the amount of occurence of keys in a hashmap 
/// and returns the amount in hashmap <String, i16>
fn count_value_occurrences(input: &HashMap<String, String>) -> HashMap<String, i16> {
    let mut count: HashMap<String, i16> = HashMap::new();

    for value in input.values() {
        *count.entry(value.clone()).or_insert(0) += 1;
    }

    count
}


///! It searches for the keys in the JSON object and returns a HashMap with the keys and values.
/// it is a recursive function that searches for the keys in the JSON object and returns a HashMap with the keys and values.
///! # Arguments:
///! * `value` - The JSON object to search.
///! * `searched_keys` - The keys to search.
///! * `route` - The route to the current object.
///! * `result` - The HashMap with the keys and values.
fn search_keys(
    value: &Value,
    searched_keys: &[String],
    route: String,
    result: &mut HashMap<String, String>,
) {
    match value {
        Value::Object(mapa) => {
            for (key, value) in mapa {
                let new_route = if route.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", route, key)
                };
                if searched_keys.contains(&key.to_string()) {
                    result.insert(new_route.clone(), value.to_string());
                }
                search_keys(value, searched_keys, new_route, result);
            }
        }
        Value::Array(lista) => {
            for (index, item) in lista.iter().enumerate() {
                let new_route = format!("{}[{}]", route, index);
                search_keys(item, searched_keys, new_route, result);
            }
        }
        _ => {}
    }
}
///! It searches for the keys in the JSON object and returns a HashMap with the keys and values.
/// it is a parallel version of the search_keys function that searches for the keys in the JSON object and returns a HashMap with the keys and values.
pub fn parallel_search_keys(
    value: &Value,
    searched_keys: &[String],
    num_threads: usize,
) -> HashMap<String, String> {
    let value = Arc::new(value.clone());
    let searched_keys = Arc::new(searched_keys.to_vec());
    let mut handles = vec![];

    for i in 0..num_threads {
        let value = Arc::clone(&value);
        let searched_keys = Arc::clone(&searched_keys);

        let handle = thread::spawn(move || {
            let mut local_result = HashMap::new();
            let chunk_size = (searched_keys.len() + num_threads - 1) / num_threads;
            let start = i * chunk_size;
            let end = (start + chunk_size).min(searched_keys.len());

            search_keys(&value, &searched_keys[start..end], String::new(), &mut local_result);
            local_result
        });

        handles.push(handle);
    }

    let mut final_result = HashMap::new();
    for handle in handles {
        final_result.extend(handle.join().unwrap());
    }

    final_result
}


