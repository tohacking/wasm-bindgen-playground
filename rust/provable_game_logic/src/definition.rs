use crate::rng::LCGRandGen;
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize)]
#[wasm_bindgen]
pub struct SpinGameInitArgs {
    // define your arguments here
    pub random_seed: u64,
    // pub testNumber: u64,
}

#[wasm_bindgen]
impl SpinGameInitArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(random_seed: u64) -> SpinGameInitArgs {
        SpinGameInitArgs {
            random_seed: random_seed,
        }
    }
}

#[wasm_bindgen]
pub fn create_spin_game_init_args(random_seed: u64) -> u64 {
    // SpinGameInitArgs::new(random_seed)
    return 9;
}

#[derive(Clone, Serialize)]
// #[wasm_bindgen]
pub struct SpinGameIntermediateStates {
    // define your game state here
    pub score: u64,
    pub total_number_seen: u64,
    pub total_capture_tries: u64,
    pub current_number: u64,
    pub random_seed: LCGRandGen,
    pub game_over: u64,
}

impl SpinGameIntermediateStates {
    pub fn new() -> SpinGameIntermediateStates {
        SpinGameIntermediateStates {
            score: 0,
            total_number_seen: 0,
            total_capture_tries: 0,
            current_number: 0,
            random_seed: LCGRandGen::new(0),
            game_over: 0,
        }
    }
}

impl fmt::Display for SpinGameIntermediateStates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GameState {{ score: {}, count: {}, tries: {}, current_number: {}, game_over: {} }}",
            self.score,
            self.total_number_seen,
            self.total_capture_tries,
            self.current_number,
            self.game_over
        )
    }
}
