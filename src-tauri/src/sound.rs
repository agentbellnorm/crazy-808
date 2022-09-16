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

struct Sources {
    sd: SoundSource,
    bd: SoundSource,
}

pub struct Sound {
    stream_handle: OutputStreamHandle,
    output_stream: OutputStream,
    sources: Sources,
}

impl Sound {
    pub fn new() -> Self {
        let (output_stream, stream_handle) = OutputStream::try_default().unwrap();
        Sound {
            stream_handle,
            output_stream,
            sources: Sources {
                bd: source("audio/BD/BD5050.WAV"),
                sd: source("audio/SD/SD5050.WAV"),
            }
        }
    }

    pub fn play_snare(&self) {
        self.stream_handle.play_raw(self.sources.sd.clone().convert_samples()).unwrap();
    
    }
    pub fn play_base_drum(&self) {
        self.stream_handle.play_raw(self.sources.bd.clone().convert_samples()).unwrap();
    }
}


