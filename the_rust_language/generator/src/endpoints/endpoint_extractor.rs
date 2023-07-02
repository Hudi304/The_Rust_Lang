use serde_json::{Map, Value};

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}

pub enum ParameterType {
    PATH,
    QUERY,
}
struct EndpointParameter {
    name: String,
    placement: ParameterType,
    _type: String,
    isRequired: bool,
}

struct RequestBody {
    _type: String,
}

#[derive(Debug)]
struct EndpointSchema {
    path: String,
    // http_method: HttpMethod,
    // tags: Vec<String>,
    // fn_name: String,
    // params: Vec<EndpointParameter>,
    // request_body: Option<RequestBody>,
    // responses: Map<ui32, String>, // status code  = key , String = type
}

// impl Debug for EndpointSchema {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} : {{\n  props : {:#?} }}", self.path)
//     }
// }

impl EndpointSchema {
    fn build((model_name, model_properties): (&String, &Value)) -> EndpointSchema {
        return EndpointSchema {
            path: model_name.clone(),
        };
    }
}

pub fn extract_endpoints(endpoints: &Map<String, Value>) {
    for (name, value) in endpoints.iter() {
        let endpoint_schema = EndpointSchema::build((name, value));
        println!("{:#?}", endpoint_schema);
    }
}
