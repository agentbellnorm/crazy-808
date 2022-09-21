use crate::sound::Sound;
use crate::state::{Bar, State, Variation};
use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread, time,
};

const NUMBER_OF_CHANNELS: usize = 17;
const NUMBER_OF_BARS: usize = 16;

impl State {
    pub fn initial() -> Self {
        State {
            current_variation: "a".to_string(),
            variation_a: Some(Variation {
                instrument: vec![
                    Bar { bar: vec![0; 16] },
                    Bar {
                        bar: vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    },
                    Bar {
                        bar: vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                    },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar {
                        bar: vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                    },
                ],
            }),
            variation_b: Some(Variation {
                instrument: vec![
                    Bar { bar: vec![0; 16] },
                    Bar {
                        bar: vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    },
                    Bar {
                        bar: vec![0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1],
                    },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar { bar: vec![0; 16] },
                    Bar {
                        bar: vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                    },
                ],
            }),
            playing: false,
            bar: 0,
            selected_instrument: 1,
        }
    }
}

pub type SenderMessage = Result<(), String>;

pub struct Engine {
    state: Arc<Mutex<State>>,
    sender: Sender<SenderMessage>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<State>>, sender: Sender<SenderMessage>) -> Self {
        Engine { state, sender }
    }

    pub fn run(&self) {
        let state_arc = self.state.clone();

        let sender_2 = self.sender.clone();

        thread::spawn(move || {
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
                    "a" => state.variation_a.clone().unwrap(),
                    "b" => state.variation_b.clone().unwrap(),
                    _ => panic!("wtf variation"),
                };

                // drop the lock here, otherwise it will not be kept until after the sleep, blocking other threads.
                std::mem::drop(state);

                sender_2
                    .send(Ok(()))
                    .unwrap_or_else(|m| panic!("Error when sending on channel from engine: {}", m));

                for channel in 0..17 {
                    if variation
                        .instrument
                        .get(channel)
                        .unwrap()
                        .bar
                        .get(current_bar as usize)
                        .unwrap()
                        .clone()
                        == 1
                    {
                        sound.play(channel);
                    }
                }

                thread::sleep(time::Duration::from_millis(200));
            }
        });
    }
}
