//? CRATES

// use std::cmp::Ordering;
// use std::io;
// use rand::Rng;

//? MODULES

mod rust_by_example;
mod tests;
mod the_book;

//? ALIASES

use rust_by_example::*;
use the_book::chapter_2::chapter_2_guessing_game;
use the_book::chapter_3::chapter_3_common_programming_concepts;
use the_book::chapter_4::{
    chapter_4_dangling_reference, chapter_4_slice, chapter_4_what_is_ownership,
};

/// !rust analyzer crashes on startup sometimes

fn main() {
    // chapter_2_guessing_game();
    // chapter_3_common_programming_concepts();
    // chapter_4_what_is_ownership();
    // chapter_4_dangling_reference();
    // chapter_4_slice();

    // ch_1::comments();
    // ch_1::formatted_print();

    tests::tests::inquire();
}
