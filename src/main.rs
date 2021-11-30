extern crate rand;

mod evol_prim;
mod sim;
mod e0;
mod e1;
mod e2;
mod e3;
mod e4;

pub use evol_prim::*;
pub use evol_prim::Base::*;
use sim::Simulation;

fn main() {
    let bs = vec![A, T, A, T, T];
    let bs2 = vec![A, C, G, T, T];

    let mut population = Vec::new();
    for _ in 0..100 {
        population.push(bs.clone());
        population.push(bs2.clone())
    }

    let rng = rand::thread_rng();
  
    let mut sim = Simulation {
        R: &e4::reproduce,
        D: &e4::death,
        sequences: population,
        max_sequences: 400,
        t: 0,
        max_t: 300,
        rng
    };

    while sim.t < sim.max_t {
        if sim.t % 1 == 0 {
            //println!("{:?}", sim.E);
            println!("Population size: {}", sim.sequences.len());
            if sim.sequences.len() > 0 {
                let fit = (&sim.sequences).into_iter()
                        .fold(0, |b, s| b + e4::count_AT_repetitions(s));
                println!("Population reproductive fitness = {}", fit as f32 / sim.sequences.len() as f32);
                let fit2 = (&sim.sequences).into_iter()
                        .fold(0, |b, s| b + e4::count_C(s));
                println!("Population mutation protection fitness = {}", fit2 as f32 / sim.sequences.len() as f32);
            }
        }
        sim.run_step();
    }

    println!("{:?}", sim.sequences);
}
