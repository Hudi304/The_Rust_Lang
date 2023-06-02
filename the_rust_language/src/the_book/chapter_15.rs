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

// enum List {
//     // Cons(i32, List),
//     Cons(i32, Box<List>),
//     Nil,
// // }
// use List::{Cons, Nil};

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

// fn cons_list() {
//     // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     // let list = Cons(1, Cons(2, Cons(3, Nil)));
//     let b = Box::new(5);
//     println!("b = {}", b);

//     let x = 5;
//     let y = &x;
//     let z = Box::new(x);
//     let k = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//     assert_eq!(5, *z);
//     assert_eq!(5, *k);

//     // deref coercion
//     let m = MyBox::new(String::from("Rust"));
//     // &MyBox -> &String -> &str
//     hello(&m);

//     // this is the same thing, if Rust didn't implement it deref coercion
//     let m = MyBox::new(String::from("Rust"));
//     hello(&(*m)[..]);
// }

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

fn drop_trait() {
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

// For a graph for example every node must ber mutable at some point,
// but there are other nodes that point to it, so it might have multiple owners

// There is a way to enable multiple ownership, by using Rc<T>
// This is a reference counting smart pointer

// ! Rc<T> is only for use in single threaded scenarios

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn Rc_type() {
    let nil = Rc::new(Nil);
    let a2 = Rc::new(Cons(10, nil));
    let a = Rc::new(Cons(5, a2));

    // Clone does not make a deep copy of the data
    // rather it increments the reference count
    let b = Cons(3, Rc::clone(&a));
    // rather then using a.clone() (deep copy)
    // the syntax for incrementing the reference count
    // is Rc::clone(a) instead
    let c = Cons(4, Rc::clone(&a));
}

fn ref_count() {
    let nil = Rc::new(Nil);
    let a2 = Rc::new(Cons(10, nil));
    let a = Rc::new(Cons(5, a2));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// interior mutability design pattern
// allows you to mutate data even if it has
// immutable references
// this is unsafe

// the unsafe code is wrapped into a safe API

// the difference between Box<T> and RefCell<T>
// is that the borrow rules are enforced at compile time
// in the cas of Box<T>
// and at runtime in the cas of RefCell<T>

// a.k.a. if you break the rules the program will panic

//? do some research on the HaltingProblem

// recap:
// Rc<T> enables multiple owners of the same data
// Box<T> and RefCell<T> have single owners

// Box<T> allows mutable or immutable references at compile time
// Rc<T> allows only immutable borrows at compile time
// RefCell<T> allows mutable or immutable borrows at runtime

// Because RefCell<T> allows mutable borrows, you can mutate
// the value even when it is immutable

fn borrow_mutable() {
    let x = 5;
    // let y = &mut x;
}

fn limit_tracker_example() {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum ListMut {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// fn multiple_owner_interior_mutability() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(ListMut::Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = ListMut::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = ListMut::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }

// fn memory_leaks() {
//     // memory that is never cleaned up -> memory leak

//     use std::cell::RefCell;
//     use std::rc::Rc;
//     use List::{Cons, Nil};

//     #[derive(Debug)]
//     enum List {
//         Cons(i32, RefCell<Rc<List>>),
//         Nil,
//     }

//     impl List {
//         fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//             match self {
//                 Cons(_, item) => Some(item),
//                 Nil => None,
//             }
//         }
//     }
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
// }

fn tree() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

pub fn main() {
    // (*m) -> the value of the box (String)
    // &(String)[..] -> &str

    // ! NO RUNTIME PENALTY, it is all resolved at compile time

    // There is also the DerefMut trait that overrides the behavior
    // of the '*' operator on mutable references

    // Rust does deref coercion for these 3 cases :
    // - &T to &U when T:Deref<Target=U>
    // - &mut T to &mut U when T:DerefMut<Target=U>
    // - &mut T to & U when T:Deref<Target=U>

    // cons_list();
    // drop_trait();
    // Rc_type();
    // ref_count();
    // multiple_owner_interior_mutability();
    // memory_leaks();
    tree();
}
