use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body7 {
    pub position: f32,         // [-1,1]
    pub learned_response: f32, // [-1,1]
    pub track: bool,
}

pub struct Environment7 {
    pub safe_zone_low: f32,  // [-1,1]
    pub safe_zone_high: f32, // [-1, 1] > safeZoneLow
}

impl Environment for Environment7 {
    fn update(&mut self, rng: &mut ThreadRng) {
        let move_on_tick = 0.3;
        self.safe_zone_low = wrapping_feature_add(self.safe_zone_low, move_on_tick);
        self.safe_zone_high = wrapping_feature_add(self.safe_zone_high, move_on_tick);
    }
}

pub fn inDangerZone(org: &Organism<Body7>, env: &Environment7) -> bool {
    !in_zone_possibly_wrapped(org.body.position, env.safe_zone_low, env.safe_zone_high)
}

pub fn death(org: &Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) -> bool {
    (inDangerZone(org, env) && rng.gen::<f32>() < 0.5) ^ (rng.gen::<f32>() < 0.001)
}

pub fn reproduce(org: &Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    if !inDangerZone(org, env) {
        // One child
        (0..2)
            .map(|_| clone_with_mutation(&org.genes, rng, 0.0, 0.0, 0.06))
            .collect()
    } else {
        Vec::new()
    }
}

pub fn update(org: &mut Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) {
    // Move in response to being in danger zone.
    // Granted perfect perception of danger
    // No relative perception of current position for now
    // let pos_change = if inDangerZone(org, env) {
    //     rng.gen() // Do something random (no learning feedback if in DZ)
    // } else {
    //     org.body.learned_response // If safe, do learned behavior
    // };

    // Mario style wraparound
    org.body.position = wrapping_feature_add(org.body.position, org.body.learned_response);
}

/**
 * Each base conveys 2 bits of information.
 * Take the first 4 bases as a one byte unsigned int.
 * Treat as little endian, missing bases treated as 0.
 * Finally, subtract 128 divide by 2**7 to cast into the range [-1, 1]
 * Similarly for the next 4 bases.
 */
pub fn build(seq: &BaseSeq, rng: &mut ThreadRng) -> Body7 {
    let mut si = seq.iter().peekable();

    let pos_raw = read4_bases_to_unsigned_byte(&mut si);

    let mut learn_raw = 0;
    if let Some(_) = si.peek() {
        learn_raw = read4_bases_to_unsigned_byte(&mut si);
    }

    // Cast into [-1, 1]
    Body7 {
        position: byte_to_feature_space(pos_raw),
        learned_response: byte_to_feature_space(learn_raw),
        track: false,
    }
}
