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
    // let s = vec![T, T, C, T];
    let s = vec![A, T, C, T];
    let b = read4BasesToUnsignedByte(&mut s.iter());
    let f = byteToFeatureSpace(b);
    println!("b: {}, f: {}", b, f);

    let mut rng = rand::thread_rng();

    let mut population = Vec::new();
    for _ in 0..100 {
        let mut seq = (0..4).map(|_| rng.gen::<Base>()).collect::<Vec<Base>>(); // T T C T
        seq.append(&mut vec![A, T, C, T]);
        let body = e7::build(&seq, &mut rng); // 166 = 0.3
        population.push(Organism { genes: seq, body });
    }

    let mut sim = Simulation {
        R: &e7::reproduce,
        D: &e7::death,
        B: &e7::build,
        U: &e7::update,
        organisms: population,
        environment: e7::Environment7 {
            safe_zone_low: -0.1,
            safe_zone_high: 0.0,
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
            let fit = sim
                .organisms
                .iter()
                .filter(|o| {
                    o.body.position >= sim.environment.safe_zone_low
                        && o.body.position <= sim.environment.safe_zone_high
                })
                .count();

            let avg_pos = sim.organisms.iter().map(|o| o.body.position).sum::<f32>()
                / sim.organisms.len() as f32;

            let avg_learning = sim
                .organisms
                .iter()
                .map(|o| o.body.learned_response)
                .sum::<f32>()
                // .filter(|r| (r - 0.3).abs() < 0.1)
                // .count() as f32
                / sim.organisms.len() as f32;

            println!(
                "avg pos {}, avg learning {}, in safe zone {}, SZ {}:{}",
                avg_pos,
                avg_learning,
                fit,
                sim.environment.safe_zone_low,
                sim.environment.safe_zone_high
            );
        }
        sim.run_step();
    }

    println!("{:?}", sim.organisms);
}
