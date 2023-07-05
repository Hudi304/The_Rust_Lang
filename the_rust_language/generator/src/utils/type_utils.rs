use serde_json::Value;

pub fn clean_model_name(name: &String) -> String {
    return name.replace("`", "");
}

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

pub fn clean_ref_name(reference: String) -> String {
    return reference.split("/").last().unwrap().to_owned();
}
