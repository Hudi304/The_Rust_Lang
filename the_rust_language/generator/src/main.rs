pub mod config;
pub mod http;

mod endpoints;
mod enums;
mod model;

mod common;
mod utils;

use endpoints::extractor;
use model::model_extractor;

use serde_json::{Map, Value};

use colored::*;

fn pipeline() {
    let settings_path = "./local.json".to_owned();
    let locals = config::get_locals(settings_path);
    let swagger_json_url = locals.swagger_url + &locals.swagger_path[..];
    let response_body = http::get_swagger_schema(swagger_json_url);

    let schemas = response_body.components.schemas;
    let paths = response_body.paths;

    let paths: Map<String, Value> = match paths {
        Value::Object(map) => map,
        _ => panic!("Root json does not have a 'paths' key"),
    };

    extractor::extract_endpoints(&paths);

    let schemas: Map<String, Value> = match schemas {
        Value::Object(map) => map,
        _ => panic!("Root json does not have a 'schemas' key"),
    };

    //TODO can you do this without cloning maybe reference counting ?

    let models: Map<String, Value> = schemas
        .clone()
        .into_iter()
        .filter(|(_, sch_value)| {
            sch_value
                .as_object()
                .expect("Schema is not an object")
                .get("properties")
                .is_some()
        })
        .collect();

    let _enums: Map<String, Value> = schemas
        .clone()
        .into_iter()
        .filter(|(_, sch_value)| {
            sch_value
                .as_object()
                .expect("Schema is not an object")
                .get("enum")
                .is_some()
        })
        .collect();

    model_extractor::extract_models(&models);
    extractor::extract_endpoints(&paths);
}

// TODO implement some kind of testing for a whole swagger.json
// TODO implement a global warning system for stuff like finding some primitive type that is not yet defined
fn main() {
    pipeline();

    let var = "this is blue".blue();

    println!("{}", var);
    // println!("--------------------------------------------!".blue());
}
