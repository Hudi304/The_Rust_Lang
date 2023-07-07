// use std::fs::File;
// use std::io::Error;
// use std::io::Write;

// pub struct FileSchema {
//     path: String,
//     name: String,
//     lines: Vec<String>,
// }

// pub fn write_file(file: FileSchema) {
//     let file_path = file.path + file.name.as_str() + ".enum.ts";

//     let content = file.lines.join("\n");

//     let mut file: File;

//     match File::create(file_path.clone()) {
//         Err(error) => {
//             // print_create_file_error(error, file_path);
//             println!("Error writing file {error}");
//             return;
//         }
//         Ok(f) => file = f,
//     }

//     match file.write_all(content.as_bytes()) {
//         Err(error) => print_write_file_error(error, &file_path),
//         Ok(_) => print_write_file_success(&file_path),
//     }
// }

// pub fn print_write_file_error(error: Error, file_path: &String) {
//     let message = "Error During File Creation :";
//     println!("{message} {file_path} \n : {}", error.to_string());
// }

// pub fn print_write_file_success(file_path: &String) {
//     let message = "Created File";
//     let file_path = file_path;
//     println!("{message} {file_path}");
// }
