use std::thread;

fn main() {
    // 
    // Using move closures with threads
    //

    // Using move the closure can take ownership of the data in closure

    // This technique is useful when creating new threads in order to
    // transfer ownership of values from one thread to another

    // thread::spawn does not take any arguments, so to use the values
    // from main thread it has to capture them

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Without move Rust infers how to capture v, and because println!
    // only needs a reference to v, the closure tries to borrow v
    // But Rust can't tell how long the spawned thread will run, so 
    // it doesn't know if the reference to v will always be valid
}
