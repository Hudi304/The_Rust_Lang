////////////////////////////////
//? SMART POINTERS
////////////////////////////////
///

// smart pointers have additional metadata and capabilities
// there are smart pointers that own the data they point to

// String and Vec<T> are some smart pointers because they own some memory
// and allow you to manipulate it

// Smart pointers implement the Deref and Drop trait

// BOX
// -When you have a type whose size can’t be known at compile time
//  and you want to use a value of that type in a context that
//  requires an exact size
// -When you have a large amount of data and you want to transfer ownership
//  but ensure the data won’t be copied when you do so
// -When you want to own a value and you care only that it’s a type
//  that implements a particular trait rather than being of a specific type

enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let b = Box::new(5);
    println!("b = {}", b);
}
