use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body7 {
    pub position: f32,        // [-1,1]
    pub learnedResponse: f32, // [-1,1]
}

pub struct Environment7 {
    pub safeZoneLow: f32,  // [-1,1]
    pub safeZoneHigh: f32, // [-1, 1] > safeZoneLow
}

pub fn death(org: &Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) -> bool {
    (org.body.position < env.safeZoneLow || org.body.position > env.safeZoneHigh)
        && rng.gen::<f32>() < 0.9
}

pub fn reproduce(s: &BaseSeq, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    // Always double
    (0..2)
        .map(|_| clone_with_mutation(s, rng, 0.01, 0.01, 0.05))
        .collect()
}

/**
 * Each base conveys 2 bits of information.
 * Take the first 4 bases as a one byte unsigned int.
 * Treat as little endian, missing bases treated as 0.
 * Finally, subtract 128 divide by 2**7 to cast into the range [-1, 1]
 */
pub fn build(seq: &BaseSeq, rng: &mut ThreadRng) -> Body7 {
    let mut num = 0;
    for base in seq.iter().take(4).rev() {
        num <<= 2;
        num += *base as u8;
    }
    Body7 {
        position: (num as i32 - 128) as f32 / 128.0,
        learnedResponse: 0.0,
    }
}
