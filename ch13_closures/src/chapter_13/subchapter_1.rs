#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

use std::thread;
use std::time::Duration;

// Closures can capture values from the scope in which they are defined
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn t_shirt_giveaway() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn this_will_fail() {
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // let n = example_closure(5)
    // does not even need to be called to fail
}

fn borrow_immutably() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn borrow_mutably() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // borrows mutable value, this can be done only once
    let mut borrows_mutably = || list.push(7);

    // println!("After calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn move_keyword() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)) //;
        .join()
        .unwrap();

    // the main thread might finish before the rest of the main
    // if the main thread finished first it will drop list
    // therefor the reference to list would be invalid

    // by moving the list to the thread the reference
    // remains valid even if the main thread finished first

    //  thread::sleep(Duration::from_secs(1));
}
// a closure body can do any of the following :
// - move a captured value
// - mutate the captured value
// - neither move or mutate the captured value
// - capture nothing from the environment to begin with

// ? they can implement at least one of the next traits
// FnOnce - can only be called once
//        - a closure that moves captured value out of it's value will only implement FnOnce
// FnMut - can be called multiple times
//       - closures that don't move but might mutate captured values
// Fn - these can be called multiple times concurrently
//    - they don't move or mutate captured values (or don't capture anything at all)

// todo this applies to functions as well */
// unwrap_or_else(Vec::new)

// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1; // these need to be called once so that the compiler can give them a type
    let add_one_v4 = |x| x + 1; // these need to be called once so that the compiler can give them a type

    add_one_v1(1);
    add_one_v2(1);
    add_one_v3(1);
    add_one_v4(1);

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    this_will_fail();

    t_shirt_giveaway();
    // expensive_closure(123);

    borrow_immutably();
    println!("----------------------");
    borrow_mutably();
    println!("----------------------");
    move_keyword();

    println!("----------------------");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    println!("----------------------");

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     // this closure only implements FnOnce because it moves value
    //     sort_operations.push(value);
    //     r.width
    // });

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        // so... this implements FnMut
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
