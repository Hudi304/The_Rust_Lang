use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // responsibility of main.rs :
    //      Calling the command line parsing logic with the argument values
    //      Setting up any other configuration
    //      Calling a run function in lib.rs
    //      Handling the error if run returns an error

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

