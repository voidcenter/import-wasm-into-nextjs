use wasm_bindgen::prelude::*;


const LCG_M: u64 = 4294967296;
const LCG_A: u64 = 22695477;
const LCG_C: u64 = 1;


// #[wasm_bindgen]
pub struct LCGRandGen {
    pub seed: u64,
}

// #[wasm_bindgen]
impl LCGRandGen {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
        }
    }

    pub fn randint(&mut self, max: u32) -> u32 {
        self.seed = (self.seed * LCG_A + LCG_C) % LCG_M;
        (self.seed % (max as u64)) as u32
    }
}


pub fn randint(seed: u64, max: i32) -> (u64, i32) {
    let seed = (seed * LCG_A + LCG_C) % LCG_M;
    (seed, (seed % (max as u64)) as i32)
    // (self.seed % (max as u64)) as u32
}



// pub fn randint(seed: u32) -> u32 {
    

//     self.seed = (self.seed.wrapping_mul(self.lcg_a).wrapping_add(self.lcg_c)) % self.lcg_m;
//     self.seed % max
// }

