use std::fmt::{self};

#[allow(dead_code, unused)]
pub fn literals_and_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    // println!("1 - 2 = {}", 1u32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

#[allow(dead_code, unused)]
pub fn tuples() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;

        (boolean, integer)
    }

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    println!("{}", matrix);
    println!("------------");

    fn transpose(mat: Matrix) -> Matrix {
        Matrix(mat.0, mat.2, mat.1, mat.3)
    }

    println!("{}", transpose(matrix));
}

use std::mem;
#[allow(dead_code, unused)]
pub fn arrays_and_slices() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element fo the slice: {}", slice[0]);
        println!("the length of the slice is: {}", slice.len())
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500]; //* 2kb

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("array occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow the whole array as a slice");

    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];
    // assert_eq!(&empty_array, &[]);
    // assert_eq!(&empty_array, &[][..]); // same but more verbose

    for i in 0..xs.len() + 1 {
        // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}
