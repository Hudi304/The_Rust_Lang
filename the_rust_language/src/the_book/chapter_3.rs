pub fn chapter_3_common_programming_concepts() {
    // 3.2 data types
    //? let x = 5;
    //? println!("The value of x is: {x}");
    // ! x = 6;
    //* x is immutable by default */
    // ! println!("The value of x is: {x}");

    //? rust panics on integer overflow
    //? it is considered an error
    //? the default float is a double (this might be a problem for embedded)

    //* chr literals cha chr = 'a' */
    //* string literals string str = "abc" */
    //* chars in Rust are stored as 32 bit UTF-8 */

    // ! Compound types
    //* let tup: (i32, f64, u8) = (500, 6.4, 1); */

    // ! Rust destructuring
    //* let (x, y, z) = tup; */

    //* tuple accessing */
    //* let x: (i32, f64, u8) = (500, 6.4, 1); */
    //* let five_hundred = x.0; */
    //* let six_point_four = x.1; */

    // ! arrays
    //* let arr = [1, 2, 3, 4, 5]; */
    //* fixed size, for dynamic vectors use Vec */

    //? let a: [i32; 5] = [1,2,3,4,5]
    //? let a = [3; 5] // => a = [3,3,3,3,3]
    //? allocated on the stack

    //? Rust knows the size of arrays at compile time
    //? and iit will not allow out of bounds memory access
    //? on the stack, the program will panic

    // 3.3 functions
}
