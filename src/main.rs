extern crate rand;

mod e0;
mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod e8;
mod e9;
mod evol_prim;
mod sim;

pub use evol_prim::Base::*;
pub use evol_prim::*;
use rand::Rng;
use sim::Simulation;

fn main() {
    // let s = vec![T, T, C, T];
    let s = vec![A, T, C, T];
    let b = read4_bases_to_unsigned_byte(&mut s.iter());
    let f = byte_to_feature_space(b);
    println!("b: {}, f: {}", b, f);

    let mut rng = rand::thread_rng();

    let mut population = Vec::new();
    for _ in 0..100 {
        let seq = (0..8).map(|_| rng.gen::<Base>()).collect::<Vec<Base>>();
        let body = e9::build(&seq, &mut rng); // byteToFeatureSpace(38) = 0.3; byteToFeatureSpace(26) = 0.2
        population.push(Organism { genes: seq, body });
    }

    let mut sim = Simulation {
        R: &e9::reproduce,
        D: &e9::death,
        B: &e9::build,
        U: &e9::update,
        organisms: population,
        environment: e9::Environment9 {
            safe_zone_low: 0.6,
            safe_zone_high: 0.8,
        },
        max_sequences: 400,
        t: 0,
        max_t: 300,
        rng,
    };

    while sim.t < sim.max_t {
        // sim.organisms = sim
        //     .organisms
        //     .into_iter()
        //     .map(|o| {
        //         let body = e8::build(&o.genes, &mut sim.rng);
        //         Organism {
        //             genes: o.genes,
        //             body,
        //         }
        //     })
        //     .collect();

        if sim.t % 1 == 0 {
            //println!("{:?}", sim.E);
            println!("Population size: {}", sim.organisms.len());
            let fit = sim
                .organisms
                .iter()
                .filter(|o| {
                    in_zone_possibly_wrapped(
                        o.body.position,
                        sim.environment.safe_zone_low,
                        sim.environment.safe_zone_high,
                    )
                })
                .count();

            let avg_pos = sim.organisms.iter().map(|o| o.body.position).sum::<f32>()
                / sim.organisms.len() as f32;

            let avg_response_product = sim
                .organisms
                .iter()
                .map(|o| o.body.stimulus_response_vector.iter().map(|e| *e).reduce(|p, e| p * e).unwrap_or(0.0))
                .sum::<f32>()
                // .filter(|r| (r - 0.3).abs() < 0.1)
                // .count() as f32
                / sim.organisms.len() as f32;
            let stdev_response_product = sim
                .organisms
                .iter()
                .map(|o| {
                    o.body
                        .stimulus_response_vector
                        .iter()
                        .map(|e| *e)
                        .reduce(|p, e| p * e)
                        .unwrap_or(0.0)
                })
                .map(|l| (l - avg_response_product).powi(2))
                .sum::<f32>()
                .sqrt()
                / sim.organisms.len() as f32;

            println!(
                "avg pos {}, avg learning {}, stdev learning {}, in safe zone {}, SZ [{},{}]",
                avg_pos,
                avg_response_product,
                stdev_response_product,
                fit,
                sim.environment.safe_zone_low,
                sim.environment.safe_zone_high
            );
        }
        sim.run_step();
    }

    println!("{:?}", sim.organisms);
}
