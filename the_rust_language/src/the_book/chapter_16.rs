// Concurrent -> different parts of the program execute independently
// Parallel -> different parts of the program execute at the same time

use std::thread;
use std::time::Duration;

use super::ch_16_x::ch_16_1::move_example;

pub fn ch_16_1() {
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
