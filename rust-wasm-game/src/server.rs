use wasm_bindgen::prelude::*;
use crate::common::*;
use crate::game::*;


// #[wasm_bindgen]
pub fn create_initial_game_states 
    (n_players: i32, seed: u64, max_rounds: i32, bump_interval: i32, n_rows: i32, n_cols: i32) -> String {

    let state = Game_states::new(n_players, seed, max_rounds, bump_interval, n_rows, n_cols);
    serde_json::to_string(&state).unwrap()

}


// take two inputs:
//    state_str: serialized game state
//    player_inputs_str: serialized player inputs
// output:
//    serialized game state
// #[wasm_bindgen]
pub fn update_game_states (state_str: String, player_inputs_str: String) -> String {

    let mut state: Game_states = serde_json::from_str(&state_str).unwrap();
    println!("new state = {:?}", &state);

    let player_inputs: Vec<Direction> = serde_json::from_str(&player_inputs_str).unwrap();
    println!("player inputs = {:?}", &player_inputs);


    if player_inputs.len() != state.player_states.len() {
        return "Error: player inputs len doesn't match player states len".to_owned();
    }

    // state.register_player_intents(&player_inputs);
    // state.update_player_positions();
    // state.step_game();

    state = update_game_states_no_json(state, player_inputs);

    serde_json::to_string(&state).unwrap()
}





pub fn update_game_states_no_json (mut state: Game_states, player_inputs: Vec<Direction>) -> Game_states {

    if player_inputs.len() != state.player_states.len() {
        return state;
        // return "Error: player inputs len doesn't match player states len".to_owned();
    }

    state.register_player_intents(&player_inputs);
    state.update_player_positions();
    state.step_game();

    state
}


