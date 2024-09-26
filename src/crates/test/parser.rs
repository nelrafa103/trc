use std::time::Instant;

use crate::crates;
/// Diferent file sizes to test the parser module:
/// - small.ts (500 lines) 4KB
/// - medium.ts (1000 lines) 8KB
/// - large.ts (2000+ lines) 16KB
///! This function tests time for the parser module
///! It is important to test the time to parse a file
const PATH: &str = "src/crates/test/ts/sizes/small.ts";
/*#[test]
fn parser_test() {
    let start = Instant::now();
    let path = "src/crates/test/ts/sizes/small.ts".to_string();

    let result = crates::parser::parser(path);
    println!("AST: {}", result.unwrap());

    let duration = start.elapsed();

    //if()
    println!("Parser test passed in {:?}", duration);
    assert!(false, "Parser test passed in {:?}", duration);
}
*/
//test();
