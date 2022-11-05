extern crate rand;

mod e0;
mod e1;
mod e2;
mod e3;
mod e4;
mod evol_prim;
mod sim;

pub use evol_prim::Base::*;
pub use evol_prim::*;
use sim::Simulation;

fn main() {
    let bs = vec![A, T, A, T, T];
    let bs2 = vec![A, C, G, T, T];

    let mut population = Vec::new();
    for _ in 0..100 {
        population.push(Organism {
            genes: bs.clone(),
            body: (),
        });
        population.push(Organism {
            genes: bs2.clone(),
            body: (),
        });
    }

    let rng = rand::thread_rng();

    let mut sim = Simulation {
        R: &e4::reproduce,
        D: &(|o, r| e4::death(&o.genes, r)),
        B: &(|_, _| ()),
        organisms: population,
        max_sequences: 400,
        t: 0,
        max_t: 300,
        rng,
    };

    while sim.t < sim.max_t {
        if sim.t % 1 == 0 {
            //println!("{:?}", sim.E);
            println!("Population size: {}", sim.organisms.len());
            if sim.organisms.len() > 0 {
                let fit = (&sim.organisms)
                    .into_iter()
                    .fold(0, |b, o| b + e4::count_AT_repetitions(&o.genes));
                println!(
                    "Population reproductive fitness = {}",
                    fit as f32 / sim.organisms.len() as f32
                );
                let fit2 = (&sim.organisms)
                    .into_iter()
                    .fold(0, |b, o| b + e4::count_C(&o.genes));
                println!(
                    "Population mutation protection fitness = {}",
                    fit2 as f32 / sim.organisms.len() as f32
                );
            }
        }
        sim.run_step();
    }

    println!("{:?}", sim.organisms);
}
