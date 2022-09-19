use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, source::{Buffered, Source}};

type SoundSource = Buffered<Decoder<BufReader<File>>>;

fn source(sound_file_string_path: &str) -> SoundSource {
    let file = BufReader::new(File::open(sound_file_string_path).unwrap());
    Decoder::new(file).unwrap().buffered()
}

// pub const HIHAT: usize = 0;
// pub const SNARE: usize = 1;
// pub const BASE_DRUM: usize = 2;

struct Sources {
    bd: SoundSource,
    sd: SoundSource,
    lt: SoundSource,
    mt: SoundSource,
    ht: SoundSource,
    lc: SoundSource,
    mc: SoundSource,
    hc: SoundSource,
    rs: SoundSource,
    cl: SoundSource,
    cp: SoundSource,
    ma: SoundSource,
    cb: SoundSource,
    cy: SoundSource,
    oh: SoundSource,
    ch: SoundSource
}

impl Sources {
    pub fn default() -> Sources {
        Sources {
            bd: source("audio/BD/BD5050.WAV"),
            sd: source("audio/SD/SD5050.WAV"),
            lt: source("audio/LT/LT50.WAV"),
            mt: source("audio/MT/MT50.WAV"),
            ht: source("audio/HT/HT50.WAV"),
            lc: source("audio/LC/LC50.WAV"),
            mc: source("audio/MC/MC50.WAV"),
            hc: source("audio/HC/HC50.WAV"),
            rs: source("audio/RS/RS.WAV"),
            cl: source("audio/CL/CL.WAV"),
            cp: source("audio/CP/CP.WAV"),
            ma: source("audio/MA/MA.WAV"),
            cb: source("audio/CB/CB.WAV"),
            cy: source("audio/CY/CY5050.WAV"),
            oh: source("audio/OH/OH50.WAV"),
            ch: source("audio/CH/CH.WAV"),
        }
    }

    pub fn get_by_short(&self, st: &str) -> &SoundSource {
        match st {
            "bd" => &self.bd,
            "sd" => &self.sd,
            "lt" => &self.lt,
            "mt" => &self.mt,
            "ht" => &self.ht,
            "lc" => &self.lc,
            "mc" => &self.mc,
            "hc" => &self.hc,
            "rs" => &self.rs,
            "cl" => &self.cl,
            "cp" => &self.cp,
            "ma" => &self.ma,
            "cb" => &self.cb,
            "cy" => &self.cy,
            "oh" => &self.oh,
            "ch" => &self.ch,
            _ => panic!("no soundsource defined for instrument {}", st)
        }
    }

    pub fn get_by_id(&self, id: usize) -> &SoundSource {
        match id {
            1 => &self.bd,
            2 => &self.sd,
            3 => &self.lt,
            4 => &self.mt,
            5 => &self.ht,
            6 => &self.lc,
            7 => &self.mc,
            8 => &self.hc,
            9 => &self.rs,
            10 => &self.cl,
            11 => &self.cp,
            12 => &self.ma,
            13 => &self.cb,
            14 => &self.cy,
            15 => &self.oh,
            16 => &self.ch,               
            _ => panic!("no such channel: {}", id)
        }
    }
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
            sources: Sources::default(),
        }
    }

    fn play_source(&self, sound_source: &SoundSource) {
        self.stream_handle.play_raw(sound_source.clone().convert_samples()).unwrap();
    }

    pub fn play(&self, channel: usize) {
        let sound_source  = self.sources.get_by_id(channel);

        self.play_source(sound_source);
    }
}


