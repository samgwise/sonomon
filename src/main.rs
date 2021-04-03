use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, Source};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("samples/lp-noise.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap().buffered().repeat_infinite();
    sink.append(source);

    // // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples());

    sink.sleep_until_end();
}