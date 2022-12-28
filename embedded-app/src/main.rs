extern crate core;
extern crate mcp23017;

mod leds;

use crate::leds::LEDS;
use engine::engine::Engine;
use engine::state::State;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn to16arr<T>(vec: Vec<T>) -> [T; 16] {
    vec.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()))
}

fn main() {
    println!("main!");
    let state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::sync_channel(1);
    let engine = Engine::new(state.clone(), engine_output_sender);

    engine.run();

    let mut leds = LEDS::new();

    loop {
        if engine_output_receiver.recv().is_ok() {
            println!("state!");
            let arc = state.clone();
            let state = arc.lock().unwrap();
            let i = state.selected_instrument;
            let current = state.bar;
            let instrument_active_bars = &state
                .variation_a
                .as_ref()
                .unwrap()
                .instrument
                .get(i as usize)
                .unwrap()
                .bar;
            let _ = leds.light_leds(to16arr(instrument_active_bars.clone()));
            // let _ = leds.light_led(current);
        }
    }
}
