fn type_synonyms() {
    type Kilometers = i32;

    // this is just a type alias

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // if we mix kilometers and i32 the compiler will not
    // give us an error

    // their main purpose is to reduce repetition
    {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            // --snip--
            todo!()
        }

        fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
            // --snip--
            todo!()
        }
    }
    // Thunk is a word for code to be evaluated later
    // so it's an appropriated name for a closure that gets stored
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
        todo!()
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        todo!()
    }

    use std::fmt;
    use std::io::Error;
    {
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
            fn flush(&mut self) -> Result<(), Error>;

            fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
        }
    }

    {
        type Result<T> = std::result::Result<T, std::io::Error>;
        // this looks much better
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }
}

fn the_never_type() {
    // a.k.a. the empty type
    //  it has no values

    // it stands in the place of the return type of a
    // function that will never return
    fn bar() -> ! {
        // --snip--
        todo!()
    }
    // this reads as the function bar returns never
    // they are called diverging functions
    // we can not create the never type so the
    // function can never return

    // what is the use of the never type
    // loop {
    //     break;
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    // }
    // this works because the continue keyword has type never
    // so the ! type can not be coerced to any other type

    // this does not return because the match arms
    // return different types
    // let guess = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // };

    // this is also used in Option<T>
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }
}

fn dynamically_sized_types_and_the_sized_trait() {
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // on of these needs 12 bytes
    // the other one needs 15 bytes

    // thi is why it is not possible to create variable holding
    // on dynamically sized types

    // this will work
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    // because now the s1 and s2 are pointers we can now what their size is
    // twice the size od a usize to be exact
    // &T is a single value that stores the memory address of T
    // &str is composed of 2 values the pointer to he value and the length

    //? the golden rule of DSTs is that we must put the value of the the DSF
    //? behind a pointer of some kind

    use std::rc::Rc;

    let s3: Box<str> = "Hello there!";
    // let s4: Rc<str> = "How's it going?";

    // this also happens with generics

    fn generic<T>(t: T) {
        // --snip--
        todo!()
    }

    fn generic<T: Sized>(t: T) {
        // --snip--
        todo!()
    }

    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }

    // this trait bound reads as T may or may not be sized
    // the type of t changed from T -> &T 
    // because if the type is not sized that means we need to put it behind 
    // some sort of pointer
    
}

pub fn main() {
    // Advanced types

    // NewTypes can be used to hide implementation
    // a.k.a. encapsulation

    type_synonyms();

    the_never_type();
}
