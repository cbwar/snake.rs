use rodio::{source::Source, Decoder};
use rodio::OutputStreamHandle;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Sound {
    Eat,
    Start,
    GameOver,
}

pub struct SoundSystem {
    stream_handle: Box<OutputStreamHandle>,
    sounds: HashMap<Sound, String>,
}
impl Debug for SoundSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SoundSystem")
    }
}
impl SoundSystem {
    pub fn new(stream_handle: OutputStreamHandle) -> SoundSystem {
        let sounds = HashMap::from([
            (Sound::Start, String::from("resources/eat.ogg")),
            (Sound::Eat, String::from("resources/eat.ogg")),
            (Sound::GameOver, String::from("resources/eat.ogg")),
        ]);
        SoundSystem {
            stream_handle: Box::new(stream_handle),
            sounds,
        }
    }
    pub fn play_snd(&self, snd: Sound) -> Result<(), rodio::PlayError> {
        // Get the file form the hashmap
        let filename = self.sounds.get(&snd).unwrap();
        println!("Playing sound: {}", filename);
        let file = BufReader::new(File::open(filename).unwrap());
        let source = Decoder::new(file).unwrap();
        self.stream_handle.play_raw(source.convert_samples())
    }
}

mod tests {

    // use super::*;

    // #[test]
    // fn test_play_snd() {
    //     play_snd();
    // }
}
