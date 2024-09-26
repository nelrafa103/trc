#![allow(clippy::print_stdout)]
use std::{fs, path::Path};

use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use pico_args::Arguments;

pub fn parser(file: String) -> Result<String, String> {
    let mut args = Arguments::from_env();

    let name = args
        .subcommand()
        .ok()
        .flatten()
        .unwrap_or_else(|| String::from(file));
    let show_ast = args.contains("--ast");
    let show_comments = args.contains("--comments");

    let path = Path::new(&name);
    println!("{}", path.display());
    let source_text = fs::read_to_string(path).map_err(|_| format!("Missing '{name}'"))?;
    let source_type = SourceType::from_path(path).unwrap();

    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    //ret.program;

    if show_ast {
        println!("AST:");
        println!("{}", serde_json::to_string_pretty(&ret.program).unwrap());

        //println!("{}", ret.program);
    }

    if show_comments {
        println!("Comments:");
        for comment in ret.trivias.comments() {
            // let s = comment.real_span().source_text(&source_text);
            //   println!("{s}");
        }
    }

    if ret.errors.is_empty() {
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
            println!("Parsed with Errors.");
        }
    }

    //  serde_json::to_string_pretty(&ret.program).unwrap()
    Ok(serde_json::to_string_pretty(&ret.program).unwrap())
}