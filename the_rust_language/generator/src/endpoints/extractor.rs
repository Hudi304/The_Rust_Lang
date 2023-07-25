use serde_json::{Map, Value};

use crate::{common::import::Import, unwrap_or_return_default};

use super::{
    http_method::HttpMethod,
    param::param::{EndpointParam, ParamPlace}, extract_tags::get_tags, extract_req_body::get_request_body_type, extract_ret_type::get_return_types, extractor_fn_name::get_fn_name,
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

pub fn extract_endpoints(paths: &Map<String, Value>) {
    for (path, methods) in paths.iter() {
        let methods = unwrap_or_continue!(methods.as_object(), path);

        for (http_method, values) in methods.into_iter() {
            let mtd = HttpMethod::new(http_method);
            let _endpoint_schema = EndpointSchema::build(mtd, path, values);
            // println!("{http_method} {path}");
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
        let (request_type, import) = get_request_body_type(values);

        let (path_params, query_params) = build_parameters(values);

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

fn build_parameters(endpoints_values: &Value) -> (Vec<EndpointParam>, Vec<EndpointParam>) {
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

#[cfg(test)]
mod extractor {

    use super::*;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    #[ignore]
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

#[cfg(test)]
mod test_unwrap_or_return_default {

    use crate::unwrap_or_return_default;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    fn multiple_methods_endpoint() {
        // Arrange
        let clo = |opt: Option<String>| -> String {
            let rez = unwrap_or_return_default!(opt, "default".to_owned(), "warning");
            return rez;
        };
        // Act

        let res = clo(Some("test".to_owned()));

        // Assert
        assert_eq!(res.eq("test"), true);

        // let res = clo(None);
    }
}
