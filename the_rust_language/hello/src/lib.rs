use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // this will panic if the system does not have any more threads available

        let thread = thread::spawn(move || loop {
            // the lock can pe poisoned if a thread that was holding the lock panics
            // let locked_receiver = receiver.lock().unwrap();

            // this blocks so if there is no joc yet, the current thread will wait for one
            // let job = locked_receiver.recv().unwrap();
            // ! why does this not work??
            // because the mutex is locked back up in the next line

            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");

            job();
        });

        return Worker { id, thread };
    }

    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(move || loop {
    //         let job = receiver.lock().unwrap().recv().unwrap();

    //         println!("Worker {id} got a job; executing.");

    //         job();
    //     });

    //     Worker { id, thread }
    // }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // this preallocates space in the vector
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for worker_id in 0..size {
            // create some threads and store them in the vector

            // multiple threads will change the receiver
            let thread_safe_receiver = Arc::clone(&receiver);
            //? bump the reference count of the atomic reference
            let worker = Worker::new(worker_id, thread_safe_receiver);
            workers.push(worker);
        }

        return ThreadPool { workers, sender };
    }

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
