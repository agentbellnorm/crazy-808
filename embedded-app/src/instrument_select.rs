use crate::shared_pull_up_input_expander::SharedPullUpInputExpander;
use engine::engine::Direction;
use std::ops::Range;

const INSTRUMENT_SELECT_IC2_ADDRESS: u8 = 0x22;

pub struct InstrumentSelect {
    clk_state: bool,
    shared_expander: SharedPullUpInputExpander,
    pin_range: Range<usize>,
}

impl InstrumentSelect {
    pub fn new(shared_expander: SharedPullUpInputExpander, pin_range: Range<usize>) -> Self {
        InstrumentSelect {
            clk_state: false,
            shared_expander,
            pin_range,
        }
    }

    pub fn read(&mut self) -> Option<Direction> {
        let all_bits = self.shared_expander.read_all_bits();
        let instrument_select_bits: [bool; 2] =
            all_bits[self.pin_range.clone()].try_into().unwrap();

        let [dt, clk] = instrument_select_bits;

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
