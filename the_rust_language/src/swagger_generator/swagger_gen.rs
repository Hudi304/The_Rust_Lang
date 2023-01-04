use std::fs;

use crate::swagger_generator::enum_generator::{filter_enums, get_file_content};
use crate::swagger_generator::generator_utils::{delete_dir_contents, write_file};
use crate::swagger_generator::model_generator::{get_model_file_content, MODELS_PATH};
use crate::{
    generator_models::swagger_format::SwaggerFormat,
    swagger_generator::model_generator::filter_models,
};
use colored::*;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::Value;

const SWAGGER_URL: &str = "http://localhost:41000/swagger/v1/swagger.json";
const ENUMS_PATH: &str = "src/swagger_generator/enums/";

pub fn write_enum_files(enums_array: &Vec<(&String, &Value)>) {
    println!("{}", "Writing enums...".cyan());
    for (file_name, value) in enums_array.into_iter() {
        let file_path = ENUMS_PATH.to_owned() + file_name + ".enum.ts";
        let file_content = get_file_content((file_name, value));
        write_file(&file_path, &file_content);
    }
}

pub fn write_model_files(models_array: &Vec<(&String, &Value)>) {
    println!("{}", "Writing models...".cyan());
    for file_data in models_array.into_iter() {
        match get_model_file_content(*file_data, models_array) {
            None => continue,
            Some((file_path, file_content)) => write_file(&file_path, &file_content),
        }
    }
}

#[derive(Deserialize)]
pub struct ReplaceAllSettings {
    // filters: Filters,
    // replace: Replace,
}

pub fn read_toml() -> ReplaceAllSettings {
    let settings_path = "src/tools/replace_all_config.toml";

    let mut config = match fs::read_to_string(settings_path) {
        Err(why) => {
            panic!("{}: {}", "couldn't open \n".red(), why);
        }
        Ok(file) => file,
    };

    let config: ReplaceAllSettings = match toml::from_str(&config) {
        Ok(d) => d,
        Err(_) => {
            panic!("Unable to load data from ");
        }
    };

    return config;
}

pub fn get_data() {
    let enums_path = String::from(ENUMS_PATH);
    let models_path = String::from(MODELS_PATH);

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

    delete_dir_contents(&enums_path);
    write_enum_files(&enums_file_configs);

    let model_file_configs = filter_models(components);

    delete_dir_contents(&models_path);
    write_model_files(&model_file_configs);
}
