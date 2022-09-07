//? CRATES

// use std::cmp::Ordering;
// use std::io;
// use rand::Rng;

//? MODULES

mod rust_by_example;
mod tests;
mod the_book;

//? ALIASES
#[allow(dead_code, unused, unused_labels)]
use rust_by_example::*;
// use the_book::chapter_2::chapter_2_guessing_game;
// use the_book::chapter_3::chapter_3_common_programming_concepts;
// use the_book::chapter_4::{
//     chapter_4_dangling_reference, chapter_4_slice, chapter_4_what_is_ownership,
// };

/// !rust analyzer crashes on startup sometimes

fn main() {
    println!("-------------------------");
    println!("Result : ");

    //? The Rust Programming Language
    // chapter_2_guessing_game();
    // chapter_3_common_programming_concepts();
    // chapter_4_what_is_ownership();
    // chapter_4_dangling_reference();
    // chapter_4_slice();

    //? Rust by example
    // ch_1::comments();
    // ch_1::formatted_print();
    // ch_1::debug();
    // ch_1::display();
    // ch_1::testcase_list();
    // ch_1::formatting();

    // ch_2::literals_and_operators();
    // ch_2::tuples();
    // ch_2::arrays_and_slices();

    // ch_3::structures();
    // ch_3::enums();
    ch_3::enum_linked_list();


    // tests::tests::inquire();

    //? The Async book
}
