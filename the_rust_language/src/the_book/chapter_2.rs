use std::{cmp::Ordering, io};

use rand::Rng;

#[allow(dead_code)]
pub fn chapter_2_guessing_game() {
    //! println() is not a function by default, use the macro

    println!("Guess the number!");
    println!("Please input you guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);  

    println!("secret number: {secret_number}");

    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //* parse is interesting
        //* it parses a variable to another type  */
        //* the type is chosen by the type annotation of tha variable */
        // let guess: u16 = guess.trim().parse().expect("please type a number!");
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //? "crab pincer operator"
        println!("You guessed: {guess}");
        //? there's more than one type of crate
        //? binary crate
        //? library crate
        // ! .lock-files
        //? it will only update if there are no breaking changes
        //? if the dependencies has breaking changes it will not update it
        //* learn more about dependency management */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
