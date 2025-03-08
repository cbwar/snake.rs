use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder};
use std::fs::File;
use std::io::BufReader;

pub fn play_snd(stream_handle: &OutputStreamHandle) -> Result<(), rodio::PlayError> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("resources/eat.ogg").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples())
}

mod tests {

    // use super::*;

    // #[test]
    // fn test_play_snd() {
    //     play_snd();
    // }
}
