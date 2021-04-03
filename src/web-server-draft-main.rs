use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};

use tokio::sync::mpsc;
use tokio::task;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

enum AudioCommand {
    Play,
    Stop
}

// Content functions
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    let (audio_com_tx, mut audio_com_rx) = mpsc::channel(32);

    let audio_runtime = tokio::task::spawn(async move {
        while let Some(command) = audio_com_rx.recv().await {
            match command {
                AudioCommand::Play => {
                    // Get a output stream handle to the default physical sound device
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    // Load a sound from a file, using a path relative to Cargo.toml
                    let file = BufReader::new(File::open("samples/heart-beat-a.wav").unwrap());
                    // Decode that sound file into a source
                    let source = Decoder::new(file).unwrap();
                    // Play the sound directly on the device
                    stream_handle.play_raw(source.convert_samples()).unwrap();

                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                AudioCommand::Stop => ()
            }
        }
    });

    // // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String)
    //     .map(move |name| async {
    //         audio_com_tx.send(AudioCommand::Play);
    //         format!("Hello, {}!", name)
    //     });

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    

    // warp::serve(hello)
    //     .run(([127, 0, 0, 1], 3030))
    //     .await;

    audio_runtime.await.unwrap();
}