use crate::{Bus, Expander};
use mcp23017::{PinMode, MCP23017};

#[derive(Clone)]
pub struct SharedPullUpInputExpander {
    expander: Expander,
}

const SHARED_PULL_UP_INPUT_EXPANDER_IC2_ADDRESS: u8 = 0x22;

impl SharedPullUpInputExpander {
    pub fn new(shared_ic2_bus: Bus) -> Self {
        let mut expander = MCP23017::new(
            shared_ic2_bus.acquire_i2c(),
            SHARED_PULL_UP_INPUT_EXPANDER_IC2_ADDRESS,
        )
        .unwrap();

        expander.all_pin_mode(PinMode::INPUT).unwrap(); // or for all pins

        for pin in 0..16 {
            expander.pull_up(pin, true).unwrap();
        }

        SharedPullUpInputExpander { expander }
    }

    pub fn read_all_bits(&mut self) -> [bool; 16] {
        let value = self.expander.read_gpioab().unwrap();
        let mut read_bits: [bool; 16] = [false; 16];

        for i in 0..16 {
            let mask = 1 << i;
            read_bits[i] = if value & mask == 0 { true } else { false };
        }

        read_bits
    }
}
