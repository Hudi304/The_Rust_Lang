use serde_json::{Map, Value};
use std::fmt;
use std::fmt::Debug;

// TODO this might be a good idea when
pub enum PropType {
    Bool,
    String,
    Number,
    Object,
}

pub struct PropertySchema {
    pub name: String,
    pub prop_type: String,
    is_array: bool,
}

impl Debug for PropertySchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n   {} {}", self.name, self.prop_type)
    }
}

impl PropertySchema {
    pub fn build((prop_name, prop_value): (&String, &Value)) -> PropertySchema {
        let prop_type = get_prop_type(prop_value);

        PropertySchema {
            name: prop_name.clone(),
            prop_type,
            is_array: false,
        }
    }
}

// TODO these 2 function can be a single big one
fn get_prop_type(full_prop: &Value) -> String {
    let prop_type = full_prop.get("type");
    let items = full_prop.get("items");
    let ref_type = get_ref(&full_prop);

    let prop_type = match (prop_type, ref_type) {
        (Some(pt), Some(rt)) => {
            let prop_type_str = pt.as_str().unwrap().to_owned();
            if (prop_type_str.eq("array")) {
                return rt + "[]";
            }
            return rt;
        }
        (None, Some(rt)) => return rt,
        (Some(pt), None) => {
            let prop_type_str = pt.as_str().unwrap().to_owned();
            if (prop_type_str.eq("array")) {
                let items_type = items.unwrap().get("type").unwrap().as_str().unwrap();
                return items_type.to_owned() + "[]";
            }
            return pt.as_str().unwrap().to_owned();
        }
        (None, None) => String::from("any"),
    };

    return prop_type;
}

// TODO add docs
// this takes the body of a prop and it returns the name of the schema
// that is referred
fn get_ref(prop_value: &Value) -> Option<String> {
    let first_level_ref = prop_value.get("$ref");
    let items_ref = prop_value.get("items");

    match (first_level_ref, items_ref) {
        (Some(_), Some(items)) => {
            if (items.get("$ref").is_none()) {
                return None;
            }
            let reference = items.get("$ref").unwrap();
            let reference = reference.as_str().unwrap();
            let reference = reference.to_owned();
            let result = clean_ref_name(reference);
            return Some(result);
        }
        (None, Some(items)) => {
            if (items.get("$ref").is_none()) {
                return None;
            }
            let reference = items.get("$ref").unwrap();
            let reference = reference.as_str().unwrap();
            let reference = reference.to_owned();
            let result = clean_ref_name(reference);
            return Some(result);
        }
        (Some(reference), None) => {
            let reference = reference.as_str().unwrap();
            let reference = reference.to_owned();
            let result = clean_ref_name(reference);
            return Some(result);
        }
        (None, None) => None,
    }
}

fn clean_ref_name(reference: String) -> String {
    return reference.split("/").last().unwrap().to_owned();
}

#[cfg(test)]
mod primitive_types {

    use super::*;

    #[test]
    fn prop_type_number() {
        let number_data = r#"
        {
            "type": "number"
        }"#;

        let prop_json: Value = serde_json::from_str(number_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("number"), true);
    }

    #[test]
    fn prop_type_string() {
        let string_data = r#"
        {
            "type": "string"
        }"#;

        let prop_json: Value = serde_json::from_str(string_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("string"), true);
    }

    #[test]
    fn prop_type_bool() {
        let bool_data = r#"
        {
            "type": "bool"
        }"#;

        let prop_json: Value = serde_json::from_str(bool_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("bool"), true);
    }

    #[test]
    fn prop_type_obj_type() {
        let obj_data = r#"
        {
            "type": "object"
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("object"), true);
    }

    #[test]
    fn prop_type_primitive_array() {
        let obj_data = r#"
        {
            "type": "array",
            "items": {
                "type": "string",
                "format": "uuid"
            }
        }
        "#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("string[]"), true);
    }
}

#[cfg(test)]
mod object_schema_types {

    use super::*;

    #[test]
    fn prop_type_obj_array() {
        let obj_data = r#"
        {
            "type": "array",
            "items": {
                "$ref": "/components/schemas/Sort10"
                }
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("Sort10[]"), true);
    }

    #[test]
    fn prop_type_obj_schema() {
        let obj_data = r#"
        {
            "$ref": "/components/schemas/Sort10"
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();

        assert_eq!(get_prop_type(&prop_json).eq("Sort10"), true);
    }
}
