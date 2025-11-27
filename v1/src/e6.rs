use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body6 {
    pub weight: f32, // [0,1]
}

pub fn death(org: &Organism<Body6>, rng: &mut ThreadRng) -> bool {
    org.body.weight > 0.1 && org.body.weight < 0.9 && rng.gen::<f32>() < 0.9
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
 * Finally divide by 2**8 to cast into the range [0, 1]
 */
pub fn build(seq: &BaseSeq, rng: &mut ThreadRng) -> Body6 {
    let mut num = 0;
    for base in seq.iter().take(4).rev() {
        num <<= 2;
        num += *base as u8;
    }
    Body6 {
        weight: num as f32 / 256.0,
    }
}
