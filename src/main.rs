use std::collections::HashMap;
mod crates;

use crates::{aux::search_json ,parser::parser};
use serde_json::Value;
 


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = "src/crates/test/ts/sizes/small.ts".to_string();

    let ejemplo: String = parser(path).unwrap();

    let json: Value = serde_json::from_str(&ejemplo)?;

    let claves_de_interes = vec!["type".to_string(), "end".to_string(), "start".to_string(), "key".to_string(), "name".to_string()];

    let mut resultados = HashMap::new();
    
    resultados = search_json::parallel_search_keys(&json, &claves_de_interes, 2);


    Ok(())
}
