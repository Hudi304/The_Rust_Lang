// Concurrent -> different parts of the program execute independently
// Parallel -> different parts of the program execute at the same time

use std::thread;
use std::time::Duration;

fn no_guarantee() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn no_guarantee_even_worse() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
    }
}

fn join_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn join_before_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// fn print_vec_in_thread() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v);
//     });
//     // error is thread may outlive the vector
//     // this makes sense
//     // the main thread might end before the spawned thread
//     // so the reference will not be valid

//     //?  fair enough this can happen as well
//     // drop(v); // oh no!

//     handle.join().unwrap();
// }

fn move_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });


    //?  value used after move
    // drop(v); // oh no!

    handle.join().unwrap();
}

fn ch_16_1() {
    // in most OSs an executed program's code is run in a process
    // the OS manages processes

    // in a program you can have different bits of code that
    // run at the same time

    // these bits of code are run by a THREAD

    // there is no guarantee of the order theses bits of code executed
    // this can lead to:
    // 1) Race conditions (threads access data in an inconsistent order)
    // 2) DeadLocks (2 threads are waiting for each other to finish)
    // 3) Bugs that are hard to reproduce

    // OSs provide an API that a language can call to create new threads
    // Rust's  std has a 1:1 model
    // 1 OS thread = 1 language thread

    // when the main thread completes all the spawned threads are shut down

    // no_guarantee();
    // no_guarantee_even_worse();

    // join_example();
    // join_before_example();
    // print_vec_in_thread();
    move_example();
}

pub fn main() {
    ch_16_1();
}
