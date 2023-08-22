use crate::core::{get_channels_to_play, get_current_bar, get_next_instrument};
use crate::sound::Sound;
use crate::state::{Bar, State, Variation};
use spin_sleep::LoopHelper;
use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
    time::{Duration},
};

const NUMBER_OF_CHANNELS: i32 = 17;
pub const NUMBER_OF_BARS: i32 = 16;

#[derive(Debug)]
pub enum Direction {
    LEFT,
    RIGHT,
}

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
            bpm: 94,
        }
    }

    pub fn get_current_variation(&self) -> String {
        let current_bar = get_current_bar(self);
        return String::from(match self.current_variation.as_str() {
            "a" => "a",
            "b" => "b",
            "ab" => match current_bar < NUMBER_OF_BARS {
                true => "a",
                false => "b",
            },
            _ => panic!("wtf variation: {}", self.current_variation),
        });
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

        let mut loop_helper = LoopHelper::builder()
            .report_interval_s(0.5) // report every half a second
            .build_with_target_rate((state_arc.lock().unwrap().bpm / 60) * 4);

        thread::spawn(move || {
            let sound = Sound::new();
            loop {
                loop_helper.loop_start(); // or .loop_start_s() for f64 seconds

                let mut state = state_arc.lock().unwrap();
                if !state.playing {
                    drop(state);
                    thread::sleep(Duration::from_millis(200));
                    continue;
                }

                state.bar += 1;

                let channels_to_play = get_channels_to_play(&state);

                // drop the lock here, otherwise it will be kept until after the sleep, blocking other threads.
                drop(state);

                sender_2
                    .send(Ok(()))
                    .unwrap_or_else(|m| panic!("Error when sending on channel from engine: {}", m));

                loop_helper.loop_sleep();

                channels_to_play.into_iter().for_each(|channel| {
                    sound.play(channel as usize);
                });
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

    pub fn move_instrument_selector(&self, direction: Direction) {
        let mut state = self.state.lock().unwrap();
        state.selected_instrument = get_next_instrument(&state, direction);
    }

    pub fn get_selected_instrument(&self) -> i32 {
        self.state.lock().unwrap().selected_instrument
    }

    pub fn toggle_channel(&self, channel: i32) {
        let mut state = self.state.lock().unwrap();
        let selected_instrument = state.selected_instrument;

        let variation = match state.get_current_variation().as_str() {
            "a" => state.variation_a.as_mut().unwrap(),
            "b" => state.variation_b.as_mut().unwrap(),
            _ => panic!("wtf variation: {}", state.current_variation),
        };

        variation
            .instrument
            .get_mut(selected_instrument as usize)
            .unwrap()
            .bar[channel as usize] ^= 1; // toggle between 0 and 1
    }
}
