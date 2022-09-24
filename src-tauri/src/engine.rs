use crate::sound::Sound;
use crate::state::{Bar, State, Variation};
use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread, time,
};

const NUMBER_OF_CHANNELS: usize = 17;
const NUMBER_OF_BARS: usize = 16;

impl State {
    pub fn initial() -> Self {
        State {
            current_variation: "ab".to_string(),
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
            playing: true,
            bar: 0,
            selected_instrument: 1,
        }
    }
}

pub type SenderMessage = Result<(), String>;

// fn s(st: ) -> String {
//     st.to_string()
// }

pub struct Engine {
    state: Arc<Mutex<State>>,
    sender: SyncSender<SenderMessage>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<State>>, sender: SyncSender<SenderMessage>) -> Self {
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
                    thread::sleep(time::Duration::from_millis(100));
                    continue;
                }

                state.bar += 1;

                let current_bar = state.bar
                    % match state.current_variation.as_str() {
                        "ab" => 32,
                        _ => 16,
                    };

                let variation = match state.current_variation.as_str() {
                    "a" => state.variation_a.clone().unwrap(),
                    "b" => state.variation_b.clone().unwrap(),
                    "ab" => match current_bar < 16 {
                        true => state.variation_a.clone().unwrap(),
                        false => state.variation_b.clone().unwrap(),
                    },
                    _ => panic!("wtf variation: {}", state.current_variation),
                };

                // drop the lock here, otherwise it will be kept until after the sleep, blocking other threads.
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
                        .get(current_bar as usize % 16)
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

    pub fn set_variation(&self, variation: String) {
        let mut state = self.state.lock().unwrap();
        state.current_variation = variation;
    }

    pub fn toggle_playing(&self) {
        let mut state = self.state.lock().unwrap();
        state.playing ^= true;
    }

    pub fn set_selected_instrument(&self, instrument: i32) {
        let mut state = self.state.lock().unwrap();
        state.selected_instrument = instrument;
    }

    pub fn toggle_channel(&self, channel: i32) {
        let mut state = self.state.lock().unwrap();
        let selected_instrument = state.selected_instrument;
        let variation = match state.current_variation.as_str() {
            "a" => state.variation_a.as_mut().unwrap(),
            "b" => state.variation_b.as_mut().unwrap(),
            _ => panic!("variation must be a or b"),
        };

        variation
            .instrument
            .get_mut(selected_instrument as usize)
            .unwrap()
            .bar[channel as usize] ^= 1; // toggle between 0 and 1
    }
}
