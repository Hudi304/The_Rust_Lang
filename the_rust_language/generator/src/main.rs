pub mod config;
pub mod http;
pub mod io_utils;

mod model;

use model::model_extractor;
use serde::{self, Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Import {
    pub name: String,
    pub path: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct SchemaProperty {
    property_type: String,
    nullable: bool,
}

// struct EnumSchema {
//     enum_values: Vec<String>,
//     enum_type: String,
// }

// TODO try implementing Debug for external struct
// impl fmt::Debug for serde_json::Map<K, V> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         for (key, value) in self.into_iter() {
//             write!(f, "[{} : {}]", key, value)
//         }
//         return fmt::Result;
//     }
// }

// TODO implement a global warning system for stuff like finding some primitive type that is not yet defined
fn main() {
    let settings_path = "./local.json".to_owned();
    let locals = config::get_locals(settings_path);
    let swagger_json_url = locals.swagger_url + &locals.swagger_path[..];
    let response_body = http::get_swagger_schema(swagger_json_url);

    let schemas = response_body.components.schemas;
    let paths = response_body.paths;

    let _paths: Map<String, Value> = match paths {
        Value::Object(map) => map,
        _ => panic!(""),
    };

    let schemas: Map<String, Value> = match schemas {
        Value::Object(map) => map,
        _ => panic!(""),
    };

    //TODO can you do this without cloning?

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

    println!("--------------------------------------------!");

    // for (path_name, path_value) in paths.into_iter() {
    //     println!("{:?}", path_name);
    // }

    println!("Hello, world!");
}
