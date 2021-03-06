// E1: Replication if prefix matches

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::evol_prim::*;
use crate::evol_prim::BaseSeq;
use crate::evol_prim::Base::*;

pub const E1_REPRODUCE_PREFIX: &'static [Base] = &[A, T, A, T];

pub fn reproduce(s: &BaseSeq, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    if s.starts_with(&E1_REPRODUCE_PREFIX) {
        return vec![clone_with_mutation(s, rng, 0.01, 0.01, 0.05), clone_with_mutation(s, rng, 0.01, 0.01, 0.05)]
    }
    Vec::new()
}

pub fn death(s: &BaseSeq, rng: &mut ThreadRng) -> bool {
    s.len() == 0 || rng.gen::<f32>() < 0.5
}