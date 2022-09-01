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

fn chapter_4_what_is_ownership() {
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
    println!("s2={s2}");

    // ! 🔥 this is an error because now s2 is the owner of the String,
    // ! and there can only be one owner at a time

    //? let s2 = s1; does not make a copy of s1

    //* anatomy of a String 🍏 */
    //* 🔴 pointer to the memory that holds the contents of the string */
    //* 🔵 a length   👉 how much memory in bytes the contents of the String are currently using*/
    //* 🟡 a capacity 👉 the total amount of memory in bytes that the String has received from the allocator
    //* [these are apparently not the same thing] */
    //? the String is stored on the stack
    //? while the data of the String itself is held on the heap

    // ! it would be very expensive to copy the data for every assignment
    // ! so rust only copies the "pointer"

    // ? if s1 and s2 would point to the same location in memory, when they go
    // ? out of scope they will call drop on the same location of memory, and this is a bug

    // ? this is not even a shallow copy because RUST invalidates the first copy
    // ? thus this is called a move

    // ? Rust never deep copies data (maybe just primitives)
    // ? because their size is known at compile time and they are
    // ? stored on the stack and operations on the stack are
    // ? faster than operations on the heap
    // ? thus there is no reason not to make a "deep copy" of a primitive

    // *  RUST has a special annotation called the Copy trait, that can be
    // *  placed on types that are stored on the stack.
    // *  If a type implements the Copy trait variables that use it do not move,
    // *  but rather are trivially copied, making them still valid after assignment
    // *  to another variable

    // *  Also Rust will not let you annotate a type with Copy if the type or
    // *  any of it's parts has implemented the drop trait, so the type itself
    // *  needs to hae a known size at compile time

    // ---------------------------------------------------

    // *  in order to make a deep copy of the string data on the heap
    // *  RUST has a method called .clone()

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1},  s2 = {s2}");

    // ------------------------------------------------------------

    // ?  passing a  variable to a function moves th variable just as assignment does

    fn ownership_demo() {
        fn takes_ownership(some_string: String) {
            // some_string comes into scope
            println!("{}", some_string);
        }
        // Here, some_string goes out of scope and `drop` is called. The backing
        // memory is freed.

        fn makes_copy(some_integer: i32) {
            // some_integer comes into scope
            println!("{}", some_integer);
        }

        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    }
    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    // Here, some_integer goes out of scope. Nothing special happens.

    println!("\nOwnership demo");

    ownership_demo();

    // *  returning a value can also transfer ownership

    fn fn_ownership() {
        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }

        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    fn_ownership();
}

fn chapter_4_references_and_borrowing() {
    //? the difference between a pointer and a reference is that
    //? a reference is guaranteed to point to a valid value of a
    //? particular type for the life of the reference

    fn compute_len(s: &String) -> usize {
        s.len()
    }

    // * the dereference operator '*' exists in RUST
    // * are values passed to a function dereferenced by default?

    let s1 = String::from("hello");
    let len = compute_len(&s1);
    print!("len = {len}");

    //? references are of course immutable by default

    fn change(some_string: &String) {
        // some_string.push_str(", world");
        //!  cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    }
    let s = String::from("hello");
    change(&s);

    // ?  the problem above can be solved by making the string reference mutable

    // ! in order to avoid a data race :
    //* */ two or more pointers access the same data at the same time
    //* */ at least one of the pointers is being used to write to the data
    //* */ there's no mechanism in use to synchronize access to the data

    //? the ability of the compiler to tell that a reference is no longer being used at a
    //? point before the end of the scope is called Non-Lexical Lifetimes (NLL)
}

fn chapter_4_dangling_reference() {
    //? dangling reference = a pointer that references a location in
    //? memory that may have been given to someone else, by freeing some
    //? memory but preserving a pointer to it

    // *  the RUST compiler guarantees that references will never be dangling

    // fn dangle() -> &String {
    //! missing lifetime specifier
    //     let s = String::from("hello");
    //     &s
    // }
    // let reference_to_nothing = dangle();

    //* */ there is a feature in RUST that lets you manipulate this behavior, lifetimes

    //Todo ---------- Rules for References ------------------
    //* */ at any given time you can have :
    //*        */ on mutable reference
    //*        */ any number of immutable references
    //* */ references must always be valid

    
}
fn main() {
    // chapter_2_guessing_game();
    // chapter_3_common_programming_concepts();
    chapter_4_what_is_ownership();
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
