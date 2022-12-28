extern crate core;
extern crate mcp23017;

mod leds;

use crate::leds::LEDS;
use engine::engine::Engine;
use engine::state::State;
use std::error::Error;
use std::os::macos::raw::stat;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::sync_channel(1);
    let engine = Engine::new(state, engine_output_sender);

    engine.run();

    let mut leds = LEDS::new();

    thread::spawn(move || loop {
        if engine_output_receiver.recv().is_ok() {
            let state = state.lock().unwrap();
            let variation = state.variation_a.unwrap();
            let i = state.selected_instrument;
            let instrument_active_bars = variation.instrument.get(i).unwrap();
            &leds.light_leds(instrument_active_bars);
        }
    });
}
