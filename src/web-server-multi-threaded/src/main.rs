#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

// Single vs Multi-threaded web server
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("---1.a th id: {:?}",std::thread::current().id());

    // // 3. todo: Multi threaded using pool
    // let pool = web_server::ThreadPool::new(4);

    let single_threaded = true;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if single_threaded {
            // 1. Single threaded
            handle_connection(stream);
        } else {
            // 2. Multi-threaded not using pool
            // create a new thread, then run the code in the closure in new thread
            println!("---1.b th id: {:?}",std::thread::current().id());
            std::thread::spawn(|| {
                println!("---2.a th id: {:?}",std::thread::current().id());
                handle_connection(stream);
            });
                // problem: this will eventually overwhelm the system because 
                // you’d be making new threads without any limit.

            // // 3. Multi threaded using pool
            // println!("---1.b th id: {:?}",std::thread::current().id());
            // pool.execute(|| {
            //     println!("---2.a th id: {:?}",std::thread::current().id());
            //     handle_connection(stream);
            // });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("---2.b th id: {:?}",std::thread::current().id());

    // Optional: Print HTTP request
    // print_http_request(stream);

    let status_line = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn print_http_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
}