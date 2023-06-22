fn associated_types() {
    pub trait Iterator {
        type Item; // this is an associated type

        fn next(&mut self) -> Option<Self::Item>;
    }
    // Item is a placeholder

    // they do look a lot like generics

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            // --snip--
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // the difference is that we don't need type annotations anymore
    // also you can not implement a trait on a type multiple times
}

use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generic_type_parameters_and_operator_overloading() {
    // you can specify a default type for a generic
    // this means that you won't need to annotate that type

    // Rust does not allow you to create your own operators or overload arbitrary ones
    // but you can overload the operators in std::ops

    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    trait Add<Rhs = Self> {
        // default type parameter
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn calling_methods_with_the_same_name() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    // this is a compiler error because Animal does not have an implementation for baby name
    // just a definition
    // println!("A baby dog is called a {}", Animal::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // this is called fully qualified syntax
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);
}

use std::fmt;

fn super_traits() {
    // let's say you want to implement a trait that can only be implemented
    // if another trait is implemented

    trait OutlinePrint: fmt::Display {
        // we want this to only work on types that implement Display
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    // this results ina compiler error because Point does not implement Display
    //? impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

fn implement_external_traits_on_external_types() {
    // ! The Orphan Rule
    //? you can only implement a trait on a type
    //? if either the trait or the type is local

    // There is a wau to get around that with the NewType Pattern
    // a.k.a. creating a new type in a tuple struct

    // let's implement Display on Vec<T>

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

pub fn main() {
    // Advanced Traits

    associated_types();
    default_generic_type_parameters_and_operator_overloading();
    calling_methods_with_the_same_name();
    super_traits();
    implement_external_traits_on_external_types();
}
