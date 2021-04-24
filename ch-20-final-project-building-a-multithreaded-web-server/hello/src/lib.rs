use std::{thread, usize};
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>, // Will hold all the jobs to be sent
}

type Job = Box<dyn FnOnce() + Send + 'static>;  

enum Message {
    NewJob(Job),
    Terminate,
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

        // Send the code to execute via sender-receiver approach

        // This won't work though because this is multiple producer
        // single consumer model
        let (sender, receiver) = mpsc::channel();

        // Thread safe smart pointer to share ownership across multiple threads
        // And allow the threads to mutate the value
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create some threads and store them in one vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    // When the pool is dropped and our threads should all join to make
    // sure they finish their work
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        // We need two loops here because if we used a single loop to iterate
        // through each worker, on the first iteration a terminate message would
        // be sent down the channel and join called on the first worker's thread
        // If that first worker was busy processing a request at that moment,
        // the second worker would pick up the terminate message from the channel
        // and shut down
        // We would be left waiting on the first worker to shut down, but it never
        // would because the second thread picked up the terminate message, Deadlock!
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}

// We will use FnOnce as the trait for passing the closure to execute
// method, this is because we want the thread to execute only once

// The F type parameter also has trait bound Send and lifetime bound 
// 'static, we need Send to transfer the closure from one thread to another
// and 'static because we don't know how long the thread will take to execute

// Since the standard thread::spawn requires some code to be passed immediately
// and we won't have that so we implement this manually

// A Worker struct will be used here, this is a data structure common in
// pooling implementations

// The ThreadPool will hold vector of Worker structs each of which will hold
// a single JoinHandle<()> instance