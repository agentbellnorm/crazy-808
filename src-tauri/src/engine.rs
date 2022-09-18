use std::{thread, time, sync::{Arc, Mutex, mpsc::Sender}};
use serde::{Serialize, Deserialize};
use crate::sound::{Sound};

const NUMBER_OF_CHANNELS: usize = 5;

type Variation = [[u8; 16]; NUMBER_OF_CHANNELS];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MachineState {
    pub variation_a: Variation,
    pub variation_b: Variation,
    pub playing: bool,
    pub bar: usize,
    pub current_variation: String,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            current_variation: "a".to_string(),
            variation_a: [
                [0; 16],
                [1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [0; 16]
            ],
            variation_b: [
                [0; 16],
                [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1],
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [0; 16]
            ],
            playing: false,
            bar: 0,
        }
    }
}

pub type SenderMessage = Result<(), String>;

pub struct Engine {
    state: Arc<Mutex<MachineState>>,
    sender: Sender<SenderMessage>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<MachineState>>, sender: Sender<SenderMessage>) -> Self {
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
            loop {
                let mut state = state_arc.lock().unwrap();

                if !state.playing {
                    std::mem::drop(state);
                    thread::sleep(time::Duration::from_millis(150));
                    continue;
                }   

                let current_bar = state.bar;
                state.bar += 1;
                if state.bar == 16 {
                    state.bar = 0;
                }

                let variation = match state.current_variation.as_str() {
                    "a" => state.variation_a,
                    "b" => state.variation_b,
                    _ => panic!("wtf variation")
                };

                let num_bars = state.variation_a.len();

                // drop the lock here, otherwise it will not be kept until after the sleep, blocking other threads.
                std::mem::drop(state);

                sender_2.send(Ok(())).unwrap_or_else(|m| panic!("Error when sending on channel from engine: {}", m));



                for (channel, _) in variation.iter().enumerate().take(num_bars) {
                    if variation[channel][current_bar] == 1 {
                        sound.play(channel);
                    }
                }

                thread::sleep(time::Duration::from_millis(200));
            }
        });
    }
}

