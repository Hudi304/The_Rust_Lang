use serde_json::{Map, Value};
use std::fmt;
use std::fmt::Debug;

pub enum PropType {
    Bool,
    String,
    Number,
    Object,
}

impl PropType {
    pub fn to_str(&self) -> &str {
        match self {
            PropType::Bool => "bool",
            PropType::String => "string",
            PropType::Number => "number",
            PropType::Object => "any",
        }
    }
}

pub struct PropertySchema {
    name: String,
    prop_type: PropType,
    is_array: bool,
}

fn get_type() {
    //TODO from the config file
    todo!()
}

impl Debug for PropertySchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n   {} {}", self.name, self.is_array)
    }
}

fn get_ref(prop_value: &Value) -> &str {
    let first_level_ref = prop_value.get("#ref");
    let items_ref = prop_value.get("items");

    match (first_level_ref, items_ref) {
        (Some(first_level), None) => first_level.as_str().unwrap(),
        (None, Some(itm_ref)) => itm_ref.get("#ref").unwrap().as_str().unwrap(),
        (None(x), None(y)) => "any", //honest to god any
        (Some(x), Some(y)) => panic!("this should not be possible"),
    }
}

impl PropertySchema {
    pub fn build((prop_name, prop_value): (&String, &Value)) -> PropertySchema {
        let prop_name = prop_name;
        let value = prop_value.as_object().unwrap();
        let mut prop_type_string: String = "".to_owned();

        let t = value.get("type");
        let r = value.get("#ref");

        let prop_type = match t {
            Some(prop_type) => handle_type(prop_type),
            _ => handle_no_type(prop_value),
        };

        let type_name = match (t, r) {
            (Some(t), Some(r)) => handle_ss(t, r),
            (None, Some(r)) => todo!(),
            (Some(t), None) => todo!(),
            (None, None) => todo!(),
        };

        fn handle_no_type(prop_value: &Value) -> String {
            let prop_type = get_ref(prop_value);
            return prop_type.to_owned();
        }

        fn handle_type(prop_type: &Value) -> String {
            prop_type.as_str().expect("type is not a string").to_owned()
        }

        fn handle_ss(prop_type: &Value, reference: &Value) {}

        if type_option.is_some() {
            prop_type_string = type_option.unwrap().as_str().unwrap().to_owned();
        } else {
            let reference = value
                .get("$ref")
                .unwrap()
                .as_str()
                .unwrap()
                .split("/")
                .last()
                .unwrap()
                .to_owned();
            prop_type_string = reference;
        }

        let prop_type = match prop_type_string.as_str() {
            "string" => PropType::String,
            "number" => PropType::Number,
            "bool" => PropType::Bool,
            "object" => PropType::Object,
            something_else => panic!(" \n Unmatched type |{:?}| \n", something_else),
        };

        return PropertySchema {
            name: prop_name.clone(),
            prop_type,
            is_array: false,
        };
    }
}
