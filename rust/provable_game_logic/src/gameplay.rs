use crate::definition::SpinGameInitArgs;
use crate::definition::SpinGameIntermediateStates;
use crate::rng::LCGRandGen;
use crate::spin::SpinGame;
use crate::spin::SpinGameTrait;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub static NUMBER_THRESHOLD: u64 = 100;
pub static MAX_STEP_COUNT: u64 = 2000;
pub static MAX_SCORE: u64 = 5;

pub static GAME_STATE: Lazy<Mutex<SpinGameIntermediateStates>> =
    Lazy::new(|| Mutex::new(SpinGameIntermediateStates::new()));

impl SpinGameTrait for SpinGame {
    /* STATEFUL FUNCTIONS This defines the initialization of the game*/
    fn initialize_game(args: SpinGameInitArgs) {
        let mut game_state = GAME_STATE.lock().unwrap();

        game_state.random_seed = LCGRandGen::new(args.random_seed);
        game_state.current_number = game_state.random_seed.randint(NUMBER_THRESHOLD * 10);
        // game_state.current_number = 0;
    }

    /* STATEFUL FUNCTIONS This is defines the logic when player moves one step/entering one command*/
    fn step(input: u64) {
        let mut game_state = GAME_STATE.lock().unwrap();

        let is_game_over: bool = check_game_over(&game_state);

        if is_game_over {
            return;
        }

        match input {
            // 0 means a capture command, the only command that can be executed in the game
            0 => {
                if game_state.current_number < NUMBER_THRESHOLD {
                    game_state.score += 1;
                }

                game_state.current_number = game_state.random_seed.randint(NUMBER_THRESHOLD * 2);
                // game_state.current_number = 0;

                game_state.total_capture_tries += 1;
            }
            // skips the current number
            1 => {
                game_state.current_number = game_state.random_seed.randint(NUMBER_THRESHOLD * 2);
                // game_state.current_number = 0;
            }

            _ => {
                panic!("Invalid command");
            }
        };

        game_state.total_number_seen += 1;

        if check_game_over(&game_state) {
            game_state.game_over = 1;
        }
    }

    /* PURE FUNCTION This function returns the game state, to be used in Rust and Zkmain */
    fn get_game_state() -> SpinGameIntermediateStates {
        let game = GAME_STATE.lock().unwrap().clone();
        return game;
    }
}

fn check_game_over(game_state: &SpinGameIntermediateStates) -> bool {
    return game_state.total_number_seen >= MAX_STEP_COUNT || game_state.score >= MAX_SCORE;
}

#[wasm_bindgen]
pub fn greet() -> u64 {
    return 31213;
}
