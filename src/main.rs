use rodio::{Decoder, OutputStream, Sink, Source};

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(fs::File::open("samples/noise.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap().buffered().repeat_infinite();
    sink.append(source);

    // // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples());

    // Let the webserver do the
    // sink.sleep_until_end();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        if sink.is_paused() {
            sink.play()
        }
        else {
            sink.pause();
        }

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
