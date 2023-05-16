use minigrep::Config;
use std::env;
use std::process;

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

    // Write a test that fails and run it to make sure it fails for the reason you expect.
    // Write or modify just enough code to make the new test pass.
    // Refactor the code you just added or changed and make sure the tests continue to pass.
    // Repeat from step 1!
}
