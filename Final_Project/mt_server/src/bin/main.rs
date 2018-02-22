use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs::File;
use std::thread;
use std::time::Duration;

extern crate mt_server;
use mt_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //We're now going to introduce a thread pool to handle the request so that we don't run out of the
    //number of threads. An async method with a set thread pool would probably still be the best method for this.
    //We still need to create a method for this though.
    let pool = ThreadPool::new(4);

    //In practice we wouldn't have this shut down after 2 requests but this shows how it can shut down gracefully.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        //Previous example didn't really read any request from the server.
        // handle_connection(stream);
        //Here we're going to explore having the code create a new thread for every
        //connection. However, it should be noted that the problems with this route are that
        //it has the potential to spawn an infinite number of threads which might crash the system.
        //thread::spawn just creates a new thread and runs the code in the closure. 
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        //We replace the thread::spawn with the pool.execute which works similar to the thread spawn method.
        //We need to create this method first.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting Down.");
}

//We've made the stream mutable in this example but it wasn't mutable above...
//It apparently needs to be mutable because of the fact that state changing can be going on.
fn handle_connection(mut stream: TcpStream) {
    //This declares the buffer on the stack as being 512 bytes
    //If arbitary buffer size was needed this would become more complex.
    let mut buffer = [0; 512];
    //This reads the bytes from TcpStream and put them into the buffer
    stream.read(&mut buffer).unwrap();
    // //The String::from_utf8_lossy takes in a &[u8] and returns a String. The lossy part
    // //makes it return a  � anytime invalid characters are seen.
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //We are now checking to see if the URI returned is /

    let get = b"GET / HTTP/1.1\r\n";
    //Here we're going to simulate what a slow request to a single threaded server is like
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //We've reduced the amount of repeated code by using a let if-else statement
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        //Here we're forcing our thread to sleep for 5 seconds.
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//The browser is taking a request of this form
// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body
//The Method says whether is getting or posting something
//Request-URI is pretty similar to URL
//HTTP-Version is exactly what it sounds like
//CRLF is either a line feed or a carriage return
//The rest is the request data.

//Responses take the following format:
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body

//So we have the HTTP-Version used in the response, a numeric status code summarizing the results,
//and a reason phrase that provides a text description of the status code.