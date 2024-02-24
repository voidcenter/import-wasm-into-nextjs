
use serde::*;
use wasm_bindgen::prelude::*;
use crate::common::*;
use crate::vec2::Vector2;




pub fn player_tilepos_to_pixelpos (tilepos: Vector2) -> Vector2 {
    const OFFSET_X: i32 = TILE_SIZE / 2;
    const OFFSET_Y: i32 = TILE_SIZE * 7 / 10;

    Vector2::new(
        tilepos.x * TILE_SIZE + OFFSET_X,
        tilepos.y * TILE_SIZE + OFFSET_Y + BANNER_HEIGHT
    )
}


// =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> =-> 



// PlayerState struct equivalent in Rust
#[derive(Debug, Serialize, Deserialize)]
// #[wasm_bindgen]
pub struct PlayerState {
    pub id: i32,
    pub pixelpos: Vector2,
    pub tilepos: Vector2,
    pub direction: Direction,
    pub moving: bool,
    pub exp_gained: i32,      // for simplicity, use i32 everywhere
    pub eliminated_ts: i32,   // for simplicity, eliminated_ts == 0 means the player is still in the game
    pub tile_size_pixel_walked: i32,
    pub last_movement_intent: Direction,
}


impl PlayerState {

    pub fn is_direction_blocked(&self, dir: &Direction) -> bool {
        let mut pos = self.tilepos.clone();
        pos.add(DIRECTION_VECS.get(dir).unwrap());
        is_tile_blocking(pos.x, pos.y)
    }

    pub fn update_tilepos(&mut self) {
        self.tilepos.add(DIRECTION_VECS.get(&self.direction).unwrap());
    }

    pub fn update_pixelpos(&mut self, pixels_to_walk: i32) {
        let mut dir_vec = DIRECTION_VECS[&self.direction].clone();
        dir_vec.mul(pixels_to_walk);
        self.pixelpos.add(&dir_vec);

        self.tile_size_pixel_walked += pixels_to_walk;
        self.tile_size_pixel_walked %= TILE_SIZE;
    }
}


