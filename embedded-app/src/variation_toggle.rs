use crate::shared_pull_up_input_expander::SharedPullUpInputExpander;
use std::ops::Range;

pub struct VariationToggle {
    shared_expander: SharedPullUpInputExpander,
    pin_range: Range<usize>,
    toggle_state: [bool; 2],
}

impl VariationToggle {
    pub fn new(shared_expander: SharedPullUpInputExpander, pin_range: Range<usize>) -> Self {
        VariationToggle {
            shared_expander,
            pin_range,
            toggle_state: [false, false],
        }
    }

    pub fn read(&mut self) -> Option<&str> {
        let all_bits = self.shared_expander.read_all_bits();
        let [prev_a, prev_b] = self.toggle_state;
        let [read_a, read_b]: [bool; 2] = all_bits[self.pin_range.clone()].try_into().unwrap();

        if read_a == prev_a && read_b == prev_b {
            return None;
        }

        self.toggle_state = [read_a, read_b];

        Some(match [read_a, read_b] {
            [false, false] => "ab",
            [true, false] => "a",
            [false, true] => "b",
            [true, true] => panic!("impossible!!"),
        })
    }
}
