pub fn panic_handling() {
    //? there are 2 ways of panicking

    //? with unwinding, the program cleans the stack of
    //? all the function calls and params it has used
    //? but this is a lot of work

    //? you can choose to let the OS clean the stack afterwords

    // panic!("crash and burn");

    // backtracing // StackTrace?

    fn panic_fn() {
        let v = vec![1, 2, 3];
        v[99];
    }

    panic_fn();
}

use std::fs::{read, File};
use std::io::ErrorKind;
use std::io::{self, Read};
pub fn recover_from_errors() {
    {
        let greeting_file_result = File::open("./hello.txt");

        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => panic!("Problem opening the file: {:?}", error),
        // };

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    // println!(" greeting_file_result = {:#?}", greeting_file_result)

    {
        let greeting_file = File::open("hello.txt").unwrap();
        // expect lets us use an error message

        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }

    {
        //? a little confused as of why this changed since the book was released

        // let username_file_result = File::open("hello.txt");

        // let mut username_file = match username_file_result {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // let mut username = String::new();

        // match username_file.read_to_string(&mut username) {
        //     Ok(_) => Ok(username),
        //     Err(e) => Err(e),
        // }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    // if the result is an Ok() then it will be returned
    // if the result is an Err() then it will be returned
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn question_mark_operator() {
    let user_name = read_username_from_file().unwrap();

    let user_name_3 = read_username_from_file_3();
    //? still the string is empty
    println!("{:?}", user_name_3)
}

use std::net::IpAddr;

pub fn to_panic_or_not_to_panic() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
