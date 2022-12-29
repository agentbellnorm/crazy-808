use crate::engine::NUMBER_OF_BARS;
use crate::state::{Bar, State, Variation};

pub fn get_current_bar(state: &State) -> i32 {
    state.bar
        % match state.current_variation.as_str() {
            "ab" => NUMBER_OF_BARS * 2,
            _ => NUMBER_OF_BARS,
        }
}

pub fn get_variation(state: &State) -> &Variation {
    let current_bar = get_current_bar(state);
    let variation = match state.current_variation.as_str() {
        "a" => &state.variation_a,
        "b" => &state.variation_b,
        "ab" => match current_bar < NUMBER_OF_BARS {
            true => &state.variation_a,
            false => &state.variation_b,
        },
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
