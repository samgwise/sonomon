// Following: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    let (status, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("[{request_line}] => [{status}] {file_name}");

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}