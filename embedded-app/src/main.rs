extern crate core;
extern crate mcp23017;
extern crate shared_bus;

mod buttons;
mod instrument_select;
mod leds;
mod shared_pull_up_input_expander;
mod variation_toggle;

use crate::buttons::Buttons;
use crate::instrument_select::InstrumentSelect;
use crate::leds::Leds;
use crate::shared_pull_up_input_expander::SharedPullUpInputExpander;
use crate::variation_toggle::VariationToggle;
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
    let shared_expander = SharedPullUpInputExpander::new(bus);
    let mut instrument_select = InstrumentSelect::new(shared_expander.clone(), 0..2);
    let mut variation_toggle = VariationToggle::new(shared_expander, 2..4);

    thread::spawn(move || loop {
        // works fine with 50ms
        if let Some(pressed) = buttons.read() {
            engine.toggle_channel(pressed as i32);
        }

        // // seems to work best around 10ms
        if let Some(turn) = instrument_select.read() {
            engine.move_instrument_selector(turn);
            println!("instrument: {}", engine.get_selected_instrument());
        }

        if let Some(variation) = variation_toggle.read() {
            engine.set_variation(variation.to_string());
            println!("variation: {}", engine.get_variation());
        }

        thread::sleep(Duration::from_millis(1));
    });

    loop {
        if engine_output_receiver.recv().is_ok() {
            let arc = state.clone();
            let state = arc.lock().unwrap();
            leds.render(&state);
        }
    }
}
