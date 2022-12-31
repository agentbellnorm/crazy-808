use crate::{Bus, Expander};
use engine::engine::Direction;
use mcp23017::{PinMode, MCP23017};

const INSTRUMENT_SELECT_IC2_ADDRESS: u8 = 0x22;

pub struct InstrumentSelect {
    clk_state: bool,
    expander: Expander,
}

impl InstrumentSelect {
    pub fn new(shared_ic2_bus: Bus) -> Self {
        let mut expander =
            MCP23017::new(shared_ic2_bus.acquire_i2c(), INSTRUMENT_SELECT_IC2_ADDRESS).unwrap();

        expander.all_pin_mode(PinMode::INPUT).unwrap(); // or for all pins

        for pin in 0..16 {
            expander.pull_up(pin, true).unwrap();
        }

        InstrumentSelect {
            clk_state: false,
            expander,
        }
    }

    pub fn read(&mut self) -> Option<Direction> {
        let value = self.expander.read_gpioab().unwrap();
        let mut read_bits: [bool; 2] = [false; 2];

        for i in 0..2 {
            let mask = 1 << i;
            read_bits[i] = if value & mask == 0 { true } else { false };
        }

        let [dt, clk] = read_bits;

        let mut direction = None;

        if clk != self.clk_state {
            if !clk {
                if !dt {
                    direction = Some(Direction::LEFT);
                } else {
                    direction = Some(Direction::RIGHT);
                }
            }

            self.clk_state = clk;
        }

        direction
    }
}
