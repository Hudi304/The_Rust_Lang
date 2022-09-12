pub fn functions() {
    //  Unlike C/C++, there's no restriction on the order of function definitions
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);

    // Function that returns a boolean value
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // Corner case, early return
        if rhs == 0 {
            return false;
        }

        // This is an expression, the `return` keyword is not necessary here
        lhs % rhs == 0
    }
    // Functions that "don't" return a value, actually return the unit type `()`
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // When a function returns `()`, the return type can be omitted from the
    // signature
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }
}

pub fn associated_fn_and_methods() {
    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all `Point` associated functions & methods go in here
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Another associated function, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            // `abs` is a `f64` method that returns the absolute value of the
            // caller
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` owns resources: two heap allocated integers
    #[derive(Debug)]
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // This method "consumes" the resources of the caller object
        // `self` desugars to `self: Self`
        fn destroy(self) {
            // Destructure `self`
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // `first` and `second` go out of scope and get freed
        }
    }

    {
        let rectangle = Rectangle {
            // Associated functions are called using double colons
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };

        // Methods are called using the dot operator
        // Note that the first argument `&self` is implicitly passed, i.e.
        // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());

        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };

        // Error! `rectangle` is immutable, but this method requires a mutable
        // object
        // rectangle.translate(1.0, 0.0);
        // TODO ^ Try uncommenting this line

        // Okay! Mutable objects can call mutable methods
        square.translate(1.0, 1.0);

        let pair = Pair(Box::new(1), Box::new(2));

        println!("{:?}", pair);

        pair.destroy();
        // println!("{:?}", pair);

        // Error! Previous `destroy` call "consumed" `pair`
        // pair.destroy();
        // TODO ^ Try uncommenting this line
    }
}
pub fn closures() {
    //?? Closures are functions that can capture the enclosing environment.
    //?? For example, a closure that captures the x variable:

    //* |val| val + x

    //? The syntax and capabilities of closures make them very convenient
    //? for on the fly usage. Calling a closure is exactly like calling a function.
    //??  However, both input and return types can be inferred and input variable names
    //?? must be specified.

    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

pub fn capturing() {
    // Closures can capture variables:

    // by reference: &T
    // by mutable reference: &mut T
    // by value: T

    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}

pub fn input_parameters() {
    // A function which takes a closure as an argument and calls it.
    // <F> denotes that F is a "Generic type parameter"
    fn apply<F>(f: F)
    where
        // The closure takes no input and returns nothing.
        F: FnOnce(),
    {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
        // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

pub fn type_anonymity() {
    // `F` must be generic.
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    {
        // `F` must implement `Fn` for a closure which takes no
        // inputs and returns nothing - exactly what is required
        // for `print`.
        fn apply<F>(f: F)
        where
            F: Fn(),
        {
            f();
        }

        let x = 7;

        // Capture `x` into an anonymous type and implement
        // `Fn` for it. Store it in `print`.
        let print = || println!("{}", x);

        apply(print);
    }
}

pub fn input_functions() {
    // Define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a wrapper function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    //? As an additional note,
    //? the Fn, FnMut, and FnOnce traits dictate
    //? how a closure captures variables from the enclosing scope.
}
