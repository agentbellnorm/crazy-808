extern crate core;
extern crate mcp23017;
extern crate shared_bus;

mod buttons;
mod leds;

use crate::buttons::Buttons;
use crate::leds::Leds;
use engine::engine::Engine;
use engine::state::State;
use mcp23017::MCP23017;
use rppal::i2c::I2c;
use shared_bus::{BusManagerStd, I2cProxy};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Bus = &'static BusManagerStd<I2c>;
type Expander = MCP23017<I2cProxy<'static, Mutex<I2c>>>;

fn main() {
    let i2c = I2c::new().unwrap();
    let bus: &'static _ = shared_bus::new_std!(I2c = i2c).unwrap();
    let state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::sync_channel(1);
    let engine = Engine::new(state.clone(), engine_output_sender);

    engine.run();

    let mut leds = Leds::new(bus);
    let mut buttons = Buttons::new(bus);

    thread::spawn(move || loop {
        if let Some(pressed) = buttons.read() {
            engine.toggle_channel(pressed as i32);
        }
        thread::sleep(Duration::from_millis(50));
    });

    loop {
        if engine_output_receiver.recv().is_ok() {
            let arc = state.clone();
            let state = arc.lock().unwrap();
            leds.render(&state);
        }
    }
}
