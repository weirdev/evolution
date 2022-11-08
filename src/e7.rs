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

fn inDangerZone(org: &Organism<Body7>, env: &Environment7) -> bool {
    org.body.position < env.safeZoneLow || org.body.position > env.safeZoneHigh
}

pub fn death(org: &Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) -> bool {
    inDangerZone(org, env) ^ (rng.gen::<f32>() < 0.2)
}

pub fn reproduce(s: &BaseSeq, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    // One child
    (0..1)
        .map(|_| clone_with_mutation(s, rng, 0.01, 0.01, 0.05))
        .collect()
}

pub fn update(org: &mut Organism<Body7>, env: &Environment7, rng: &mut ThreadRng) {
    // Move in response to being in danger zone.
    // No perception of current position for now
    if inDangerZone(org, env) {
        // Update position with learned response
        let mut new_pos = org.body.position + (org.body.learnedResponse * rng.gen::<f32>());
        // Mario style wraparound
        new_pos = ((new_pos + 3.0) % 2.0) - 1.0;

        org.body.position = new_pos;
    }
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

    let pos_raw = read4BasesToUnsignedByte(&mut si);

    let mut learn_raw = 0;
    if let Some(_) = si.peek() {
        learn_raw = read4BasesToUnsignedByte(&mut si);
    }

    // Cast into [-1, 1]
    Body7 {
        position: byteToFeatureSpace(pos_raw),
        learnedResponse: byteToFeatureSpace(learn_raw),
    }
}
