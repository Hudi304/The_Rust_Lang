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

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let k = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *k);

    // deref coercion
    let m = MyBox::new(String::from("Rust"));
    // &MyBox -> &String -> &str
    hello(&m);

    // this is the same thing, if Rust didn't implement it deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

// The Drop trait can be used to make a little code run every time a variable 
// needs to be deallocated from the heap
// the compiler will call that code for you

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn main() {
    cons_list();
    // (*m) -> the value of the box (String)
    // &(String)[..] -> &str

    // ! NO RUNTIME PENALTY, it is all resolved at compile time

    // There is also the DerefMut trait that overrides the behavior
    // of the '*' operator on mutable references

    // Rust does deref coercion for these 3 cases :
    // - &T to &U when T:Deref<Target=U>
    // - &mut T to &mut U when T:DerefMut<Target=U>
    // - &mut T to & U when T:Deref<Target=U>

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}
