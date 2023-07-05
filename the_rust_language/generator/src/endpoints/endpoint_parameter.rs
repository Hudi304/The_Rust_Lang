use serde_json::{Map, Value};

use crate::utils::type_utils::get_ref;

#[derive(Debug)]
pub enum ParamPlace {
    QUERY,
    PATH,
}

impl ParamPlace {
    pub fn new(method: &str) -> ParamPlace {
        match method {
            "query" => Self::QUERY,
            "path" => Self::PATH,
            unknown_type => {
                println!("Warning : unknown parameter type {unknown_type}");
                return Self::QUERY;
            }
        }
    }
}

#[derive(Debug)]
pub struct EndpointParameter {
    pub param_place: ParamPlace,
    pub param_name: String,
    pub param_type: String,
}

impl EndpointParameter {
    pub fn build(param_value: &Value) -> EndpointParameter {
        let param_place = param_value
            .get("in")
            .expect("Parameter does not specify it's type");

        let param_place = param_place.as_str().expect("key 'in' is not a string");
        let param_place = ParamPlace::new(param_place);

        EndpointParameter {
            param_place: param_place,
            param_type: get_param_type(param_value),
            param_name: get_param_name(param_value),
        }
    }
}

fn get_param_name(param_value: &Value) -> String {
    param_value
        .get("name")
        .expect("Endpoint parameter does not have a name")
        .as_str()
        .expect("Endpoint name is not a string")
        .to_owned()
}

fn get_param_type(param_value: &Value) -> String {
    let ref_param_type = get_ref(param_value, "schema");
    let schema_type_opt = param_value.get("schema").unwrap().get("type");

    let mut param_type = String::from("Default_param_type");

    if let Some(ref_name) = ref_param_type {
        param_type = ref_name;
    };

    if let Some(param_val) = schema_type_opt {
        param_type = param_val.as_str().unwrap().to_owned();
    };

    return param_type;
}
