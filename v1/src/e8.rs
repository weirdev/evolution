use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body8 {
    pub position: f32,                 // [-1,1]
    pub stimulus_response_factor: f32, // [-1,1]
    pub track: bool,
}

pub struct Environment8 {
    pub safe_zone_low: f32,  // [-1,1]
    pub safe_zone_high: f32, // [-1, 1] > safeZoneLow
}

impl Environment for Environment8 {
    fn update(&mut self, rng: &mut ThreadRng) {
        self.safe_zone_low = rng.gen::<f32>() * 0.8;
        self.safe_zone_high = self.safe_zone_low + 0.2;
    }
}

pub fn death(org: &Organism<Body8>, env: &Environment8, rng: &mut ThreadRng) -> bool {
    (!in_zone_possibly_wrapped(org.body.position, env.safe_zone_low, env.safe_zone_high)
        && rng.gen::<f32>() < 0.6)
        ^ (rng.gen::<f32>() < 0.001)
}

pub fn reproduce(org: &Organism<Body8>, env: &Environment8, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    if in_zone_possibly_wrapped(org.body.position, env.safe_zone_low, env.safe_zone_high) {
        // One child
        (0..2)
            .map(|_| clone_with_mutation(&org.genes, rng, 0.0, 0.0, 0.06))
            .collect()
    } else {
        Vec::new()
    }
}

pub fn update(org: &mut Organism<Body8>, env: &Environment8, rng: &mut ThreadRng) {
    // Move in response to being in danger zone.
    // Move to exactly where stimulus * stimulus_reception_factor * stimulus factor indicates

    // stimulus = the actual real world event = the position of the middle of the safe zone
    // stimulus_reception_factor = the first order (unlearned) perception of the stimulus
    // In the future, instead of a dumb factor stimulus_reception_factor should be a function
    // that generates a basic input of the real world as the first input into the organisms
    // learning system. Similar to the conceptless electrical impulses provided by the eyes
    // to the brain.
    // As a dumb factor, stimulus_reception_factor essentially functions as the error we need
    // to learn to correct for with stimulus_response_factor.
    // stimulus_response_factor = the learned transformation from the received stimulus to
    // a response.
    // In the future, instead of a dumb factor stimulus_response_factor should be a full neural
    // architecture producing the response from received stimuli.

    let stimulus = env.safe_zone_low + ((env.safe_zone_high - env.safe_zone_low) / 2.0);
    let stimulus_reception_factor = 4.0;

    org.body.position = stimulus * stimulus_reception_factor * org.body.stimulus_response_factor
}

/**
 * Each base conveys 2 bits of information.
 * Take the first 4 bases as a one byte unsigned int.
 * Treat as little endian, missing bases treated as 0.
 * Finally, subtract 128 divide by 2**7 to cast into the range [-1, 1]
 */
pub fn build(seq: &BaseSeq, rng: &mut ThreadRng) -> Body8 {
    let mut si = seq.iter().peekable();

    let mut learn_raw = 0;
    if let Some(_) = si.peek() {
        learn_raw = read4_bases_to_unsigned_byte(&mut si);
    }

    // Cast into [-1, 1]
    Body8 {
        position: 0.0,
        stimulus_response_factor: byte_to_feature_space(learn_raw),
        track: false,
    }
}
