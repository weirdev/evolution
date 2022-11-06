extern crate rand;

mod e0;
mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod evol_prim;
mod sim;

pub use evol_prim::Base::*;
pub use evol_prim::*;
use rand::Rng;
use sim::Simulation;

fn main() {
    let mut rng = rand::thread_rng();

    let mut population = Vec::new();
    for _ in 0..100 {
        let seq = (0..4).map(|_| rng.gen::<Base>()).collect();
        let body = e7::build(&seq, &mut rng);
        population.push(Organism { genes: seq, body });
    }

    let mut sim = Simulation {
        R: &e7::reproduce,
        D: &e7::death,
        B: &e7::build,
        U: &(|_, _| {}),
        organisms: population,
        environment: e7::Environment7 {
            safeZoneLow: -0.2,
            safeZoneHigh: 0.0,
        },
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
                let fit = sim
                    .organisms
                    .iter()
                    .filter(|o| {
                        o.body.position >= sim.environment.safeZoneLow
                            && o.body.position <= sim.environment.safeZoneHigh
                    })
                    .count();
                println!("{} in safe zone", fit);
            }
        }
        sim.run_step();
    }

    println!("{:?}", sim.organisms);
}
