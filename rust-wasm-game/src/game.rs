use serde::*;
use wasm_bindgen::prelude::*;
use crate::common::*;
use crate::player::*;
use crate::lcg_rand_gen::*;
use crate::vec2::*;
use std::cmp;



// ServerToClientMessage struct equivalent in Rust
#[derive(Debug, Serialize, Deserialize)]
// #[wasm_bindgen]          // let's use json
pub struct Game_states {
    pub state: GameState,
    pub round: i32,
    pub n_fire_throws: i32,
    pub n_players: i32,
    pub max_rounds: i32,
    pub bump_interval: i32,
    pub n_rows: i32,
    pub n_cols: i32,

    pub ts: u64,  // current ts
    pub start_ts: u64,  // when this game started
    pub last_update_ts: u64,
    pub warning_starting_ts: u64,   // 0 for default
    pub end_game_ts: u64,           // 0 for default

    // these can be merged 
    pub player_states: Vec<PlayerState>,    
    pub row_firing: Vec<bool>,
    pub col_firing: Vec<bool>,

    pub lcg_rand_seed: u64,
}



// #[wasm_bindgen]          // let's use json
impl Game_states {

    pub fn new (n_players: i32, seed: u64, 
                max_rounds: i32, bump_interval: i32, 
                n_rows: i32, n_cols: i32) -> Game_states {

        let mut rand = LCGRandGen::new(seed);

        // create state 
        let mut player_states = Vec::new();
        for id in 0..n_players {
    
            let tilepos = Vector2::new(
                (rand.randint(n_cols as u32 - 2) + 1) as i32,
                (rand.randint(n_rows as u32 - 2) + 1) as i32
            );
    
            let player = PlayerState {
                id,
                tilepos,
                pixelpos: player_tilepos_to_pixelpos(tilepos),
                direction: Direction::NONE, 
                moving: false,
                exp_gained: 0,
                eliminated_ts: 0,
                tile_size_pixel_walked: 0,
                last_movement_intent: Direction::NONE
            };
    
            player_states.push(player);
        }
    
        // pack 
        Game_states {
            state: GameState::STARTING,
            round: 0,
            n_fire_throws: 0,
            n_players,
            max_rounds,
            bump_interval,
            n_rows,
            n_cols,
            
            player_states,
            row_firing: vec![false; n_rows as usize], // Example row firing statuses
            col_firing: vec![false; n_cols as usize], // Example column firing statuses
    
            start_ts: current_milliseconds(),
            ts: current_milliseconds(),
            last_update_ts: current_milliseconds(),
            warning_starting_ts: 0, 
            end_game_ts: 0, 
    
            lcg_rand_seed: rand.seed,
        }
    }


    // 1. register move intents:  TS: movePlayer(intent);
    pub fn register_player_intents(&mut self, player_inputs: &Vec<Direction>) {
        for (i, dir) in player_inputs.iter().enumerate() {
            let state = &mut self.player_states[i];

            state.last_movement_intent = *dir;
            if !state.moving {
                state.direction = *dir;   // copy value
                if state.is_direction_blocked(dir) {
                    state.moving = false;
                } else{
                    state.moving = true;
                    state.update_tilepos();
                }
            }
        }
    }


    // 2. update player pixel poses    TS: update(delta: number)
    pub fn update_player_positions(&mut self) {

        let delta = (current_milliseconds() - self.last_update_ts) as i32;
        self.last_update_ts = current_milliseconds();
    
        for mut player in &mut self.player_states {
            player.last_movement_intent = Direction::NONE;
            if player.moving {
    
                let pixesl_to_walk = PIXEL_WALKED_PER_SEC * delta / 1000;
                let will_cross_border = player.tile_size_pixel_walked + pixesl_to_walk >= TILE_SIZE;
    
                // move if not crossing border
                if !will_cross_border {
                    player.update_pixelpos(pixesl_to_walk);
    
                // continue/stop if crossing border
                } else {
                    let should_continue_moving = 
                        player.direction == player.last_movement_intent &&
                        !player.is_direction_blocked(&player.last_movement_intent);
    
                    if should_continue_moving {
                        player.update_pixelpos(pixesl_to_walk);
                        player.update_tilepos();
                
                    } else {
                        player.update_pixelpos(TILE_SIZE - player.tile_size_pixel_walked);
                        player.moving = false;
                    }
                } // will_cross_border
            }  // if state.moving
        }

    }


    pub fn reward_survivors<F>(&mut self, func: F)    
    where
        F: Fn(i32) -> i32,
    {
        for mut player in &mut self.player_states {
            if player.eliminated_ts == 0 {
                player.exp_gained = func(player.exp_gained);
            }
        }
    }


    pub fn end_game(&mut self) {
        self.state = GameState::FINISHED;
        self.end_game_ts = current_milliseconds();
    }


    //game update:   TS: game:update(player_pos: Vector2[]) 
    pub fn step_game(&mut self) {
        if self.state == GameState::FINISHED {
            return;
        }

        let ts = current_milliseconds() as i64;
        // println!("{} {} {}", ts, self.s)
        let lapse = ts - self.start_ts as i64 - INITIAL_WAIT as i64;
        if lapse < INITIAL_WAIT as i64 {
            return;
        }
    
        let last_round = self.round;
        self.round = cmp::max(0, (lapse / ROUND_TIME as i64) as i32);
    

        // new round 
        let new_round = self.round != last_round;
        if new_round {
    
            let n_fire_throws = self.n_fire_throws.clone();
            self.reward_survivors(|x| x + n_fire_throws);
        
            // if all players have been elimited, end game
            let all_players_eliminated = 
                self.player_states.iter().filter(|s| s.eliminated_ts > 0).count() as i32 
                == self.n_players;
                        
            if all_players_eliminated {
                self.end_game();
                return;
            }
        }


        // game won
        let game_won = self.round >= self.max_rounds;
        if game_won {
            self.end_game();
            self.reward_survivors(|x| x * 2);
            return;
        }


        // fire and warning
        self.n_fire_throws = (self.round / self.bump_interval) + 1;
        let round_lapse = lapse % ROUND_TIME;
        let previous_state = self.state;
        self.state = if round_lapse < WAITING_TIME {
            GameState::WAITING
        } else if round_lapse < WAITING_TIME + WARNING_TIME {
            GameState::WARNING
        } else {
            GameState::FIRING
        };

        if self.state != previous_state && self.state == GameState::WARNING {
            self.determine_fire_throws();
        }

        if self.state == GameState::FIRING {
            self.check_fire();
        }

    }



    pub fn determine_fire_throws(&mut self) {
        
        self.warning_starting_ts = current_milliseconds();
        self.row_firing = vec![false; self.n_rows as usize];
        self.col_firing = vec![false; self.n_cols as usize];
        
        for i in 0..self.n_fire_throws {

            let res = randint(self.lcg_rand_seed, self.n_rows + self.n_cols - 2 - 2);
            self.lcg_rand_seed = res.0;
            let r = res.1;

            if r < self.n_rows - 2 {
                self.row_firing[(r + 1) as usize] = true;
            } else {
                self.col_firing[(r - (self.n_rows - 2) + 1) as usize] = true;
            }
        }    
    }


    pub fn check_fire(&mut self) {
        
        for mut player in &mut self.player_states {
            if player.eliminated_ts > 0 {
                continue;
            }

            for idx in 0..self.n_rows {
                if self.row_firing[idx as usize] && idx == player.tilepos.y {
                    player.eliminated_ts = current_milliseconds() as i32;
                }
            }

            for idx in 0..self.n_cols {
                if self.col_firing[idx as usize] && idx == player.tilepos.x {
                    player.eliminated_ts = current_milliseconds() as i32;
                }
            }
        }
    }


}


