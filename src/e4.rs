// E4: The more prefix repetitions of AT, the faster reproduction; C's confer mutation protection

use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::Base::*;
use crate::evol_prim::BaseSeq;
use crate::evol_prim::*;

pub fn reproduce(s: &BaseSeq, rng: &mut ThreadRng) -> Vec<BaseSeq> {
    let at_reps = count_AT_repetitions(s);
    match at_reps {
        0 | 1 => Vec::new(),
        _ => {
            let c_count = count_C(s);
            let c = if c_count == 0 { 1.0 } else { c_count as f32 };
            (0..at_reps)
                .map(|_| clone_with_mutation(s, rng, 0.01 / c, 0.01 / c, 0.05 / c))
                .collect()
        }
    }
}

pub fn death(s: &BaseSeq, rng: &mut ThreadRng) -> bool {
    s.len() == 0 || rng.gen::<f32>() < 0.5
}

pub fn count_AT_repetitions(s: &BaseSeq) -> usize {
    let mut at_reps = 0;
    while (at_reps * 2) + 1 < s.len() && s[at_reps * 2] == A && s[(at_reps * 2) + 1] == T {
        at_reps += 1;
    }
    at_reps
}

pub fn count_C(s: &BaseSeq) -> usize {
    s.iter().filter(|b| **b == C).count()
}
