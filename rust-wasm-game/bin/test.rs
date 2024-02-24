
extern crate game; 

use game::common::*;
use game::vec2::*;
use game::server::*;
use game::game::*;
use game::*;
use serde::*;


fn test_vec2() {
    let mut vector1 = Vector2::new(1, 2);
    let vector2 = Vector2::new(3, 4);

    vector1.add(&vector2);

    println!("Result: {:?}", vector1);

    // Accessing static constants
    println!("ZERO: {:?}", Vector2::zero());
    println!("RIGHT: {:?}", Vector2::right());
    // println!("LEFT: {:?}", Vector2::LEFT);
    // println!("UP: {:?}", Vector2::UP);
    // println!("DOWN: {:?}", Vector2::DOWN);
    // println!("ONE: {:?}", Vector2::ONE);
}



fn main() {

    const n_players: i32 = 1;
    const seed: u64 = 2345344;
    const max_round: i32 = 5;
    const bump_interval: i32 = 2;
    const n_rows: i32 = MAP_HEIGHT;
    const n_cols: i32 = MAP_WIDTH;

    let state_str = create_initial_game_states(n_players, seed, max_round, bump_interval, n_rows, n_cols);

    let player_inputs = vec![Direction::RIGHT; n_players as usize];
    let player_inputs_str = serde_json::to_string(&player_inputs).unwrap();
    println!("player inputs str = {}", player_inputs_str);

    // state transition
    let new_state_str = update_game_states(
        state_str,
        player_inputs_str,
    );

    let new_state: Game_states = serde_json::from_str(&new_state_str).unwrap();
    println!("{:?}", new_state);
}


