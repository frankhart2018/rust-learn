use std::thread;
use std::time::Duration;

fn main() {
    // Problems that can arise while using threads:-

    // 1. Race conditions, where threads are accessing data or resources
    // in an inconsistent order

    // 2. Deadlocks, where two threads are waiting for each other to 
    // finish using a resource the other thread has, preventing both
    // threads from continuing

    // 3. Bugs that happen only in certain situations and are hard
    // to reproduce and fix reliably

    //
    // Creating new thread with spawn
    //

    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // With the spawn() function the spawned thread will be stopped
    // when the main thread ends even if it has not finished running
}
