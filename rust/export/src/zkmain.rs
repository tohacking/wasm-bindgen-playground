use provable_game_logic::definition::SpinGameInitArgs;
use provable_game_logic::definition::SpinGameIntermediateStates;
use provable_game_logic::spin::SpinGame;
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::wasm_output;

use provable_game_logic::spin::SpinGameTrait;

/*
    PUBLIC INPUTS marked by `wasm_input`: e.g wasm_input(1) specifies a public input of type u64
    PRIVATE INPUTS marked by `wasm_input`: e.g wasm_input(0) specifies a priva  te input of type u64
    PUBLIC OUTPUTS marked by `wasm_output`: e.g wasm_output(var) specifies an output `var` of type u64
*/
#[wasm_bindgen]
pub fn zkmain() -> i64 {
    //     uint256 _transactionNonce = uint64(instances[0][0]);
    //     uint256 _gameId = uint64(instances[0][1]);
    //     address _inputSigner = parseAddress([instances[0][2], instances[0][3], instances[0][4]]);
    //     bytes memory _stateChanges = parseOutputToBytes(instances[0][5]);
    // specify the public inputs

    let randomSeed: u64 = unsafe { wasm_input(1) };
    let transactionNonce: u64 = unsafe { wasm_input(1) };
    let gameId: u64 = unsafe { wasm_input(1) };
    let inputSigner0: u64 = unsafe { wasm_input(1) };
    let inputSigner1: u64 = unsafe { wasm_input(1) };
    let inputSigner2: u64 = unsafe { wasm_input(1) };

    SpinGame::initialize_game(SpinGameInitArgs {
        random_seed: randomSeed,
    });

    // specify the private inputs
    let private_inputs_length = unsafe { wasm_input(0) };

    for _i in 0..private_inputs_length {
        let _private_input = unsafe { wasm_input(0) };
        SpinGame::step(_private_input);
    }

    unsafe {
        let final_game_state: SpinGameIntermediateStates = SpinGame::get_game_state();
        zkwasm_rust_sdk::dbg!("final_game_state: {}\n", final_game_state);

        // specify the output
        wasm_output(final_game_state.score as u64);
        wasm_output(final_game_state.total_number_seen as u64);
        wasm_output(final_game_state.total_capture_tries as u64);
        wasm_output(final_game_state.current_number as u64);
        wasm_output(final_game_state.game_over as u64);
    }

    0
}
