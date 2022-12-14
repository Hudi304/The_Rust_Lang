use std::io::Error;

use colored::Colorize;

pub fn print_delete_file_error(error: Error, file_path: &String) {
    println!(
        "{} {} {}",
        "ERROR on file DELETE".red(),
        &file_path,
        error.to_string()
    );
}

pub fn print_delete_file_success(file_path: &String) {
    println!("{} {}", "Deleted File :".yellow(), &file_path);
}

pub fn print_create_file_error(error: Error, file_path: &String) {
    let message = "Error During File Creation :".yellow();
    println!("{message} {file_path} \n : {}", error.to_string());
}

pub fn print_write_file_error(error: Error, file_path: &String) {
    let message = "Error During File Creation :".yellow();
    println!("{message} {file_path} \n : {}", error.to_string());
}

pub fn print_write_file_success(file_path: &String) {
    let message = "Created File".cyan();
    let file_path = file_path.green();
    println!("{message} {file_path}");
}
