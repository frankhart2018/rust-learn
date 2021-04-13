use std::{sync::mpsc, thread};
use std::time::Duration;

fn main() {
    // A channel has a transmitter and a receiver
    // It is closed when either the transmitter or revceiver is dropped

    let (tx, rx) = mpsc::channel();

    // mpsc - multi producer single consumer
    // This can have multiple sending ends that produce value
    // but only one receiving end that consumes those values

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // send returns a Result<T, E>
        // send takes ownership of the value so we cannot use val variable anymore after send()
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // recv() blocks the main thread's execution and wait 
    // until a value is sent down the channel
    // Once sent, recv will return a Result<T, E>
    // When the sending end of the channel is closes, recv will 
    // return an error to signal that no more values will be coming
    
    // try_recv() method doesn't block, but will return a Result<T, E> immediately
    // an Ok value holding a message if one is available and an Err value if there
    // aren't any messages this time
    // This is useful when the thread has other work to do while waiting for messages

    //
    // Sending multiple values and seeing the receiver waiting
    //

    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiver can not be treated as an iterator
    for received in rx1 {
        println!("Got: {}", received);
    }

    // 
    // Creating multiple producers by cloning the transmitter
    //

    let (tx2, rx2) = mpsc::channel();

    let tx2_1 = tx2.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2_1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }
}
