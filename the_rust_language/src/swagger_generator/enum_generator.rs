//TODO create directories if they don't exist
use std::io::Write;
const ENUMS_DEFAULT_DECLARATION: &str = "export enum";
use serde_json::{Map, Value};

pub fn get_enum_content((_, value): (&String, &Value)) -> String {
    let mut content = Vec::new();
    let values_array = value.as_array().unwrap();

    for enum_value in values_array.into_iter() {
        //TODO handle errors here
        let enum_value_string = enum_value.to_string().replace("\"", "");
        writeln!(content, "{enum_value_string} = \"{enum_value_string}\",")
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    return String::from_utf8(content).unwrap();
}

pub fn get_file_content((enum_name, value): (&String, &Value)) -> String {
    if !value.is_array() {
        return String::from("");
    }

    let enum_declaration = String::from(ENUMS_DEFAULT_DECLARATION);
    let enum_body = get_enum_content((enum_name, value));
    let result = format!("{enum_declaration} {enum_name} {{ \n {enum_body} }}");
    return result;
}

//TODO make this and models take schemas
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
