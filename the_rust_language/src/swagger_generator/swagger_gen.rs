use crate::enum_generator::enum_generator::{filter_enums, write_file};
use crate::models::swagger_format::SwaggerFormat;
use crate::swagger_generator::model_generator::model_generator::filter_models;
use colored::*;
use reqwest::blocking::get;
use serde_json::Value;
use std::fs::{self};

const SWAGGER_URL: &str =
    "https://natcom-api-development.azurewebsites.net/swagger/v1/swagger.json";

const ENUMS_PATH: &str = "src/swagger_generator/enums/";

pub fn write_enum_files(enums_array: Vec<(&String, &Value)>) {
    for (key, value) in enums_array.into_iter() {
        write_file(ENUMS_PATH, (key, value));
    }
}

pub fn clean_directory(path: &str) {
    let dir_contents = fs::read_dir(path).unwrap();
    for file in dir_contents {
        let file_un = file.unwrap();
        let file_path = path.to_owned() + file_un.file_name().to_str().unwrap();
        let fp_clone = file_path.clone();

        fs::remove_file(file_path)
            .map_err(|err| println!("{:?}", err))
            .ok();

        println!("{} {}", "Deleted File :".red(), fp_clone);
    }
}

pub fn get_data() {
    let res = match get(SWAGGER_URL) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let json_response = match res.json::<SwaggerFormat>() {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Failed to parse JSON : {}", err),
    };

    let components = match json_response.components.as_object() {
        Some(paths_array) => Some(paths_array),
        None => None,
    };

    let components = components.unwrap();

    let enums_file_configs = filter_enums(components);

    clean_directory(ENUMS_PATH);
    println!("{}", "Writing enums...".cyan());
    write_enum_files(enums_file_configs);

    filter_models(components)
}
