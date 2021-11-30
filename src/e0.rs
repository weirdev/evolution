use rand::Rng;
use rand::prelude::ThreadRng;

use crate::evol_prim::*;
use crate::evol_prim::BaseSeq;
use crate::evol_prim::Base::*;

const E1_REPRODUCE_PREFIX: &'static [Base] = &[A, T, A, T];

pub fn reproduce(s: &BaseSeq, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    if s.starts_with(&E1_REPRODUCE_PREFIX) {
        panic!() // Stop simulation when true reproduction starts
    } else if rng.gen_ratio(1, 2) {
        return vec![clone_with_mutation(s, rng, 0.1, 0.1, 0.33), clone_with_mutation(s, rng, 0.1, 0.1, 0.33)]
        // TODO: For E0 (primordial soup), we should really just be injecting random sequences into the primordial soup, not cloning existing sequences with modification
    }
    Vec::new()
}

pub fn death(s: &BaseSeq, rng: &mut ThreadRng) -> bool {
    s.len() == 0 || rng.gen::<f32>() < 0.5
}