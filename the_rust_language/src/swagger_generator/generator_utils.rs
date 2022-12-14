use colored::Colorize;
use std::{
    fs::{self, File, ReadDir},
    io::{Error, Write},
};

use crate::swagger_generator::generator_printers::print_create_file_error;

use super::generator_printers::{
    print_delete_file_error, print_delete_file_success, print_write_file_error,
    print_write_file_success,
};

pub fn print_file_error(error: Error, file_name: &String) {
    let action = "Could not Write File".red();
    let file = file_name.yellow();
    println!("  {} {} \n {}", action, file, error.to_string());
}

pub fn delete_dir_contents(dir_path: &String) {
    let dir_contents: ReadDir;
    match fs::read_dir(&dir_path) {
        Err(error) => {
            let error = error.to_string();
            println!("Could not find directory {dir_path} \n {error}");
            return;
        }
        Ok(contents) => dir_contents = contents,
    };

    for rez_f in dir_contents {
        let file;
        match rez_f {
            Err(error) => {
                println!("Invalid file ? \n {error}");
                continue;
            }
            Ok(f) => file = f,
        };

        let file_name: String = match file.file_name().to_str() {
            None => {
                println!("{}", "Invalid file Name");
                continue;
            }
            Some(f_name) => String::from(f_name),
        };

        let file_path = dir_path.to_owned() + &file_name;
        delete_file(&file_path);
    }
}

pub fn delete_file(file_path: &String) {
    match fs::remove_file(&file_path) {
        Err(error) => print_delete_file_error(error, file_path),
        Ok(_) => print_delete_file_success(file_path),
    }
}

pub fn write_file(file_path: &String, file_content: &String) {
    let mut file;

    match File::create(file_path) {
        Err(error) => {
            print_create_file_error(error, file_path);
            return;
        }
        Ok(f) => file = f,
    }

    match file.write_all(file_content.as_bytes()) {
        Err(error) => print_write_file_error(error, file_path),
        Ok(_) => print_write_file_success(file_path),
    }
}
