use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// !rust analyzer crashes on startup sometimes

fn chapter_3_common_programming_concepts() {
    // 3.2 data types
    //? let x = 5;
    //? println!("The value of x is: {x}");
    // ! x = 6;
    //* x is immutable by default */
    // ! println!("The value of x is: {x}");

    //? rust panics on integer overflow
    //? it is considered an error
    //? the default float is a double (this might be a problem for embedded)

    //* chr literals cha chr = 'a' */
    //* string literals string str = "abc" */
    //* chars in Rust are stored as 32 bit UTF-8 */

    // ! Compound types
    //* let tup: (i32, f64, u8) = (500, 6.4, 1); */

    // ! Rust destructuring
    //* let (x, y, z) = tup; */

    //* tuple accessing */
    //* let x: (i32, f64, u8) = (500, 6.4, 1); */
    //* let five_hundred = x.0; */
    //* let six_point_four = x.1; */

    // ! arrays
    //* let arr = [1, 2, 3, 4, 5]; */
    //* fixed size, for dynamic vectors use Vec */

    //? let a: [i32; 5] = [1,2,3,4,5]
    //? let a = [3; 5] // => a = [3,3,3,3,3]
    //? allocated on the stack

    //? Rust knows the size of arrays at compile time
    //? and iit will not allow out of bounds memory access
    //? on the stack, the program will panic

    // 3.3 functions
}

fn chapter_4_ownership() {
    //? HEAP the heap is less organised
    //? - you request a certain amount of space from the heap
    //? - The memory allocator finds an empty spot in the heap that is big enough,
    //? - marks it as being in use, and returns a pointer, which is the address of that location
    //? pushing to the stack is faster than allocating on the heap

    //! The 3 RULES of ownership
    //* Each value in RUST has an owner */
    //* There can only be one owner at a time */
    //* When the Owner goes out of scope, the value will be dropped */
    //? C++ Resource Acquisition Is Initialization (RAII)

    let x = 5;
    let y = x;

    println!("x={x}, y={y}");
    //* x=5, y=5 , because integers have a known size at compile time
    //* and they are pushed onto the stack */
    let s1 = String::from("hello");
    let s2 = s1;

    //? "hello" is a string literal it's value will be hardcoded into the binary
    //? the String type wraps it and manages the memory access in order to make it
    //? dynamic

    // print!("s1={s1}");
    print!("s2={s2}");

    // ! 🔥 this is an error because now s2 is the owner of the String,
    // ! and there can only be one owner at a time
}

fn main() {
    chapter_4_ownership()
    // chapter_2_guessing_game();
    // chapter_3_common_programming_concepts();
}

fn chapter_2_guessing_game() {
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
