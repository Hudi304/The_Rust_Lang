use colored::Colorize;
use list::List;
use serde::Deserialize;
use std::io::{BufRead, Read, Write};
use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

use toml::*;
#[derive(Debug, Deserialize)]

pub struct Filters {
    termination: String,
}

#[derive(Deserialize, Clone)]
pub struct Pair {
    findThis: String,
    replace: String,
}

#[derive(Deserialize)]
pub struct Replace {
    arr: Vec<Pair>,
}

#[derive(Deserialize)]
pub struct ReplaceAllSettings {
    filters: Filters,
    replace: Replace,
}

pub fn has_termination(file_path: &Path, termination: &str) -> bool {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_termination = file_name.split(".").last().unwrap();

    if file_termination.to_owned().eq(termination) {
        return true;
    }
    return false;
}

pub fn read_settings() -> ReplaceAllSettings {
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

    println!("{}", config.filters.termination);

    return config;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn replace_in_all_files(directory_path: String) {
    let settings = read_settings();

    let mut arr_list: List<Pair> = List::new();

    let length = settings.replace.arr.len();

    let arr = settings.replace.arr;

    // for pair in settings.replace.arr.into_iter() {
    //     arr_list.push(pair)
    // }

    let arr_list = arr_list;

    for entry in WalkDir::new(directory_path) {
        let unwrapped_entry;
        match entry {
            Err(error) => {
                println!("{}", error.to_string());
                continue;
            }
            Ok(entry) => unwrapped_entry = entry,
        }

        let file_path = unwrapped_entry.path();
        let display = file_path.display();

        let mut file = match File::open(&file_path) {
            Err(why) => {
                println!("{} {}: {}", "couldn't open".red(), display, why);
                continue;
            }
            Ok(file) => file,
        };

        let metadata = file.metadata().unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // println!("{}", file_name.green());

        if has_termination(file_path, "scss") {
            println!("{}", file_name.green());

            let mut src = File::open(&file_path).unwrap();
            let mut data = String::new();
            src.read_to_string(&mut data).unwrap();
            drop(src); // Close the file early

            // let mut arr_clone: Vec<Pair> = vec![];

            let mut new_data: String = data;
            for i in 0..length {
                let fnd = arr[i].findThis.clone();

                let with = arr[i].replace.clone();

                new_data = new_data.replace(&fnd, &with);
            }

            // for rep in arr_list {}

            // let new_data = data.replace(&*"something", &*"somethingElse");

            let mut dst = File::create(&file_path).unwrap();
            dst.write(new_data.as_bytes()).unwrap();

            println!("done");
        }

        let file_type = metadata.file_type();

        if (file_type.is_file()) {}
    }
}
