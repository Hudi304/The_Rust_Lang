use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// std::error::Error is a trait object
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // this is the idiomatic way to say that
    // we call run just for it's side effects
    Ok(())
}

//? basically a' stands for :
// the data returned by the search() function will live
// as long as the contents argument
// the data referenced by a string slice needs to be valid
// in order for the reference to be valid
//? we want to return a vector of references to the contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //? what the function needs to do :
    // Iterate through each line of the contents.
    // Check whether the line contains our query string.
    // If it does, add it to the list of values we’re returning.
    // If it doesn’t, do nothing.
    // Return the list of results that match.
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
