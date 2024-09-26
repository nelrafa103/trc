use std::collections::HashMap;
mod crates;

use crates::tokens::keys::process;
use crates::{aux::search_json, parser::parser, tokens::keys};
use serde_json::Value;
use std::time::Instant;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn count_value_occurrences(input: &HashMap<String, String>) -> HashMap<String, i16> {
    let mut count: HashMap<String, i16> = HashMap::new();

    for value in input.values() {
        *count.entry(value.clone()).or_insert(0) += 1;
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Leer el archivo JSON
    //  aux::lint_files::lint_files()?;
    //  let num_threads = 4;
    //let counter = Arc::new(Mutex::new(0));

    let path = "src/crates/test/ts/test.ts".to_string();
    /*let y = linter(path.clone());
    if y.is_err() {
        println!("{}", y.unwrap_err().to_string());
    } */
    // let por_ejemplo: String = "Hello world".to_string();

    let ejemplo: String = parser(path).unwrap();
    //println!("{}", y.unwrap_err().to_string());
    //   let contenido = fs::read_to_string("test.json")?;
    let json: Value = serde_json::from_str(&ejemplo)?;

    // Claves que te interesan
    let claves_de_interes = vec!["type".to_string(), "kind".to_string(), "email".to_string()];

    //println!("{}", json);
    let mut resultados = HashMap::new();
    search_json::search_keys(&json, &claves_de_interes, String::new(), &mut resultados);

    // Imprimir los resultados
    //         let start = Instant::now();
    let start = Instant::now();

    let mut prueba: HashMap<String, i16> = count_value_occurrences(&resultados);
    let mut counter: Duration = Duration::new(0, 0);
    search_json::parallel_search_keys(&json, &claves_de_interes, 10);
  //  let duracion = start.elapsed();
    //println!("Tiempo de ejecución: {:?}", duracion);

    println!("{}",json);
    for (ruta, valor) in resultados {
        println!("route: {}, value: {}", ruta,valor);

        let processed = process(valor);
        let start = Instant::now();
        counter += start.elapsed();

        let value = keys::control_structure_to_rust(processed);
        //    println!("{}", value);
        let duration = start.elapsed();

        // println!("Tiempo de ejecución: {:?}", duration);

        // let j = keys::identifiers_to_rust(&valor, &mut prueba)?;
        //println!("{}", j);
    }
    let duration = start.elapsed();

    println!("Tiempo de ejecución de la traduccion: {:?}", counter);
    println!("Tiempo de ejecución: {:?}", duration);

    Ok(())
}

//fn main() {
//  search_keys()
/*   let ejemplo = String::from("impl");
let ejemplo3 = String::from("impl");
let ejemplo2 = String::from("impl");

let mut identifiers_list: HashMap<String, i16> = HashMap::new();
let prueba1 =
    crates::tokens::keys::identifiers_to_rust(ejemplo.as_str(), &mut identifiers_list);
let prueba2 =
    crates::tokens::keys::identifiers_to_rust(ejemplo2.as_str(), &mut identifiers_list);
crates::tokens::keys::identifiers_to_rust(ejemplo3.as_str(), &mut identifiers_list);

for (clave, valor) in identifiers_list.clone() {
    println!("Clave: {}, Valor: {}", clave, valor);
} */
// print!("Prueba 1: {}, Prubea 2: {}", prueba1, prueba2);
// crates::parser::parser_file();
//}
