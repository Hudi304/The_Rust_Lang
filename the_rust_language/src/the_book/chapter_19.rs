use super::ch_19_x::ch_19_1;
use super::ch_19_x::ch_19_2;

pub fn main() {
    //? The unsafe superpowers
    // Dereference a raw pointer
    // Call an unsafe function or method
    // Access or modify a mutable static variable
    // Implement an unsafe trait
    // Access fields of unions

    // ! the borrow checker is still there!
    // you only get these 5 new superpowers

    // the unsafe block does not mean that the code is dangerous
    // it simply means that the compiler can not guarantee that it is
    // so the programmer must do the checking by hand

    ch_19_1::ch_19_1();
    ch_19_2::ch_19_2();
}
