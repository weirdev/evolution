// Evolution Primitives

use rand::distributions::{Distribution, Standard};
use rand::{prelude::ThreadRng, Rng};

pub type BaseSeq = Vec<Base>;

use Base::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Base {
    A,
    C,
    T,
    G,
}

impl Distribution<Base> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Base {
        match rng.gen_range(0..4) {
            0 => A,
            1 => C,
            2 => T,
            _ => G,
        }
    }
}

pub fn clone_with_mutation(
    seq: &BaseSeq,
    rng: &mut ThreadRng,
    insertion_prob: f32,
    deletion_prob: f32,
    base_change_prob: f32,
) -> BaseSeq {
    let mut new = BaseSeq::new();
    for b in seq {
        if rng.gen::<f32>() < insertion_prob {
            new.push(rng.gen());
        }

        if rng.gen::<f32>() > deletion_prob {
            // Else skip
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

#[derive(Debug, Clone)]
pub struct Organism<O> {
    pub genes: BaseSeq,
    pub body: O,
}

pub fn read4BasesToUnsignedByte(bases: &mut dyn Iterator<Item = &Base>) -> u8 {
    let mut byte: u8 = 0;
    for _ in 0..4 {
        byte <<= 2;
        byte += bases.next().map(|b| *b as u8).unwrap_or(0);
    }
    byte
}

pub fn byteToFeatureSpace(byte: u8) -> f32 {
    (byte.wrapping_add(128) as i32 - 128) as f32 / 128.0
}
