extern crate bufstream;
extern crate rand;

/*
    The purpose of this module is to alleviate
    imports of many common I/O traits by adding
    a glob import to the top of I/O heavy modules.
    http://doc.rust-lang.org/std/io/prelude/
*/

use std::io::prelude::*;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

/*
    This crate provides a BufStream type which
    provides buffering of both the reading and
    writing halves of a Read + Write type.
    http://doc.rust-lang.org/std/io/prelude/
*/
use bufstream::BufStream;

/*
    <Experimental>
    http://doc.rust-lang.org/0.12.0/std/io/struct.BufferedStream.html
*/
// use std::io::BufferedStream

use rand::Rng;

fn handle_client(stream: TcpStream) {
    let mut bstream = BufStream::new(stream);
    let mut buffer;
    let mut upstream_buffer;
    let client_number = rand::thread_rng().gen_range(1, 101);
    let client_name = format!("Client-{}", client_number);
    let mut upstream = TcpStream::connect("127.0.0.1:8889").unwrap();

    println!("Connected to {}...", client_name);

    // Read value into buffer from stream
    loop {
        // Clear out buffer on each iteration
        buffer = vec![0; 512];
        upstream_buffer = vec![0; 512];

        let _ = match bstream.read(&mut buffer) {
            Err(e) => panic!("[Error] Client > Server: {}", e),
            Ok(n) => {
                if n == 0 {
                    break; // EOF so break
                }
                n
            }
        };

        // Do not need the number of bytes being written right now.
        io::stdout().write(&buffer.clone()).unwrap();

        let _ = match upstream.write(&buffer.clone()) {
            Err(e) => panic!("[Error] error writing to upstream: {}", e),
            Ok(n) => {
                n
            }
        };

        let _ = upstream.read(&mut upstream_buffer);

        let _ = match bstream.write(&upstream_buffer) {
            Err(e) => panic!("[Error] Server > Client: {}", e),
            Ok(_) => {
                let _ = bstream.flush();
            }
        };
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    println!("Listening on localhost:8888");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("Failed: {}", e) }
            Ok(stream) => {
                // Spawn a new thread for each connection
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
       }
    }
}
