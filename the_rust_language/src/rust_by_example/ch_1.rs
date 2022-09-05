pub fn comments() {
    //? normal comments are ignored by the compiler

    //* "///" - generates docs for the following item
    //* "//!" - generates docs for the enclosing item

    println!(" '///' - generates docs for the following item");
    println!(" '//!' - generates docs for the enclosing item");
}

pub fn formatted_print() {
    // macros in std::fmt

    //* format() */
    //* print[ln]!() */
    //* eprint[ln]!() */
    format!("{} {}! ", "Hello", "world");
    format!("{} {}! ", "Hello", "world");


  

}
