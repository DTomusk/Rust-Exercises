// HTTP and TCP are both request response protocols
// TCP describes how the information gets from one server to another
// HTTP specifies the information being transferred
// in the vast majority of cases HTTP sends data over TCP

// there are certain traits in the prelude that let us read from and write to the stream
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    // server listens on the given address
    // connecting to a port to listen is called binding to a port
    // bind returns a result meaning it can fail (we can't access some ports)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);

    // incoming returns an iterator containing a sequence of streams
    // a single stream represents an open connection between a client and a server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // basic multi threadedness
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// stream must be mutable because TcpStream keeps track of what data it returns internally
fn handle_connection(mut stream: TcpStream) {
    // the buffer holds the data that is read in
    let mut buffer = [0; 512];
    // takes bytes from the stream and reads them to the buffer
    stream.read(&mut buffer).unwrap();

    // the start of the request we're handling
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"Get /sleep HTTP/1.1\r\n";

    // remove repetition by running the same code regardless of what the response is
    let (status_line, filename) = if buffer.starts_with(get) {
        // if the data in the buffer starts with get then we know we've received a well formed request
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // simulate a slow request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        // if buffer doesn't start with get then we've received some other request
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // get the contents of the page from our html file
    let contents = fs::read_to_string(filename).unwrap();

    // set the appropriate response from the start of the buffer
    let response = format!("{}{}", status_line, contents);

    // write our response
    stream.write(response.as_bytes()).unwrap();
    // ensure execution doesn't stop until all bytes have been written
    stream.flush().unwrap();
}

// a thread pool is a set of spawned threads that are waiting to handle requests
// when a task is received a thread is assigned to it
// we limit the number of threads to prevent denial of service attacks
