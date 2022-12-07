use crate::swagger_generator::models::swagger_format::SwaggerFormat;
use reqwest::blocking::get;
use serde_json::{Map, Value};
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;

const Swagger_URL: &str =
    "https://natcom-api-development.azurewebsites.net/swagger/v1/swagger.json";

const ENUMS_PATH: &str = "src/swagger_generator/enums/";

const ENUMS_DEFAULT_DECLARATION: &str = "export enum";

//TODO create directories if they don't exist
pub fn write_file(path: &str, (key, value): (&String, &Value)) {
    let file_path = path.to_owned() + key + ".enum.ts";
    let mut file = File::create(file_path).unwrap();
    let file_content = get_file_content((key, value));
    file.write_all(file_content.as_bytes()).unwrap();
}

//TODO make this return a result
pub fn get_file_content((enum_name, value): (&String, &Value)) -> String {
    if !value.is_array() {
        return String::from("");
    }

    let enum_declaration = String::from(ENUMS_DEFAULT_DECLARATION);
    let enum_body = get_enum_content((enum_name, value));
    let result = format!("{enum_declaration} {enum_name} {{ \n {enum_body} }}");
    return result;
}

pub fn get_enum_content((_, value): (&String, &Value)) -> String {
    let mut content = Vec::new();
    let values_array = value.as_array().unwrap();
    for enum_value in values_array.into_iter() {
        let enum_value_string = enum_value.to_string().replace("\"", "");
        writeln!(content, "{enum_value_string} = \"{enum_value_string}\",")
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
    let buffer = String::from_utf8(content).unwrap();
    return buffer;
}

pub fn write_enum_files(enums_array: Vec<(&String, &Value)>) {
    for (key, value) in enums_array.into_iter() {
        // println!("\n {} : {} ", key, value);
        write_file(ENUMS_PATH, (key, value));
    }
}

pub fn filter_enums(components: &Map<String, Value>) -> Vec<(&String, &Value)> {
    let schemas = components.get("schemas").unwrap();
    let schemas = schemas.as_object().unwrap();
    let schemas_keys = schemas.keys();

    let mut enums_array: Vec<(&String, &Value)> = Vec::new();

    for model_name in schemas_keys {
        let schema = schemas.get_key_value(model_name).unwrap();
        let (_, value) = schema;
        let schema_value = value.as_object().unwrap();

        if schema_value.contains_key("enum") {
            let (_, values_array) = schema_value.get_key_value("enum").unwrap();
            enums_array.push((model_name, values_array));
        }
    }

    return enums_array;
}

pub fn get_data() {
    let res = match get(Swagger_URL) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let json_response = match res.json::<SwaggerFormat>() {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Failed to parse JSON : {}", err),
    };

    let swagger_paths = match json_response.paths.as_object() {
        Some(paths_array) => Some(paths_array),
        None => None,
    };

    let components = match json_response.components.as_object() {
        Some(paths_array) => Some(paths_array),
        None => None,
    };

    let swagger_paths = swagger_paths.unwrap();
    let components = components.unwrap();

    let enums_file_configs = filter_enums(components);
    // println!("Writing enums...".cyan());
    write_enum_files(enums_file_configs);
}
