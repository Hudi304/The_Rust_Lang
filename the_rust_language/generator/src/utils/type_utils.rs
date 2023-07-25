use serde_json::Value;

use crate::common::import::Import;

pub fn clean_model_name(name: &String) -> String {
    return name.replace("`", "");
}

pub fn clean_ref_name(reference: String) -> String {
    return reference.split("/").last().unwrap().to_owned();
}

// TODO idea try to find the type, schema and ref by iterating recursively over the &Value you get as param

// TODO add docs
// this takes the body of a prop and it returns the name of the schema
// that is referred
pub fn get_ref(prop_value: &Value, secondary_key: &str) -> Option<String> {
    let first_level_ref = prop_value.get("$ref");
    let items_ref = prop_value.get(secondary_key);

    match (first_level_ref, items_ref) {
        (Some(_), Some(items)) => {
            if items.get("$ref").is_none() {
                return None;
            }
            let reference = items.get("$ref").unwrap();
            let reference = reference.as_str().unwrap();
            let reference = reference.to_owned();
            let result = clean_ref_name(reference);
            return Some(result);
        }
        (None, Some(items)) => {
            if items.get("$ref").is_none() {
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

// TODO this needs a refactor
// TODO these 2 function can be a single big one
// ! implement generic here
pub fn extract_type(schema: &Value, secondary_key: &str) -> (String, Option<Import>) {
    let type_key = "type";
    let prop_type: Option<&Value> = schema.get(type_key);
    let compute_schema_type = |p_type: &Value| Some(p_type.as_str().unwrap_or("any").to_owned());
    let prop_type = prop_type.and_then(compute_schema_type);
    let items = schema.get(secondary_key);
    let ref_type = get_ref(&schema, secondary_key);

    let import = match ref_type {
        // TODO :  implement build for Import as well
        Some(ref reference) => Some(Import {
            name: reference.clone().to_owned(),
            path: "".to_owned(),
        }),
        None => None,
    };

    let (schema_type, import) = match (prop_type, ref_type) {
        (Some(st), Some(rt)) => match st.eq("array") {
            true => (rt + "[]", import),
            false => (rt, import),
        },
        (Some(st), None) => {
            // TODO this still needs a refactor
            if st.eq("array") {
                if items.is_some() {
                    let items_type = items.unwrap().get(type_key).unwrap().as_str().unwrap();
                    return (items_type.to_owned() + "[]", import);
                }
                return (st, import);
            }
            return (st, import);
        }
        (None, Some(rt)) => return (rt, import),
        (None, None) => return (String::from("any"), None),
    };

    return (schema_type, import);
}

#[cfg(test)]
mod prim_extract_type {

    use super::*;

    static SECONDARY_KEY: &str = "items";

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition

    // Tests name "{function_name} {scenario} {expected_behavior}"
    #[test]
    fn number_no_import() {
        // Arrange
        let number_data = r#"{ "type": "number" }"#;

        // Act
        let schema: Value = serde_json::from_str(number_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        // Assert
        assert_eq!(schema_type.eq("number"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn string_no_import() {
        let string_data = r#"{ "type": "string" }"#;

        let schema: Value = serde_json::from_str(string_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("string"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn bool_no_import() {
        let bool_data = r#"{ "type": "bool" }"#;

        let schema: Value = serde_json::from_str(bool_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("bool"), true);
        assert_eq!(imp.is_none(), true);
    }

    // You sure this is what you want?
    #[test]
    fn object_no_import() {
        let obj_data = r#"{ "type": "object" }"#;

        let schema: Value = serde_json::from_str(obj_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("object"), true);
        assert_eq!(imp.is_none(), true);
    }

    #[test]
    fn string_array_no_import() {
        let obj_data = r#"
        {
            "type": "array",
            "items": {
                "type": "string",
                "format": "uuid"
            }
        }"#;

        let schema: Value = serde_json::from_str(obj_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("string[]"), true);
        assert_eq!(imp.is_none(), true);
    }
}

#[cfg(test)]
mod obj_extract_type {

    use super::*;

    static SECONDARY_KEY: &str = "items";

    #[test]
    fn top_level_ref_model_name_and_import() {
        let obj_data = r#"
        {
            "$ref": "/components/schemas/Sort10"
        }"#;

        let schema: Value = serde_json::from_str(obj_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("Sort10"), true);
        assert_eq!(imp.unwrap().name.eq("Sort10"), true);
    }

    #[test]
    fn imbricated_ref_model_name_array_and_import() {
        let obj_data = r#"
        {
            "type": "array",
            "items": {
                "$ref": "/components/schemas/Sort10"
                }
        }"#;

        let schema: Value = serde_json::from_str(obj_data).unwrap();
        let (schema_type, imp) = extract_type(&schema, SECONDARY_KEY);

        assert_eq!(schema_type.eq("Sort10[]"), true);
        assert_eq!(imp.unwrap().name.eq("Sort10"), true);
    }
}

#[cfg(test)]
mod endpoint_return_type {

    use std::fs;

    use super::*;

    static SECONDARY_KEY: &str = "items";

    pub fn read_json(path: &str) -> Value {
        let error_message = format!("\n     Could not read {path}.json file \n");
        let file = &fs::read_to_string(path).expect(&error_message);
        return serde_json::from_str::<Value>(&file).unwrap();
    }

    #[test]
    fn top_level_ref_model_name_and_import() {
        // Arrange
        let obj_data = read_json("./src/utils/test_case_1.json");

        let schema = obj_data.get("/path").unwrap();
        let schema = schema.get("get").unwrap();
        let schema = schema.get("responses").unwrap();
        let schema = schema.get("200").unwrap();
        let schema = schema.get("content").unwrap();
        let schema = schema.get("application/json").unwrap();
        let schema = schema.get("schema").unwrap();

        // Act
        let (schema_type, imp) = extract_type(schema, SECONDARY_KEY);
        // println!("\n\n schema_type : {} \n\n", schema_type);
        // println!("\n\n import : {:?} \n\n", imp);

        // Assert
        assert_eq!(schema_type.eq("Model[]"), true);
        assert_eq!(imp.unwrap().name.eq("Model"), true);
    }
}
