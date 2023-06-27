pub mod config;
pub mod http;

use reqwest::blocking::get;
use serde::{self, Deserialize, Serialize};


use serde_json::{};
#[derive(Deserialize)]
struct SwaggerSchema {
    schema_type: String,
    //TODO check if you can add a type for the value
    properties: serde_json::Value,
    additionalProperties: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaProperty {
    property_type: String,
    nullable: bool,
}

struct ModelSchema {
    schema_type: String,
    //TODO check if you can add a type for the value
    properties: serde_json::Value,
    additionalProperties: bool,
}

struct EnumSchema {
    enum_values: Vec<String>,
    enum_type: String,
}

// TODO try implementing Debug for external struct
// impl fmt::Debug for serde_json::Map<K, V> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         for (key, value) in self.into_iter() {
//             write!(f, "[{} : {}]", key, value)
//         }
//         return fmt::Result;
//     }
// }

fn filter_schemas(schemas: serde_json::Value) {
    let schemas: serde_json::Map<String, serde_json::Value> = match schemas {
        serde_json::Value::Object(map) => map,
        _ => panic!(""),
    };
}

fn main() {
    let settings_path = "./local.json".to_owned();
    let locals = config::get_locals(settings_path);
    let swagger_json_url = locals.swagger_url + &locals.swagger_path[..];
    let response_body = http::get_swagger_schema(swagger_json_url);

    let schemas = response_body.components.schemas;
    let paths = response_body.paths;

    let paths: serde_json::Map<String, serde_json::Value> = match paths {
        serde_json::Value::Object(map) => map,
        _ => panic!(""),
    };

    let schemas: serde_json::Map<String, serde_json::Value> = match schemas {
        serde_json::Value::Object(map) => map,
        _ => panic!(""),
    };

    // for (schema_name, schema_value) in schemas.into_iter() {
    //     println!("{:?}", schema_name);
    // }

    for (path_name, path_value) in paths.into_iter() {
        println!("{:?}", path_name);
    }

    println!("Hello, world!");
}
