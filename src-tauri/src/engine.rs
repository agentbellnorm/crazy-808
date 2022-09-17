use std::{thread, time, sync::{Arc, Mutex, mpsc::Sender}};
use crate::sound::Sound;

const NUMBER_OF_CHANNELS: usize = 3;

type Variation = [[u8; 16]; NUMBER_OF_CHANNELS];

pub struct MachineState {
    pub variation_a: Variation,
    pub variation_b: Variation,
    pub playing: bool,
    pub current_variation: String,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            current_variation: "a".to_string(),
            variation_a: [
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            ],
            variation_b: [
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1],
            ],
            playing: false,
        }
    }
}

pub struct Engine {
    state: Arc<Mutex<MachineState>>,
    sender: Sender<usize>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<MachineState>>, sender: Sender<usize>) -> Self {
        Engine {
            state,
            sender,
        }
    }

    pub fn run(&self) {
        let state_arc = self.state.clone();

        let sender_2 = self.sender.clone();

        thread::spawn( move || {
            let sound = Sound::new();
            let mut bar = 0;
            loop {
                let state = state_arc.lock().unwrap();

                let variation = match state.current_variation.as_str() {
                    "a" => state.variation_a,
                    "b" => state.variation_b,
                    _ => panic!("wtf variation")
                };

                let num_variations = state.variation_a.len();

                std::mem::drop(state);

                for (channel, _) in variation.iter().enumerate().take(num_variations) {
                    if variation[channel][bar] == 1 {
                        sound.play(channel);
                    }
                }

                sender_2.send(bar).unwrap_or_else(|m| panic!("Error when sending on channel from engine: {}", m));

                bar += 1;
                if bar == 16 {
                    bar = 0;
                }

                thread::sleep(time::Duration::from_millis(200));
            }
        });
    }
}

