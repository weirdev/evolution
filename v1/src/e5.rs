use rand::prelude::ThreadRng;
use rand::Rng;

use crate::evol_prim::*;

#[derive(Debug, Clone)]
pub struct Body5 {
    pub age: u32,
}

pub fn death(org: &Organism<Body5>, rng: &mut ThreadRng) -> bool {
    org.body.age > 2 && rng.gen::<f32>() < 0.5
}

pub fn update(org: &mut Organism<Body5>, rng: &mut ThreadRng) {
    org.body.age += 1;
}
