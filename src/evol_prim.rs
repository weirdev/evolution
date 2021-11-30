use rand::{Rng, prelude::ThreadRng};
use rand::distributions::{Distribution, Standard};

// Evolution Primitives
pub type BaseSeq = Vec<Base>;

use Base::*;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Base {
    A,
    C,
    T,
    G
}

impl Distribution<Base> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Base {
        match rng.gen_range(0..4) {
            0 => A,
            1 => C,
            2 => T,
            _ => G
        }
    }
}

pub fn clone_with_mutation(seq: &BaseSeq, rng: &mut ThreadRng, 
    insertion_prob: f32, deletion_prob: f32, base_change_prob: f32) -> BaseSeq {
        let mut new = BaseSeq::new();
        for b in seq {
            if rng.gen::<f32>() < insertion_prob {
                new.push(rng.gen());
            }
            
            if rng.gen::<f32>() > deletion_prob { // Else skip
                let next = if rng.gen::<f32>() < base_change_prob {
                    rng.gen()
                } else {
                    *b
                };
                new.push(next);
            }
        }
        if rng.gen::<f32>() < insertion_prob {
            new.push(rng.gen());
        }

        new
}