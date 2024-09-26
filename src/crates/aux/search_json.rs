use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

///! It searches for the keys in the JSON object and returns a HashMap with the keys and values.
/// it is a recursive function that searches for the keys in the JSON object and returns a HashMap with the keys and values.
///! # Arguments:
///! * `value` - The JSON object to search.
///! * `searched_keys` - The keys to search.
///! * `route` - The route to the current object.
///! * `result` - The HashMap with the keys and values.
pub fn search_keys(
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
    group_size: usize,
) -> HashMap<String, String> {
    let value = Arc::new(value.clone());
    let result = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for chunk in searched_keys.chunks(group_size) {
        let value = Arc::clone(&value);
        let result = Arc::clone(&result);
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            let mut local_result = HashMap::new();
            search_keys(&value, &chunk, String::new(), &mut local_result);
            let mut global_result = result.lock().unwrap();
            global_result.extend(local_result);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

/*fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Leer el archivo JSON
    let contenido = fs::read_to_string("datos.json")?;
    let json: Value = serde_json::from_str(&contenido)?;

    // keys que te interesan
    let keys_de_interes = vec![
        "nombre".to_string(),
        "edad".to_string(),
        "email".to_string()
    ];

    let mut result = HashMap::new();
    search_keys(&json, &keys_de_interes, String::new(), &mut result);

    // Imprimir los result
    for (route, value) in result {
        println!("{}: {}", route, value);
    }

    Ok(())
}
*/
