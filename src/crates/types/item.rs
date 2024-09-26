use primitives::Number;
use primitives::String;
enum Structs {
    S1(Number),
    S2(String),
}

impl Item for Structs {
    fn new() -> Self {
        match Structs {
            S1(Number) => Number {
                name: "Number".to_string(),
                max_safe_integer: std::i64::MAX,
                min_safe_integer: std::i64::MIN,
                min_value: std::f64::MIN,
                max_value: std::f64::MAX,
                nan: std::f64::NAN,
                value: _value,
            },
            S2(String) => String {
                name: "String".to_string(),
                length: i64::MAX,
                value: "".to_string(),
            },
        }
    }
}
