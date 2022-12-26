#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use std::{
    io::{prelude::*, BufReader},
    // net::{TcpListener, TcpStream},
};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use async_std::task::spawn;
use std::time::Duration;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        // * for_each_concurrent processes each connection concurrently, 
        // but on the SAME thread
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();

            // 1. Single-Thread and concurrent: 
            // Serving Requests in Concurrently (using async code)
            // handle_connection(tcpstream).await;
    
            // 2. Multi-Thread (parallel) and concurrent: 
            // Serving Requests in Parallel
            spawn(handle_connection(tcpstream));
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // stream: HTTP request
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file) = if buffer.starts_with(get) {
        println!("--- buffer.starts_with get");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        println!("--- buffer.starts_with sleep");
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        println!("--- else 404.html");
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = std::fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("file not found: {}", file));

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}