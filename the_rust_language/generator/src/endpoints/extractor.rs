use serde_json::{Map, Value};

use crate::{common::import::Import, utils::type_utils::extract_type};

use super::{
    http_method::HttpMethod,
    param::param::{EndpointParam, ParamPlace},
};

macro_rules! unwrap_or_continue {
    ($res:expr, $path:ident) => {
        match $res {
            Some(val) => val,
            None => {
                println!("An error: {}; skipped.", stringify!($path));
                continue;
            }
        }
    };
}

macro_rules! unwrap_or_return_default {
    ($res:expr, $default:expr) => {
        match $res {
            Some(req_body) => req_body,
            None => return $default,
        }
    };
    ($res:expr, $default:expr, $warning_message:expr) => {
        match $res {
            Some(req_body) => req_body,
            None => {
                println!("{}", stringify!($warning_message));
                return $default;
            }
        }
    };
}

pub fn extract_endpoints(paths: &Map<String, Value>) {
    for (path, methods) in paths.iter() {
        let methods = unwrap_or_continue!(methods.as_object(), path);

        for (http_method, values) in methods.into_iter() {
            let mtd = HttpMethod::new(http_method);
            let _endpoint_schema = EndpointSchema::build(mtd, path, values);
            // println!("{http_method} {path}");
            // println!(
            //     "{} {:?} ",
            //     _endpoint_schema.request_body_type.0, _endpoint_schema.request_body_type.1
            // );

            println!("{}, {:#?}", _endpoint_schema.path, _endpoint_schema)
        }
    }
}

#[derive(Debug)]
pub struct EndpointSchema {
    pub path: String,
    pub http_method: HttpMethod,
    pub controller_name: String,
    pub fn_name: String,
    pub path_params: Vec<EndpointParam>,
    pub query_params: Vec<EndpointParam>,

    pub request_body_type: (String, Option<Import>), // TODO maybe also request body types
    pub response_type: (String, Option<Import>), // TODO: if the response types are different duplicate the Api call with different headers (text/plain for example)
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

        let (request_type, import) = get_request_body_type(values);

        return EndpointSchema {
            path: path.clone(),
            http_method: mtd,
            controller_name: ctrl_name,
            fn_name: fn_name,
            path_params: path_params,
            query_params: query_params,
            response_type: get_return_types(values),
            request_body_type: (request_type, import),
            tags: tags,
        };
    }
}

fn get_request_body_type(values: &Value) -> (String, Option<Import>) {
    let default = (String::from("any"), None);

    let req_body = unwrap_or_return_default!(values.get("requestBody"), default);
    let req_body = unwrap_or_return_default!(req_body.get("content"), default);
    let req_body = unwrap_or_return_default!(req_body.get("application/json"), default);
    let req_body = unwrap_or_return_default!(req_body.as_object(), default);

    for (req_type, value) in req_body.into_iter() {
        return extract_type(value, "schema");
    }

    return ("any".to_owned(), None);
}
// returns any if any of the check fail
// TODO there is a bug here test with /supplier-portal/invoices
fn get_return_types(endpoints_values: &Value) -> (String, Option<Import>) {
    let any = String::from("any");
    let default: (String, Option<Import>) = (String::from("any"), None);

    let responses = endpoints_values.get("responses");
    let responses = unwrap_or_return_default!(responses, default);

    let err_msg = "Endpoint 'responses' is not and object";
    let response = unwrap_or_return_default!(responses.as_object(), default, err_msg);

    for (status_code, values) in response.into_iter() {
        match status_code.as_str() {
            "200" => (),
            "201" => (),
            _ => continue,
        }
        let err_msg = "Endpoint 'responses' is not and object";
        let resp_type = unwrap_or_return_default!(values.get("content"), default);
        let resp_type = unwrap_or_return_default!(resp_type.get("application/json"), default);
        let resp_type = unwrap_or_return_default!(resp_type.get("schema"), default);
        let response_type = extract_type(resp_type, "items");

        return response_type;
    }

    return default;
}

fn get_parameters(endpoints_values: &Value) -> (Vec<EndpointParam>, Vec<EndpointParam>) {
    let default = (vec![], vec![]);
    let parameters = endpoints_values.get("parameters");
    let parameters = unwrap_or_return_default!(parameters, default);
    let parameters = unwrap_or_return_default!(parameters.as_array(), default);

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
    let tags = unwrap_or_return_default!(endpoints_values.get("tags"), vec![]);

    let tags: &Vec<Value> = tags.as_array().unwrap();

    if tags.len() > 1 {
        println!("Warning: more then one tag was found")
    }

    tags.into_iter()
        .map(|tag| tag.as_str().unwrap_or("").to_owned())
        .collect()
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
