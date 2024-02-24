use std::time::{SystemTime, UNIX_EPOCH};
use serde::*;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::vec2::Vector2;



pub const MAX_N_PLAYERS: i32 = 4;

pub const BASE_TILE_SIZE: i32 = 32;
pub const SCALING_FACTOR: i32 = 1;
pub const TILE_SIZE: i32 = BASE_TILE_SIZE * SCALING_FACTOR;

pub const BANNER_HEIGHT: i32 = TILE_SIZE * 5 / 4; // 1.25 as an integer
pub const PIXEL_WALKED_PER_SEC: i32 = TILE_SIZE * 4;

pub const MAP_WIDTH: i32 = 13;
pub const MAP_HEIGHT: i32 = 10;

pub const SCREEN_WIDTH: i32 = TILE_SIZE * MAP_WIDTH;
pub const SCREEN_HEIGHT: i32 = TILE_SIZE * MAP_HEIGHT + BANNER_HEIGHT;



pub const INITIAL_WAIT: i64 = 1000;
pub const WAITING_TIME: i64 = 1000;
pub const WARNING_TIME: i64 = 3000;
pub const FIRING_TIME: i64 = 3000;
pub const FADING_WAIT: i64 = 2000;
pub const FADING_TIME: i64 = INITIAL_WAIT;
pub const ROUND_TIME: i64 = WAITING_TIME + WARNING_TIME + FIRING_TIME;





pub fn current_milliseconds() -> u64 {
    // SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards")
    //     .as_millis() as u64 // cast from u128 to u64
    return 1000;
}



#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// #[wasm_bindgen]
pub enum GameState {
    STARTING = 0,
    WAITING = 1,
    WARNING = 2,
    FIRING = 3,
    FINISHED = 4,
}

pub const GameStateList: [GameState; 5] = 
    [GameState::STARTING, GameState::WAITING, GameState::WARNING, GameState::FIRING, GameState::FINISHED];


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
// #[wasm_bindgen]
pub enum Direction {
    NONE,
    LEFT,
    UP,
    RIGHT,
    DOWN,
}


lazy_static! {
    pub static ref DIRECTION_VECS: HashMap<Direction, Vector2> = {
        let mut map = HashMap::new();
        map.insert(Direction::UP, Vector2::new(0, -1));
        map.insert(Direction::DOWN, Vector2::new(0, 1));
        map.insert(Direction::LEFT, Vector2::new(-1, 0));
        map.insert(Direction::RIGHT, Vector2::new(1, 0));
        map.insert(Direction::NONE, Vector2::new(0, 0));
        map
    };
}




// hardcoded map
pub fn is_tile_blocking(x: i32, y: i32) -> bool {
    (x <= 0) || (x >= MAP_WIDTH - 1) || (y <= 0) || (y >= MAP_HEIGHT - 1)
}

