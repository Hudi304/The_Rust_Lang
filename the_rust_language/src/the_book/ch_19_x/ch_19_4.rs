fn function_pointers() {
    // fn is a type
    // Fn is a trait

    // function pointers implement all 3 of the closure traits
    // Fn, FnMut, FnOnce
    // you can always pass a function as an argument
    // to a function that expects a closure

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // one place where you would want to accept just functions is
    // when interfacing with external code that does not have closures
    // like C
    // C function cant take functions as parameters
    // but C does not have closures

    let list_of_numbers = vec![1, 2, 3];

    // let string_list_iter = list_of_numbers.iter();
    // let to_string = |i| i.to_string();
    // let list_of_strings: Vec<String> = string_list_iter.map(to_string).collect();

    let string_list_iter = list_of_numbers.iter();
    let list_of_strings: Vec<String> = string_list_iter.map(ToString::to_string).collect();

    println!("{}", list_of_strings.join(","));

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn returning_closures() {
    // closures are represented by traits
    // that means we can not return closures directly

    // in most cases you can just return the concrete type
    // that implements the trait

    // you can not do that wit closures because they don't have a concrete type
    // you are also not allowed to return fn

    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    // rust can not know how much memory it will need to store the closure
    
    // just box it and put it on the heap
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

pub fn main() {
    //? ADVANCED FUNCTIONS AND CLOSURES
    println!("Ch 19.4");
    function_pointers();
    returning_closures();
}
