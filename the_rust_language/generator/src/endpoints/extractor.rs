use serde_json::{Map, Value};

use crate::{common::import::Import, utils::type_utils::extract_type};

use super::{
    http_method::HttpMethod,
    param::param::{EndpointParam, ParamPlace},
};

#[derive(Debug)]
pub struct EndpointSchema {
    pub path: String,
    pub http_method: HttpMethod,
    pub controller_name: String,
    pub fn_name: String,
    pub path_params: Vec<EndpointParam>,
    pub query_params: Vec<EndpointParam>,

    pub response_type: (String, Option<Import>),
    // request_body: Option<RequestBody>,
    // responses: Map<ui32, String>, // status code  = key , String = type
    pub tags: Vec<String>,
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
            response_type: get_return_types(values),

            tags: tags,
        };
    }
}

// returns any if any of the check fail
// TODO there is a bug here test with /supplier-portal/invoices
fn get_return_types(endpoints_values: &Value) -> (String, Option<Import>) {
    let any = String::from("any");

    let responses = match endpoints_values.get("responses") {
        Some(models_per_status_code) => models_per_status_code,
        None => return (any, None),
    };

    let mut result = (any.clone(), None);

    let endpoint_responses = responses
        .as_object()
        .expect("Endpoint 'responses' is not and object ");

    for (status_code, values) in endpoint_responses.into_iter() {
        match status_code.as_str() {
            "200" => (),
            "201" => (),
            _ => continue,
        }

        let context = match values.get("content") {
            Some(ctx) => ctx,
            None => return (any, None),
        };

        let app_json = match context.get("application/json") {
            Some(app_json) => app_json,
            None => return (any, None),
        };

        let type_schema = match app_json.get("schema") {
            Some(type_schema) => type_schema,
            None => return (any, None),
        };

        let response_type = extract_type(type_schema, "items");

        result = response_type;
    }

    return result;
}

fn get_parameters(endpoints_values: &Value) -> (Vec<EndpointParam>, Vec<EndpointParam>) {
    let parameters = match endpoints_values.get("parameters") {
        Some(params) => params,
        None => return (vec![], vec![]),
    };

    let parameters = parameters
        .as_array()
        .expect("Endpoint parameters is not an array");

    let mut path_params: Vec<EndpointParam> = vec![];
    let mut query_params: Vec<EndpointParam> = vec![];

    for param in parameters.into_iter() {
        let param = EndpointParam::build(param);

        match param.param_place {
            ParamPlace::PATH => path_params.push(param),
            ParamPlace::QUERY => query_params.push(param),
        }
    }

    return (path_params, query_params);
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

    if tags.len() > 1 {
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
            let mtd = HttpMethod::new(http_method);
            let _endpoint_schema = EndpointSchema::build(mtd, path, values);
            println!("{http_method} {path}");
            println!("{:#?}", _endpoint_schema);
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
        // Arrange
        let number_data = r#"{ }"#;

        // Act
        let prop_json: Value = serde_json::from_str(number_data).unwrap();
        let (return_type, import) = get_return_types(&prop_json);
        // println!("{}, {:?} ", return_type, import);

        // Assert
        assert_eq!(return_type.eq("any"), true);
        assert_eq!(import.is_none(), true);
    }
}
