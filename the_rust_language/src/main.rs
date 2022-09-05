//? CRATES

use rand::Rng;
use std::cmp::Ordering;
use std::io;

//? MODULES

mod rust_by_example;
mod the_book;

//? ALIASES

use crate::rust_by_example::chapter_1::rust_by_example as nice_try;

use the_book::chapter_2::chapter_2_guessing_game;

use the_book::chapter_3::chapter_3_common_programming_concepts;
use the_book::chapter_4::{
    chapter_4_dangling_reference, chapter_4_slice, chapter_4_what_is_ownership,
};

/// !rust analyzer crashes on startup sometimes

fn main() {
    chapter_2_guessing_game();
    chapter_3_common_programming_concepts();
    chapter_4_what_is_ownership();
    chapter_4_dangling_reference();
    chapter_4_slice();

    rust_by_example::chapter_1::rust_by_example();
    nice_try();
}
