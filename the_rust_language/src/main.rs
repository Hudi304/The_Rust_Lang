//? CRATES
// use std::cmp::Ordering;
// use std::io;
// use rand::Rng;
//? MODULES

mod rust_by_example;
mod swagger_generator;
mod tests;
mod the_book;
mod tools;


//? ALIASES
#[allow(dead_code, unused, unused_labels)]
use rust_by_example::*;
use swagger_generator::*;

// use crate::the_book::{chapter_11, chapter_13};

// use crate::tools::tools::replace_in_all_files;

// use crate::the_book::{chapter_10, chapter_5, chapter_6, chapter_8, chapter_9};

// use the_book::chapter_2::chapter_2_guessing_game;
// use the_book::chapter_3::chapter_3_common_programming_concepts;
// use the_book::chapter_4::{
//     chapter_4_dangling_reference, chapter_4_slice, chapter_4_what_is_ownership,
// };

// !rust analyzer crashes on startup sometimes
//todo formatted print
//todo println!("rct = {:#?}", rect1);

//TODO split into separate files
fn main() {
    println!("-------------------------");
    println!("Result : ");

    //? The Rust Programming Language
    {
        // chapter_2_guessing_game();
        // chapter_3_common_programming_concepts();
        // chapter_4_what_is_ownership();
        // chapter_4_dangling_reference();
        // chapter_4_slice();
        // chapter_5::structs();
        // chapter_5::program_with_structs();
        // chapter_5::methods();
        // chapter_6::enums_and_pattern_matching();
        // chapter_6::option_enum();
        // chapter_6::match_control_flow();
        // chapter_6::if_let();

        //TODO read chapter_7

        // chapter_8::common_collections();
        // chapter_8::strings();
        // chapter_8::hash_maps();

        // chapter_9::panic_handling();
        // chapter_9::recover_from_errors();
        // chapter_9::question_mark_operator();
        // chapter_9::to_panic_or_not_to_panic();

        // chapter_10::reducing_code_duplication();
        // chapter_10::generics();
        // chapter_10::traits_defining_shared_behavior();
        // chapter_10::traits_as_parameters();
        // chapter_10::where_clause();
        // chapter_10::returning_types_that_implement_traits();
        // chapter_10::conditional_method_implementation();

        // chapter_10::conditional_method_implementation();
        // chapter_10::lifetimes

        // chapter_11::tests();
        // chapter_12::

        // chapter_13::ch13();
    }

    //? Rust by example
    {
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
        // ch_3::enum_linked_list();

        // ch_5::casting();
        // ch_5::literals();
        // ch_5::inference();
        // ch_5::aliasing();

        // ch_6::from_into();
        // ch_6::try_from_try_into();
        // ch_6::string_parsing();

        // ch_7::expressions();

        // ch_8::flow_control();
        // ch_8::loop_keyword();
        // ch_8::nesting_and_labels();
        // ch_8::returning_from_loops();
        // ch_8::while_keyword();
        // ch_8::for_and_range();
        // ch_8::match_keyword();
        // ch_8::destructuring_tuples();
        // ch_8::destructuring_array_slices();
        // ch_8::destructuring_enums();
        // ch_8::destructuring_pointers_and_references();
        // ch_8::destructuring_structs();

        // ch_8_5::guards();
        // ch_8_5::bindings();
        // ch_8_6::if_let();
        // ch_8_7::while_let();

        {
            // ch_9::functions()
            // ch_9::associated_fn_and_methods();
            // ch_9::closures();
            // ch_9::capturing();
            // ch_9::input_parameters();
            // ch_9::type_anonymity();
            // ch_9::input_functions();
            // ch_9::output_parameters();
        }
    }

    // swagger_gen::song_example();
    // swagger_gen::get_data();
    // replace_in_all_files("./src".to_owned());
    // tests::tests::inquire();
    //? The Async book

    the_book::chapter_14::add_one(1);

    println!("FINISHED SUCCESSFULLY ");
}
