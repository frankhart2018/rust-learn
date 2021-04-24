use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    // bind() returns Result<T, E>, which on success returns an instance of TcpListener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Thread pool
    let pool = ThreadPool::new(4);

    // The incoming() method on TcpListener returns an iterator that gives us sequence
    // of streams

    // A single stream of represents an open connection between the client and the
    // server 

    // A connection is the name for the full request and response process in which
    // a client connects to the server, the server generates a response, and the
    // server closes its connection

    // This for loop will thus process each connections and produce a series of streams
    // for us to handle

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // Unrecognizable utf-8 chars will be replaced with invalid sequence 
    // in this lossy function
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        println!("Sleep over, back to work!");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();

    // flush() will wait and prevent the program from continuing
    // until all the bytes are written to the connection
    stream.flush().unwrap();
}

// 
// Improving throughput with a thread pool
//

// A thread pool is a group of spawned threads that are waiting and ready
// to handle a task
// When the program receives a new task, it assigns one of the threads in
// the pool to the task, and that thread will process the task
// The remaining threads in the pool are available to handle any other
// tasks that come in while the first thread is processing
// When the first thread is done processing its task, it's returned 
// to the pool of idle threads, ready to handle a new task
                