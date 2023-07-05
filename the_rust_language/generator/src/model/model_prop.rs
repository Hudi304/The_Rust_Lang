use serde_json::Value;
use std::fmt;
use std::fmt::Debug;

use crate::utils::type_utils::get_ref;
use crate::Import;

// TODO this might be a good idea when
// pub enum PropType {
//     Bool,
//     String,
//     Number,
//     Object,
// }

pub struct PropertySchema {
    pub name: String,
    pub prop_type: String,
    pub import: Option<Import>,
}

impl Debug for PropertySchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} - ({:?})", self.name, self.prop_type, self.import)
    }
}

impl PropertySchema {
    pub fn build((prop_name, prop_value): (&String, &Value)) -> PropertySchema {
        let (prop_type, import_option) = get_prop_type(prop_value);

        PropertySchema {
            name: prop_name.clone(),
            prop_type,
            import: import_option,
        }
    }
}
// TODO this needs a refactor
// TODO these 2 function can be a single big one
// ! implement generic here
pub fn get_prop_type(full_prop: &Value) -> (String, Option<Import>) {
    let prop_type = full_prop.get("type");
    let items = full_prop.get("items");
    let ref_type = get_ref(&full_prop, "items");

    let import_option = match ref_type {
        Some(ref reference) => Some(Import {
            name: reference.clone().to_owned(),
            path: "".to_owned(),
        }),
        None => None,
    };

    let (a, b) = match (prop_type, ref_type) {
        (Some(pt), Some(rt)) => {
            let prop_type_str = pt.as_str().unwrap().to_owned();
            if prop_type_str.eq("array") {
                return (rt + "[]", import_option);
            }
            return (rt, import_option);
        }
        (Some(pt), None) => {
            let prop_type_str = pt.as_str().unwrap().to_owned();
            if prop_type_str.eq("array") {
                let items_type = items.unwrap().get("type").unwrap().as_str().unwrap();
                return (items_type.to_owned() + "[]", import_option);
            }
            return (pt.as_str().unwrap().to_owned(), import_option);
        }
        (None, Some(rt)) => return (rt, import_option),
        (None, None) => return (String::from("any"), None),
    };

    return (a, b);

    // return (prop_type, import_option);
}

#[cfg(test)]
mod primitive_types {

    use super::*;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    fn prop_type_number() {
        let number_data = r#"{ "type": "number" }"#;

        let prop_json: Value = serde_json::from_str(number_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("number"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn prop_type_string() {
        let string_data = r#"{ "type": "string" }"#;

        let prop_json: Value = serde_json::from_str(string_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("string"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn prop_type_bool() {
        let bool_data = r#"{ "type": "bool" }"#;

        let prop_json: Value = serde_json::from_str(bool_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("bool"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn prop_type_obj_type() {
        let obj_data = r#"{ "type": "object" }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("object"), true);
        assert_eq!(imp.is_none(), true);
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
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("string[]"), true);
        assert_eq!(imp.is_none(), true);
    }
}

#[cfg(test)]
mod object_schema_types {

    use super::*;

    #[test]
    fn prop_type_obj_schema() {
        let obj_data = r#"
        {
            "$ref": "/components/schemas/Sort10"
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("Sort10"), true);
        assert_eq!(imp.unwrap().name.eq("Sort10"), true);
    }

    #[test]
    fn prop_type_obj_schema_array() {
        let obj_data = r#"
        {
            "type": "array",
            "items": {
                "$ref": "/components/schemas/Sort10"
                }
        }"#;

        let prop_json: Value = serde_json::from_str(obj_data).unwrap();
        let (props_type, imp) = get_prop_type(&prop_json);

        assert_eq!(props_type.eq("Sort10[]"), true);
        assert_eq!(imp.unwrap().name.eq("Sort10"), true);
    }
}
