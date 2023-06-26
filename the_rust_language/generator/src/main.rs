use reqwest::blocking::get;
use serde::{self, Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
struct LocalFileFormat<'a> {
    swagger_url: &'a str,
    swagger_path: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseInfo {
    pub title: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SwaggerFormat {
    pub openapi: String,
    pub info: ResponseInfo,
    pub paths: serde_json::Value,      // components: String,
    pub components: serde_json::Value, // components: String,
}

fn main() {
    let settings_path = "./local.json";

    let locals =
        &fs::read_to_string(settings_path).expect("Could not read ROOT/local.json file")[..];
    let locals = serde_json::from_str::<LocalFileFormat>(locals)
        .expect("Could not deserialize ROOT/local.json file");

    let swagger_json_url = locals.swagger_url.to_owned() + locals.swagger_path;

    println!("{:?}", swagger_json_url);

    let response: reqwest::blocking::Response = match get(swagger_json_url) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let response_body = response
        .text()
        .expect("Could not deserialize response body");

    let response_body: SwaggerFormat = serde_json::from_str(&response_body[..])
        .expect("Could not parse api call response body to json");

    println!("{:?}", response_body.info);

    println!("Hello, world!");
}
