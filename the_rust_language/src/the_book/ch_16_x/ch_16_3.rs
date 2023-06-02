pub fn mutex_example() {
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        // this will block the thread until the lock is open
        // also this will fail if the thread holding the lock panicked
        *num = 6;
    }

    println!("m = {:?}", m);
}

// fn mutex_threads() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         // you can't move counter in a closure multiple times
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// fn mutex_threads() {
//     // yeah, so Rc<T> can not guarantee that the reference count
//     // is correct when it comes from more then on thread
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

pub fn mutex_threads() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // because the counter is immutable and we can change it
    // it means that Mutex<T> implements the interior mutability pattern

    // as the use of Rc<T> can create references cycles
    // the use of Mutex<T> can create deadlocks
}
