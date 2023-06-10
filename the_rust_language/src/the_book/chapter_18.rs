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

pub fn main() {
    println!("CHAPTER 18");
    // if_let();
    // while_let();
    // for_loops();
    // let_statement();
    function_parameters();
}
