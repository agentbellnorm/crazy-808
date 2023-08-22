use crate::engine::{Direction, NUMBER_OF_BARS};
use crate::state::{Bar, State, Variation};

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    n.max(min).min(max)
}

pub fn get_current_bar(state: &State) -> i32 {
    state.bar
        % match state.current_variation.as_str() {
            "ab" => NUMBER_OF_BARS * 2,
            _ => NUMBER_OF_BARS,
        }
}

pub fn get_variation(state: &State) -> &Variation {
    let variation = match state.get_current_variation().as_str() {
        "a" => &state.variation_a,
        "b" => &state.variation_b,
        _ => panic!("wtf variation: {}", state.current_variation),
    };
    match variation {
        Some(v) => v,
        None => panic!("waah"),
    }
}

/*
   Vec of indices for each instrument to play on the current bar
*/
pub fn get_channels_to_play(state: &State) -> Vec<i32> {
    let current_bar = get_current_bar(state);
    (0..(NUMBER_OF_BARS + 1))
        .into_iter()
        .filter(|channel| {
            get_variation(state)
                .instrument
                .get(*channel as usize)
                .unwrap()
                .bar
                .get((current_bar % NUMBER_OF_BARS) as usize)
                .unwrap()
                == &1
        })
        .collect::<Vec<i32>>()
}

pub fn get_instrument(state: &State) -> &Vec<i32> {
    let instrument_id = state.selected_instrument as usize;
    &get_variation(state)
        .instrument
        .get(instrument_id)
        .unwrap()
        .bar
}

pub fn get_next_instrument(state: &State, direction: Direction) -> i32 {
    let as_number = match direction {
        Direction::LEFT => -1,
        Direction::RIGHT => 1,
    };

    clamp(state.selected_instrument + as_number, 1, 16)
}
