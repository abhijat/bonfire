#[macro_use]
extern crate serde_json;

use std::fs::read_to_string;
use std::path::PathBuf;

use rustpython_parser::parser::parse_program;
use structopt::StructOpt;

use crate::schema_filters::{ClassDefinition, schema_class_definitions};

mod utils;
mod schema_filters;
mod statement_filters;

fn main() {
    let options = Opt::from_args();
    let expression = read_to_string(&options.roast).unwrap();
    let parsed = parse_program(expression.as_str()).unwrap();
    let schemas: Vec<ClassDefinition> = parsed
        .statements
        .iter()
        .flat_map(schema_class_definitions)
        .collect();

    schemas
        .iter()
        .map(ClassDefinition::to_json)
        .for_each(|v| {
            if options.prettify {
                eprintln!("{}", serde_json::to_string_pretty(&v).unwrap())
            } else {
                eprintln!("{}", serde_json::to_string(&v).unwrap())
            }
        });
}


#[derive(Debug, StructOpt)]
#[structopt(name = "schema-translator", about = "A small utility to convert marshmallow schema to json-schema")]
struct Opt {
    #[structopt(parse(from_os_str))]
    roast: PathBuf,

    #[structopt(short, long)]
    prettify: bool,
}
