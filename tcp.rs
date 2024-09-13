//importing the necessary libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    //read from the stream
    stream.read(&mut buffer).expect("Failed to read from stream");
    //write to the stream
    
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    
    let response = "Hello Client!".as_bytes();
    stream.write(response).expect("Failed to write to stream");

}

// entry point of the program
fn main() {
    // create a tcp listener
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    // accept connections and process them in a separate function
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}