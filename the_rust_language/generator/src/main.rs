use reqwest::blocking::get;
use serde::{self, Deserialize, Serialize};
use serde_json::Value::Object;
use std::{fs, path};
#[derive(Serialize, Deserialize, Debug)]
struct LocalFileFormat<'a> {
    swagger_url: &'a str,
    swagger_path: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseInfo {
    pub title: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SwaggerFormat {
    pub openapi: String,
    pub info: ResponseInfo,
    pub paths: serde_json::Value,
    pub components: SwaggerComponents,
}
#[derive(Serialize, Deserialize, Debug)]
struct SwaggerComponents {
    schemas: serde_json::Value,
}

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

fn main() {
    let settings_path = "./local.json";

    let locals =
        &fs::read_to_string(settings_path).expect("Could not read ROOT/local.json file")[..];
    let locals = serde_json::from_str::<LocalFileFormat>(locals)
        .expect("Could not deserialize ROOT/local.json file");

    let swagger_json_url = locals.swagger_url.to_owned() + locals.swagger_path;

    println!("{:?}", swagger_json_url);

    let response: reqwest::blocking::Response = match get(swagger_json_url) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let response_body = response
        .text()
        .expect("Could not deserialize response body");

    let response_body: SwaggerFormat = serde_json::from_str(&response_body[..])
        .expect("Could not parse api call response body to json");

    // println!("{:?}", response_body.info);

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
