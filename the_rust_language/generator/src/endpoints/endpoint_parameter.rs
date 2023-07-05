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
    // find a better name of this
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

        let ref_param_type = get_ref(param_value, "schema");
        let param_type = param_value.get("schema").unwrap().get("type");

        let mut par_type = String::from("Default_param_type");
        if let Some(ref_name) = ref_param_type {
            par_type = ref_name;
        };
        if let Some(param_val) = param_type {
            par_type = param_val.as_str().unwrap().to_owned();
        };

        EndpointParameter {
            param_place: param_place,
            param_type: par_type,
            param_name: param_value
                .get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        }
    }
}
