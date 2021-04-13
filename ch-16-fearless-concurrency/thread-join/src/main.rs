use std::thread;
use std::time::Duration;

fn main() {
    //
    // Waiting for all threads to finish using join handles
    //

    // We can fix the problem of the thread ending prematurely or not
    // getting to run at all by saving the return value of thread::spawn
    // This returns a JoinHandle, which is an owned value that when we 
    // call join method on it will wait for its thread to finish

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
