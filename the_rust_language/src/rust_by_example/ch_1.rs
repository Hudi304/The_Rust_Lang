pub fn comments() {
    //? normal comments are ignored by the compiler

    //* "///" - generates docs for the following item
    //* "//!" - generates docs for the enclosing item

    println!(" '///' - generates docs for the following item");
    println!(" '//!' - generates docs for the enclosing item");
}

pub fn formatted_print() {
    // macros in std::fmt

    //* format() */
    //* print[ln]!() */
    //* eprint[ln]!() */
    println!();

    format!("{} {}! ", "Hello", "world");
    format!("{} {}! ", "Hello", "world");

    println!();

    print!("print!(\"{{}} days\", 31) = ");
    print!("\"{} days\"\n", 31);

    println!();

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //? BEAUTIFUL
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!();

    println!("Base 10 repr:               {}", 69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    println!();

    println!(
        "1 aligned to the right by 5 spaces = |{number:>5}|",
        number = 1
    );

    println!("zero padding = |{number:0>5}|", number = 1);

    println!("padding with a variable = |{number:0>5}|", number = 1);

    //? Rust even checks to make sure the correct number of arguments are used.
    // *  Compile time check for the number of parameters
    // ! println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"x
    println!("My name is {0}, {1} {0}", "Bond", "James");

    //? Only types that implement fmt::Display can be formatted with '{}'
    //? by default user-defined types do not implement it

    struct Structure(i32);

    // println!("This struct '{}' will not print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 6;
    println!("Numbers captured from the environment = |{number:>width$}|");

    println!("The fmt::Debug contains many traits that govern the display of text.");
    println!("The most important ones are :");
    println!(
        "   -{text}",
        text = "fmt:Debug,  {:?} formats text for debugging purposes"
    );
    println!(
        "   -{text}",
        text = "fmt:Display,  {} marker formats text in a more elegant, user friendly fashion"
    );

    println!("Implementing the fmt::Display trait automatically implements the ToString trait which allows us to convert the type to String.")
}

pub fn debug() {
    // ? the compiler knows how to derive primitives,
    // ? but any other must be implemented somehow

    // ? fmt::Debug makes this easy
    // ? all types can derive (automatically create) the fmt::Debug

    // ? fmt::Display on the other hand must be implemented by hand

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);
    // println!("can not be printed : {}", UnPrintable(3))

    #[derive(Debug)]
    struct Printable(i32);

    println!("Printable Struct with DEBUG: {:?}", Printable(123));

    example();

    pretty_printing();

    fn example() {
        #[derive(Debug)]
        struct Structure(i32);

        #[derive(Debug)]
        struct Deep(Structure);

        println!("{:?} months in a year.", 12);

        println!(
            "{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor = "actor's"
        );

        println!("Now {:?} will print!", Structure(3));

        println!("Now {:?} will print!", Deep(Structure(7)));
    }

    fn pretty_printing() {
        #[derive(Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8,
        }

        let name = "Peter";

        let age = 27;

        let peter = Person { name, age };

        println!("{:#?}", peter);

        println!("Pretty awesome");
    }
}
