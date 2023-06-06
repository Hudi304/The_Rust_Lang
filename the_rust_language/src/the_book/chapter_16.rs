use super::ch_16_x::ch_16_1;
use super::ch_16_x::ch_16_2;
use super::ch_16_x::ch_16_3;
use super::ch_16_x::ch_16_4;

// Concurrent -> different parts of the program execute independently
// Parallel -> different parts of the program execute at the same time

fn ch_16_1_main() {
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

    ch_16_1::move_example();
}

fn ch_16_2_main() {
    // send_string_through_channel();
    // use_after_sent();
    // send_multiple();
    ch_16_2::multiple_producers();
}

fn ch_16_3_main() {
    // message passing is not the only way
    // there is shared-state as well

    // MUTEX -> allows you to access data from one thread at a time
    // (mutual exclusion)

    // Mutexes have 2 rules
    // - attempt to acquire the lock before using the data
    // - when you're done you must unlock the mutex so other threads can access it

    // Rust's rule prevent you from making Mutex mistakes

    // mutex_example();
    ch_16_3::mutex_threads();
}

fn ch_16_4_main() {
    // there are only 2 things related to concurrency in rust
    // std::marker's Sync and Send

    //? SEND
    // Send indicates that the ownership of the value can be transferred between threads
    // almost all types in rust are Send

    // Rc<T> is not one of them  (more then one thread can update the reference count at the same time)
    // any type that is composed of Send types is marked as Send

    //? SYNC
    // indicates that it is safe for a type to be referenced by multiple threads
    // aka T is Sync if &T (immut ref) is Send 

    // Rc<T>, Cell<T>, RefCell<T> are not Sync
    // Mutex<T> is Sync

    //? how to implement them?
    // you don't they are marker traits (interfaces)
    // they do not contain any methods
    
}

pub fn main() {
    // ch_16_1_main();
    // ch_16_2_main();
    ch_16_3_main();
}
