use std::{thread, time, sync::{Arc, Mutex, mpsc}};
use std::ops::DerefMut;
use crate::sound::Sound;

const NUMBER_OF_CHANNELS: usize = 3;

pub struct MachineState {
    pub variation_a: [[u8; 16]; NUMBER_OF_CHANNELS],
    pub variation_b: [[u8; 16]; NUMBER_OF_CHANNELS],
    pub playing: bool
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            variation_a: [
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            ],
            variation_b: [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                [1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            ],
            playing: false,
        }
    }
}

pub struct Engine<'a> {
    state: Arc<Mutex<MachineState>>,
    beat_callback: Option<Arc<Mutex<dyn FnMut() + 'a + Sync + Send>>>,
}

impl<'a> Engine<'a> {
    pub fn new(state: Arc<Mutex<MachineState>>) -> Self {
        Engine {
            state,
            beat_callback: None,
        }
    }

    pub fn onBeat(mut self, callback: impl FnMut() + 'a + Sync + Send) {
        self.beat_callback = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn run(&self) {
        let state_arc = self.state.clone();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let sound = Sound::new();
            let mut bar = 0;
            loop {
                let state = state_arc.lock().unwrap();

                for channel in 0..state.variation_a.len() {
                    if state.variation_a[channel][bar] == 1 {
                        sound.play(channel);
                    }
                }
                
                bar += 1;
                if bar == 16 {
                    bar = 0;
                }

                tx.send(()).unwrap();

                thread::sleep(time::Duration::from_millis(200));
            }
        });

        for _ in rx {
            if let Some(c) = &self.beat_callback {
                c.lock().unwrap()();
            }
        }
    }
}

