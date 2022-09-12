pub fn expressions() {
    let x = 5;

    x;

    x + 1;
    15;

    //? Blocks are expressions too

    let x = 3u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x * x * x;

        // X^3  + X^2  + X
        x_cubed + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
