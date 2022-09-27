use std::fmt::Debug;
use std::fmt::Display;

pub fn reducing_code_duplication() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    {
        fn largest(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }
}

pub fn generics() {
    {
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    {
        // ?  type T is restricted to types that have the trait PartialOrd
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

    {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
    }

    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // ! no runtime cost for generics
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub fn traits_defining_shared_behavior() {
    // they are similar with interfaces
    //? but not the same

    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        // println!("1 new tweet: {}", tweet.summarize());
    }

    {
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        impl Summary for NewsArticle {}

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        println!("1 new tweet: {}", tweet.summarize());
    }
}

pub fn traits_as_parameters() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_3(item1: &impl Summary, item2: &impl Summary) {}
    //? this makes sense if we want item 1 and item 2 to have different
    //? types as long as they implement Summary

    pub fn notify_4<T: Summary>(item1: &T, item2: &T) {}
    //? this forces item1 and item2 to have the same type
}

pub fn traits_plus_operator() {
    pub fn notify(item: &(impl Summary + Display)) {}

    pub fn notify_2<T: Summary + Display>(item: &T) {}
}

pub fn where_clause() {
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        return 2;
    }

    fn some_function_2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        return 2;
    }
}

pub fn returning_types_that_implement_traits() {
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // fn returns_summarizable_2(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
}

pub fn conditional_method_implementation() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    //? only pairs of types that implement Display and PartialOrd
    //? have the cmp_display() method
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

pub fn lifetimes() {
    //lifetimes ensure that the references are valid as long as we need them too

    // every reference has a lifetime that is the scop for which that reference is valid

    // most of the time they are implicit and infered

    //? PREVENTING DANGLING REFERENCES

    let r;

    {
        let x = 5;
        r = 5;
        // r = &x;
    }
    //* `x` does not live long enough BEAUTIFUL

    println!("r: {}", r);

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    fn longest_with_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest_with_lifetimes(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    // let result;
    {
        let string2 = String::from("xyz");
        // result = longest_with_lifetimes(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
