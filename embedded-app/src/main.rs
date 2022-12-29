extern crate core;
extern crate mcp23017;

mod leds;

use crate::leds::LEDS;
use engine::engine::Engine;
use engine::state::State;
use std::sync::{Arc, Mutex};

fn main() {
    let state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::sync_channel(1);
    let engine = Engine::new(state.clone(), engine_output_sender);

    engine.run();

    let mut leds = LEDS::new();

    loop {
        if engine_output_receiver.recv().is_ok() {
            let arc = state.clone();
            let state = arc.lock().unwrap();
            leds.render(&state);
        }
    }
}
