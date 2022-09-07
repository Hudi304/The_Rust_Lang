#[allow(dead_code, unused)]
pub fn comments() {
    //? normal comments are ignored by the compiler

    //* "///" - generates docs for the following item
    //* "//!" - generates docs for the enclosing item

    println!(" '///' - generates docs for the following item");
    println!(" '//!' - generates docs for the enclosing item");
}
#[allow(dead_code, unused)]
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
#[allow(dead_code, unused)]

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

use std::fmt::{self, Display};
#[allow(dead_code, unused)]

pub fn display() {
    println!("Display implementation example");

    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "{}", self.0);
        }
    }

    println!("Display print : {}", Structure(3));

    println!();
    println!("There is no implementation for the Vec<T> or for any other generic container");
    println!("fmt::Display must the be used for the generic cases");
    println!();

    println!(
        "This is not a problem because Display can be implemented for any other non generic types"
    );

    println!("MinMax------------------------");

    #[derive(Debug)]
    struct MinMax(i64, i64); //? struct made out of a tuple

    //? what the hell is <'_>
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return write!(f, "({},{})", self.0, self.1);
        }
    }

    println!("MinMax(3,4) = {}", MinMax(3, 4));

    println!("Point2D------------------------");

    #[derive(Debug)]
    struct Point2D {
        //? normal struct
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    println!("Point2S(3,4) = {}", Point2D { x: 3.0, y: 4.0 });

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display MinMax: {}", minmax);
    println!("Debug MinMax: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // ? Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // ? requires `fmt::Binary` to be implemented. This will not work.
    // ? println!("What does Point2D look like in binary: {:b}?", point);

    // After checking the output of the above example, use the Point2D struct
    // as a guide to add a Complex struct to the example. When printed in the same way,
    // the output should be:
    // Display: 3.3 + 7.2i
    // Debug: Complex { real: 3.3, imag: 7.2 }

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let cpl = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Debug : {:?}", cpl);

    println!("Display : {}", cpl)
}
#[allow(dead_code, unused)]

pub fn testcase_list() {
    struct List(Vec<i32>);

    // impl fmt::Display for List {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let vec = &self.0;
    //         write!(f, "[")?;
    //         for (index, v) in vec.iter().enumerate() {
    //             if index != 0 {
    //                 write!(f, ", ")?;
    //             }
    //             write!(f, "{}", v)?;
    //         }
    //         write!(f, "]")
    //     }
    // }

    let v = List(vec![1, 2, 3]);

    // ? Try changing the program so that the index of each element
    // ? in the vector is also printed. The new output should look like this:

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (index, v) in vec.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", index, v)?;
            }
            write!(f, "]")
        }
    }

    println!("{}", v);
}

#[allow(dead_code, unused)]
pub fn formatting() {
    let foo: i64 = 432132132121;

    let fmt_norm = format!("{}", foo);
    let fmt_hex = format!("0x{:X}", foo);
    let fmt_oct = format!("0o{:o}", foo);

    println!("{}", fmt_norm);
    println!("{}", fmt_hex);
    println!("{}", fmt_oct);

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // ?  write works exactly like format but it writes the result into a buffer, first arg

            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }

    let colors = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    for color in colors.iter() {
        println!("{:?}", *color);
    }

    // ? Add an implementation of the fmt::Display trait
    // ? for the Color struct above so that the output displays as:

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let r = format!("{:0>2}", self.red);

            fn form(str: &u8) -> String {
                format!("{:X}", str)
            }

            write!(
                f,
                "RGB ({r}, {g}, {b}) ox{r:0>2}{g:0>2}{b:0>2}",
                r = form(&self.red),
                g = form(&self.green),
                b = form(&self.blue)
            )
        }
    }

    for color in colors.iter() {
        println!("{}", *color);
    }
}
