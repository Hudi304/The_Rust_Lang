use reqwest;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggerFormat {
    pub openapi: String,
    pub info: ResponseInfo,
    pub paths: serde_json::Value,
    pub components: SwaggerComponents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggerComponents {
    pub schemas: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseInfo {
    pub title: String,
    pub version: String,
}

pub fn get_swagger_schema(swagger_json_url: String) -> SwaggerFormat {
    let response: reqwest::blocking::Response = match reqwest::blocking::get(swagger_json_url) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let response_body = response
        .text()
        .expect("Could not deserialize response body");

    let response_body: SwaggerFormat = serde_json::from_str(&response_body[..])
        .expect("Could not parse api call response body to json");

    return response_body;
}
