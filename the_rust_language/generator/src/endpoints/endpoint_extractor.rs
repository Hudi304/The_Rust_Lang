use serde_json::{Map, Value};

use super::{
    endpoint_parameter::{EndpointParameter, ParamPlace},
    http_method::HttpMethod,
};

pub enum ParameterType {
    PATH,
    QUERY,
}
// struct EndpointParameter {
//     name: String,
//     placement: ParameterType,
//     _type: String,
//     isRequired: bool,
// }

struct RequestBody {
    _type: String,
}

#[derive(Debug)]
struct EndpointSchema {
    path: String,
    http_method: HttpMethod,
    controller_name: String,
    fn_name: String,
    path_params: Vec<EndpointParameter>,
    query_params: Vec<EndpointParameter>,

    // request_body: Option<RequestBody>,
    // responses: Map<ui32, String>, // status code  = key , String = type
    tags: Vec<String>,
}

impl EndpointSchema {
    fn build(mtd: HttpMethod, path: &String, values: &Value) -> EndpointSchema {
        // todo should these even be normal strings or can they be references?

        let tags = get_tags(values);
        let ctrl_name = tags.first().unwrap().clone();
        let fn_name = get_fn_name(values);

        let (path_params, query_params) = get_parameters(values);

        return EndpointSchema {
            path: path.clone(),
            http_method: mtd,
            controller_name: ctrl_name,
            fn_name: fn_name,
            path_params: path_params,
            query_params: query_params,

            tags: tags,
        };
    }
}

fn get_return_types(endpoints_values: &Value) -> String {
    let responses = match endpoints_values.get("responses") {
        Some(models_per_status_code) => models_per_status_code,
        None => return String::from("any"),
    };

    String::from("")
}

fn get_parameters(endpoints_values: &Value) -> (Vec<EndpointParameter>, Vec<EndpointParameter>) {
    let parameters = match endpoints_values.get("parameters") {
        Some(params) => params,
        None => return (vec![], vec![]),
    };
    // .expect("Endpoint does not have parameters");

    let parameters = parameters
        .as_array()
        .expect("Endpoint parameters is not an array");

    let mut path_params: Vec<EndpointParameter> = vec![];
    let mut query_params: Vec<EndpointParameter> = vec![];

    for param in parameters.into_iter() {
        let param = EndpointParameter::build(param);

        match param.param_place {
            ParamPlace::PATH => path_params.push(param),
            ParamPlace::QUERY => query_params.push(param),
        }
    }

    (path_params, query_params)
}

fn get_fn_name(endpoints_values: &Value) -> String {
    let fn_name = endpoints_values.get("operationId");

    let fn_name = match fn_name {
        Some(fn_name) => fn_name,
        None => {
            println!("Warning : operationId not found");
            return String::new();
        }
    };

    let fn_name = fn_name.as_str().unwrap_or("default_fn_name");
    fn_name.to_owned()
}

fn get_tags(endpoints_values: &Value) -> Vec<String> {
    let tags = endpoints_values
        .get("tags")
        .expect("Endpoint does not have tags");

    let tags: &Vec<Value> = tags.as_array().unwrap();

    if (tags.len() > 1) {
        println!("Warning: more then one tag was found")
    }

    tags.into_iter()
        .map(|tag| tag.as_str().unwrap_or("").to_owned())
        .collect()
}

pub fn extract_endpoints(paths: &Map<String, Value>) {
    for (path, methods) in paths.iter() {
        let methods = match methods.as_object() {
            Some(methods) => methods,
            None => {
                println!("Warning : {path} has no methods");
                continue;
            }
        };

        for (http_method, values) in methods.into_iter() {
            println!("{http_method} {path}");
            let mtd = HttpMethod::new(http_method);
            let endpoint_schema = EndpointSchema::build(mtd, path, values);
            println!("{:#?}", endpoint_schema);
        }
    }
}

#[cfg(test)]
mod extractor {

    use super::*;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    fn multiple_methods_endpoint() {
        let number_data = r#"{ }"#;

        let prop_json: Value = serde_json::from_str(number_data).unwrap();
        let return_type = get_return_types(&prop_json);

        assert_eq!(return_type.eq("any"), true);
    }
}
