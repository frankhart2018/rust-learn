use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //
    // Using mutexes to allow accesses to data from one thread at a time
    // 

    // Mutex is an abbreviation for mutual exclusion, it allows only one thread
    // to access some data at any given time
    // To access data in a mutex, a thread must first signal that it wants access
    // by asking to acquire the mutex's lock
    // The lock is a data structure that is part of the mutex that keeps track
    // of who currently has exclusive access to the data
    // Therefore, mutex is described as guarding the data it holds via the locking system

    // Two rules for using mutexes:-

    // 1. We must attempt to acquire the lock before using the data

    // 2. When we're done with the data that the mutex guards, we must unlock
    // the data so other threads can acquire the lock

    //
    // The API of Mutex<T>
    //

    // Mutex in single-threaded context

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); 
        *num = 6;
    }

    println!("m = {:?}", m);

    // The lock() would fail if another thread holding the lock panicked
    // That is why we have specified unwrap()

    // Mutex<T> is a smart pointer, the call to lock() returns a smart pointer
    // called MutexGuard wrapped in a LockResult
    // MutexGuard implements Deref trait to point to the inner data
    // This also implements Drop implementation that releases the lock automatically
    // when a MutexGuard goes out of scope

    //
    // Sharing a Mutex<T> between multiple threads
    // 

    // This won't work as Rc is not safe to use with threads

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // Instead we will have to use Arc<T>
    // A stands for atomic, meaning it is an atomically reference counted type
    // Atomics are additional kind of concurrency primitive 

    // But all primitive types aren't atomic because this comes with a 
    // performance penalty

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

    // Mutex<T> provides interior mutability like Cell family

    // Mutex<T> comes with the risk of creating deadlocks
}
