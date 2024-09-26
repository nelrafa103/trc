use crate::crates::aux::search_json;
use serde_json::Value;
use std::collections::HashMap;

use std::fs;
use std::time::Instant;
#[test]
fn search_json_test() {
    let start = Instant::now();

    let json_string = fs::read_to_string("src/crates/test/json/test.json").unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let claves_de_interes = vec!["type".to_string(), "kind".to_string(), "email".to_string()];
    let mut resultados = HashMap::new();
    search_json::search_keys(&json, &claves_de_interes, String::new(), &mut resultados);

    let duration = start.elapsed();

    // Times table in microseconds:
    const GOOD_DURATION: u128 = 1000;
    const ACCEPTABLE_DURATION: u128 = 50000;
    const BAD_DURATION: u128 = 100000;

    let mut is_in_range = false;
    if duration.as_micros() <= GOOD_DURATION {
        is_in_range = true;
        assert!(
            is_in_range,
            "The time requiere to read the file: is :{}, is a good time",
            duration.as_micros()
        );
    } else if duration.as_micros() <= ACCEPTABLE_DURATION {
        is_in_range = true;
        assert!(
            is_in_range,
            "The time requiere to read the file: is :{}, is under the acceptable time",
            duration.as_micros()
        );
    } else if duration.as_micros() > ACCEPTABLE_DURATION {
        is_in_range = false;
        assert!(
            is_in_range,
            "The time requiere to read the file: is :{}, is under the acceptable time",
            duration.as_micros()
        );
    } else if duration.as_micros() >= BAD_DURATION {
        assert!(
            is_in_range,
            "The time requiere to read the file: is :{}, a bad time",
            duration.as_micros()
        );
    }
}
