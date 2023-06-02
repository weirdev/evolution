use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body10 {
    pub position: f32, // [-1,1]
    // First component is selected, second is learned
    pub stimulus_response_vector: [f32; 2], // [-1,1]
    pub learning_factor: f32,               // [0,1]
    pub track: bool,
}

pub struct Environment10 {
    pub safe_zone_low: f32,  // [-1,1]
    pub safe_zone_high: f32, // [-1, 1] > safeZoneLow
}

impl Environment for Environment10 {
    fn update(&mut self, rng: &mut ThreadRng) {
        self.safe_zone_low = rng.gen::<f32>() * 0.8;
        self.safe_zone_high = self.safe_zone_low + 0.2;
    }
}

pub fn death(org: &Organism<Body10>, env: &Environment10, rng: &mut ThreadRng) -> bool {
    (!in_zone_possibly_wrapped(org.body.position, env.safe_zone_low, env.safe_zone_high)
        && rng.gen::<f32>() < 0.4)
        ^ (rng.gen::<f32>() < 0.001)
}

pub fn reproduce(org: &Organism<Body10>, env: &Environment10, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    if in_zone_possibly_wrapped(org.body.position, env.safe_zone_low, env.safe_zone_high) {
        // One child
        (0..1)
            .map(|_| clone_with_mutation(&org.genes, rng, 0.0, 0.0, 0.06))
            .collect()
    } else {
        Vec::new()
    }
}

fn stimulus_response_circuit(
    org: &Organism<Body10>,
    env: &Environment10,
    rng: &mut ThreadRng,
) -> f32 {
    // stimulus = the actual real world event = the position of the middle of the safe zone
    // stimulus_reception = the first order (unlearned) perception of the stimulus

    // A function that generates a basic input of the real world as the first input into the
    // organisms learning system. Similar to the conceptless electrical impulses provided by
    // the eyes to the brain.

    // As a dumb factor, stimulus_reception_factor essentially functions as the error we need
    // to learn to correct for with stimulus_response_factor.
    // stimulus_response_factor = the learned transformation from the received stimulus to
    // a response.
    // In the future, instead of a dumb factor stimulus_response_factor should be a full neural
    // architecture producing the response from received stimuli.

    let stimulus = env.safe_zone_low + ((env.safe_zone_high - env.safe_zone_low) / 2.0);
    let reception = stimulus_reception(stimulus);

    stimulus_response(
        reception,
        &mut org.body.stimulus_response_vector.iter(),
        rng,
    )
}

pub fn update(org: &mut Organism<Body10>, env: &Environment10, rng: &mut ThreadRng) {
    // Move in response to being in danger zone.

    org.body.position = stimulus_response_circuit(org, env, rng);
}

fn stimulus_reception(stimulus: f32) -> f32 {
    stimulus * 3.0 // Second order
                   // stimulus * 4.0 // first order
}

fn stimulus_response(
    reception: f32,
    stimulus_response_vector: &mut dyn Iterator<Item = &f32>,
    _: &mut ThreadRng,
) -> f32 {
    let layer1 = stimulus_response_vector.map(|e| reception * e);

    layer1.reduce(|p, e| p + e).unwrap_or(0.0)
}

pub fn learn(org: &mut Organism<Body10>, env: &Environment10, rng: &mut ThreadRng) {
    // Will be its own neural net or some other logic under selection
    // For now, test with a perfectly accurate hardcoded solution
    // TODO: learn function should not have direct access to the current values
    // of the sim response circuit

    let debug = rng.gen::<f32>() < 0.001;

    let target_pos = env.safe_zone_low + ((env.safe_zone_high - env.safe_zone_low) / 2.0);
    let current_learned_pos = stimulus_response_circuit(org, env, rng);
    let loss = target_pos - current_learned_pos;

    if debug {
        println!(
            "target_pos: {}, current_learned_pos: {}, loss: {}, learning_factor: {}, updated_srv1: {}, updated_srv1_if_perfect_lf: {}",
            target_pos, current_learned_pos, loss, org.body.learning_factor, org.body.stimulus_response_vector[1] + loss * org.body.learning_factor, org.body.stimulus_response_vector[1] + loss * 0.3333
        );
    }

    // IP
    org.body.stimulus_response_vector[1] += loss * org.body.learning_factor;

    // org.body.stimulus_response_vector[1] =
    //     (1.0 - (3.0 * org.body.stimulus_response_vector[0])) / 3.0
}

/**
 * Each base conveys 2 bits of information.
 * Take the first 4 bases as a one byte unsigned int.
 * Treat as little endian, missing bases treated as 0.
 * Finally, subtract 128 divide by 2**7 to cast into the range [-1, 1]
 */
pub fn build(seq: &BaseSeq, _: &mut ThreadRng) -> Body10 {
    let mut si = seq.iter().peekable();

    let mut response1_raw = 0;
    if let Some(_) = si.peek() {
        response1_raw = read4_bases_to_unsigned_byte(&mut si);
    }

    let mut response2_raw = 0;
    if let Some(_) = si.peek() {
        response2_raw = read4_bases_to_unsigned_byte(&mut si);
    }

    // Cast into [-1, 1]
    Body10 {
        position: 0.0,
        stimulus_response_vector: [byte_to_feature_space(response1_raw), 0.0],
        // stimulus_response_vector: [0.0, 0.0],
        learning_factor: byte_to_feature_space(response2_raw),
        // learning_factor: 0.0,
        track: false,
    }
}
