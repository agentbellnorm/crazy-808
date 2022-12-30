use crate::{Bus, Expander};
use mcp23017::{PinMode, MCP23017};
use rppal::i2c::I2c;
use std::sync::Mutex;

const BUTTONS_IC2_ADDRESS: u8 = 0x21;

pub struct Buttons {
    expander: Expander,
    buttons_state: [bool; 16],
}

impl Buttons {
    pub fn new(shared_ic2_bus: Bus) -> Self {
        let mut expander =
            MCP23017::new(shared_ic2_bus.acquire_i2c(), BUTTONS_IC2_ADDRESS).unwrap();
        expander.all_pin_mode(PinMode::INPUT).unwrap(); // or for all pins

        for pin in 0..16 {
            expander.pull_up(pin, true).unwrap();
        }

        Buttons {
            expander,
            buttons_state: [false; 16],
        }
    }

    pub fn read(&mut self) {
        let value = self.expander.read_gpioab().unwrap();
        let mut read_bits: [bool; 16] = [false; 16];

        for i in 0..16 {
            let mask = 1 << i;
            read_bits[i] = if value & mask == 0 { true } else { false };
        }

        for i in 0..16 {
            match (self.buttons_state[i], read_bits[i]) {
                (false, true) => println!("{} up", i + 1),
                (true, false) => println!("{} down", i + 1),
                _ => continue,
            }

            self.buttons_state[i] = read_bits[i];
        }
    }
}
