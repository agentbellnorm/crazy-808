use std::{thread, time, sync::{Arc, Mutex}};
use crate::sound::Sound;


pub struct MachineState {
    pub drum: String,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            drum: format!("sd"),
        }
    }
}

pub struct Engine {
    state: Arc<Mutex<MachineState>>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<MachineState>>) -> Self {
        Engine {
            state,
        }
    }

    pub fn run(&self) {
        let state_arc = self.state.clone();

        thread::spawn(move || {
            let sound = Sound::new();
            loop {
                match state_arc.lock().unwrap().drum.as_str() {
                    "sd" => sound.play_snare(),
                    "bd" => sound.play_base_drum(),
                    _ => panic!("what drum??")
                }

                thread::sleep(time::Duration::from_millis(50));
            }
        });
    }
}

