
use wasm_bindgen::prelude::*;
use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;


use zkwasm_rust_sdk::{
    wasm_input,
    require,
    // wasm_dbg
};


pub mod common;
pub mod lcg_rand_gen; // Assuming you placed LCGRandGen code in lcgrandgen.rs or lcgrandgen/mod.rs
pub mod vec2;
pub mod player;
pub mod server;
pub mod game;


use crate::common::*;
use crate::vec2::*;
use crate::server::*;
use game::Game_states;
use player::PlayerState;

// =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> 


// static mut POSITION: i32 = 100;

// #[wasm_bindgen]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[wasm_bindgen]
// pub fn get_position() -> i32 {
//     unsafe { POSITION }
// }

// #[wasm_bindgen]
// pub fn perform_command(command: i32) {
//     if command == 0 {
//         unsafe { POSITION -= 1 }
//     } else {
//         unsafe { POSITION += 1 }
//     }
// }





#[wasm_bindgen]
pub fn zkmain() {
    unsafe {

        let mut state =  Game_states {
            state: GameState::STARTING,
            round: 0,
            n_fire_throws: 1,
            n_players: 1,
            max_rounds: 5,
            bump_interval: 2,
            n_rows: 3,
            n_cols: 3,

            ts: 25,
            start_ts: 10,
            last_update_ts: 10,
            warning_starting_ts: 10,
            end_game_ts: 10,

            player_states: vec![ PlayerState {
                id: 0,
                pixelpos: Vector2::new(10, 10),
                tilepos: Vector2::new(1, 1),
                direction: Direction::NONE,
                moving: false, 
                exp_gained: 0,
                eliminated_ts: 0,
                tile_size_pixel_walked: 0,
                last_movement_intent: Direction::NONE
            }
            ],
            row_firing: vec![false, false, false],
            col_firing: vec![false, false, false],
            lcg_rand_seed: 23424,
        };

        let pis = vec![Direction::RIGHT];
        state = update_game_states_no_json(state, pis);

    }
}

