use crate::{Bus, Expander};
use engine::core::{get_current_bar, get_instrument};
use engine::engine::NUMBER_OF_BARS;
use engine::state::State;
use mcp23017::MCP23017;

pub const LEDS_IC2_ADDRESS: u8 = 0x20;

pub struct Leds {
    expander: Expander,
}

fn to16arr(vec: &Vec<i32>) -> [u16; 16] {
    let mut arr: [u16; 16] = [0; 16];
    for i in 0..16 {
        arr[i as usize] = *vec.get(i).unwrap() as u16;
    }

    arr
}

impl Leds {
    pub fn new(bus: Bus) -> Self {
        let mut expander =
            MCP23017::new(bus.acquire_i2c(), LEDS_IC2_ADDRESS).expect("Could not get led expander");
        expander.all_pin_mode(mcp23017::PinMode::OUTPUT).unwrap(); // set all pins to outputs
        expander.write_gpioab(0).unwrap(); // set all pins to 0

        Leds { expander }
    }

    pub fn all_on(&mut self) {
        self.expander.write_gpioab(u16::MAX).unwrap()
    }

    pub fn all_off(&mut self) {
        self.expander.write_gpioab(0).unwrap();
    }

    pub fn light_leds(&mut self, leds: [u16; 16]) {
        let as_number = leds
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc | (bit as u16) << i);
        self.expander.write_gpioab(as_number).unwrap();
    }

    pub fn light_led(&mut self, led: i32) {
        println!("lighting {}", led);
        self.expander.write_gpioab(led as u16).unwrap()
    }

    pub fn render(&mut self, state: &State) {
        let current_bar = get_current_bar(state);
        let active_bars = get_instrument(state);

        let mut as_array = to16arr(active_bars);

        // light the led of the current sixteenth
        let actual_beat = (current_bar) % NUMBER_OF_BARS;
        as_array[actual_beat as usize] = 1;

        self.light_leds(as_array);
    }
}
