use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Write;

fn handle_client (mut stream: TcpStream) {
    let mut buf;

    // Read value into buffer from stream
    loop {
        // Clear out buffer on each iteration
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // EOF so break
                    break;
                }
                m
            },
        };

        // Nope doesn't work.. something about the Format trait
        // not being implemented :(
        // println!("GOT A MESSAGE: {}", buf);

        // write it back
        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Listening on localhost:8888");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("Failed: {}", e) }
            Ok(stream) => {
                handle_client(stream)
            }
       }
    }
}
