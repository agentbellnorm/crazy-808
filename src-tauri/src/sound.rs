use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, OutputStreamHandle, source::{Buffered, Source}};

type SoundSource = Buffered<Decoder<BufReader<File>>>;

fn source(sound_file_string_path: &str) -> SoundSource {
    let file = BufReader::new(File::open(sound_file_string_path).unwrap());
    Decoder::new(file).unwrap().buffered()
}

pub const HIHAT: usize = 0;
pub const SNARE: usize = 1;
pub const BASE_DRUM: usize = 2;

struct Sources {
    sd: SoundSource,
    bd: SoundSource,
    ch: SoundSource,
}

pub struct Sound {
    stream_handle: OutputStreamHandle,
    _output_stream: OutputStream, // Someone needs to own it, but it's not used directly
    sources: Sources,
}

impl Sound {
    pub fn new() -> Self {
        let (output_stream, stream_handle) = OutputStream::try_default().unwrap();
        Sound {
            stream_handle,
            _output_stream: output_stream,
            sources: Sources {
                bd: source("audio/BD/BD5050.WAV"),
                sd: source("audio/SD/SD5050.WAV"),
                ch: source("audio/CH/CH.WAV"),
            }
        }
    }

    pub fn play_snare(&self) {
        self.stream_handle.play_raw(self.sources.sd.clone().convert_samples()).unwrap();
    
    }
    pub fn play_base_drum(&self) {
        self.stream_handle.play_raw(self.sources.bd.clone().convert_samples()).unwrap();
    }

    fn play_source(&self, sound_source: &SoundSource) {
        self.stream_handle.play_raw(sound_source.clone().convert_samples()).unwrap();
    }

    pub fn play(&self, channel: usize) {
        let sound_source = match channel {
            0 => &self.sources.ch,
            1 => &self.sources.bd,
            2 => &self.sources.sd,
            _ => panic!("no such channel: {}", channel)
        };

        self.play_source(sound_source);
    }
}


