use serde::{self, Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFileFormat {
    pub swagger_url: String,
    pub swagger_path: String,
}

pub fn get_locals(locals_path: String) -> LocalFileFormat {
    let locals = &fs::read_to_string(locals_path).expect("Could not read ROOT/local.json file")[..];
    let locals = serde_json::from_str::<LocalFileFormat>(locals)
        .expect("Could not deserialize ROOT/local.json file");

    return locals;
}
