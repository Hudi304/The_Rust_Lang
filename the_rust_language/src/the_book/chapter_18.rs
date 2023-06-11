fn ch_18_1() {
    fn if_let() {
        //? Match arms
        //   match VALUE {
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        // }

        // fn inc() {
        //     match x {
        //         None => None,
        //         Some(i) => Some(i + 1),
        //     }
        // }

        // match expressions need to be exhaustive

        //? The "if let" statement

        // Rust does not require that the conditions in a series of :
        // if let, else if, else if let
        // relate to each other

        fn color() {
            let favorite_color: Option<&str> = None;
            let is_tuesday = false;
            let age: Result<u8, _> = "34".parse();

            // the big disadvantage of 'if let' statements is that the compiler
            // does not check for exhaustiveness

            if let Some(color) = favorite_color {
                println!("Using your favorite color, {color}, as the background");
            } else if is_tuesday {
                println!("Tuesday is green day!");
            } else if let Ok(age) = age {
                // this age variable is a shadow of the value inside the Ok()
                // but the scope of the shadow is not valid until the curly brackets start
                // so we can't do 'if let Ok(age) = age && age > 30'
                if age > 30 {
                    println!("Using purple as the background color");
                } else {
                    println!("Using orange as the background color");
                }
            } else {
                println!("Using blue as the background color");
            }
        }

        color();
    }

    fn while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        // this stops when pop returns a None() because that does not match anymore
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    fn for_loops() {
        // they have patterns too
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    fn let_statement() {
        let x = 5;
        // let PATTERN = EXPRESSION;
        let (x, y, z) = (1, 2, 3);
        let (x, y, _) = (1, 2, 3);
    }

    fn function_parameters() {
        fn foo(x: i32) {
            // code goes here
        }

        // x is a pattern

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
        print_coordinates(&point);
    }

    // if_let();
    // while_let();
    // for_loops();
    // let_statement();
    function_parameters();
}

fn ch_18_2() {
    // Patterns that can fail to match are refutable
    // Patterns that will match any possible value are irrefutable
    // let x = 5; is an irrefutable pattern
    // if let Some(x) = a_value is a reference pattern

    // function parameters and for loops can only take irrefutable patterns
    // if let and while let accept both, but the compiler will
    // warn you about irrefutable patterns because they don't
    // really make sense in context

    //? this does not compile
    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value;

    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    if let x = 5 {
        println!("{}", x);
    };
}

fn ch_18_3() {
    fn simple_match() {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    fn shadow_match() {
        let x = Some(5);
        let y = 10;

        match x {
            //this does not match
            Some(50) => println!("Got 50"),
            // this y is a shadow of y, and does not mean 10
            // it means any value
            // this is because this is a new scope
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }

    fn or_match() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    fn range_match() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    fn char_range() {
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    fn destructure() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        // these are the same
        // let Point { x, y } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        match p {
            // the match statement stops after a pattern matches
            // to {0,0} would be on the X axis here
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    fn enum_destructure() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }

    fn nested_structures() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    fn ignore_values() {
        //? ignore a parameter
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }

        {
            // there is a difference between using '_'
            //  and starting the name of the variable with '_'
            // "_" matches everything but does not bind
            // "_s" matches everything but it is still a variable and binds
            let s = Some(String::from("Hello!"));
            // s is move into _s here
            if let Some(_s) = s {
                println!("found a string");
            }

            // we are trying to use a variable that has been moved
            // println!("{:?}", s);
        }

        {
            let s = Some(String::from("Hello!"));

            if let Some(_) = s {
                println!("found a string");
            }
            println!("{:?}", s);
            // this works just fine
            // because the was no binding
        }

        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            // this looks a lot like lisp for some reason
            Point { x, .. } => println!("x is {}", x),
        }
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        let numbers = (2, 4, 8, 16, 32);

        //? this is ambiguous and will not compile
        // match numbers {
        //     (.., second, ..) => {
        //         println!("Some numbers: {}", second)
        //     },
        // }
    }

    fn match_guards() {
        let num = Some(4);

        // ! the compiler can no longer check for exhaustiveness when match guards are used
        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }

    fn fixing_shadowing_with_guards() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),

            // if n == y is not a pattern, so it does not introduce new shadowing
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);

        let x = 4;
        let y = false;

        match x {
            // if y applies to 4,5 and 6
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    fn bindings() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 3 };

        match msg {
            Message::Hello {
                // rename id if it is in the interval [3,7]
                id: id_variable @ 3..=7,
                // test and capture the variable
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                // println!("Found an id in range: {}", id);
                // ! can not find name id
                //? the value was not captured but the interval can match and the code 
                //? can be executed 
                println!("Found an id in another range")
            }
            // because any value can match this pattern we can use it 
            // there are no tests done on it 
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }

    // simple_match();
    // shadow_match();
    // or_match();
    // range_match();
    // char_range();
    // destructure();
    // enum_destructure();
    // ignore_values();
    // match_guards();
    // fixing_shadowing_with_guards();
    bindings();
}

pub fn main() {
    println!("CHAPTER 18");
    // ch_18_1();
    // ch_18_2();
    ch_18_3();
}
