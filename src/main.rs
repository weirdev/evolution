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
mod e10;
mod evol_prim;
mod sim;
mod vis;

use evol_prim::Base::*;
use evol_prim::*;
use rand::Rng;
use sim::Simulation;
use vis::create_image;

fn main() {
    create_image();
    return;
    // let s = vec![T, T, C, T];
    let s = vec![A, T, C, T];
    let b = read4_bases_to_unsigned_byte(&mut s.iter());
    let f = byte_to_feature_space(b);
    println!("b: {}, f: {}", b, f);

    let mut rng = rand::thread_rng();

    let mut population = Vec::new();
    for _ in 0..100 {
        let seq = (0..8).map(|_| rng.gen::<Base>()).collect::<Vec<Base>>();
        let body = e10::build(&seq, &mut rng); // byteToFeatureSpace(38) = 0.3; byteToFeatureSpace(26) = 0.2
        population.push(Organism { genes: seq, body });
    }

    let mut sim = Simulation {
        R: &e10::reproduce,
        D: &e10::death,
        B: &e10::build,
        U: &e10::update,
        L: &e10::learn,
        organisms: population,
        environment: e10::Environment10 {
            safe_zone_low: 0.6,
            safe_zone_high: 0.8,
        },
        max_sequences: 400,
        t: 0,
        max_t: 300,
        rng,
    };

    let mut last_5_fit_sum = 0;
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

            if sim.t + 5 >= sim.max_t {
                last_5_fit_sum += fit;
            }

            let avg_pos = sim.organisms.iter().map(|o| o.body.position).sum::<f32>()
                / sim.organisms.len() as f32;

            let avg_response_sum = sim
                .organisms
                .iter()
                .map(|o| o.body.stimulus_response_vector.iter().map(|e| *e).reduce(|p, e| p + e).unwrap_or(0.0))
                .sum::<f32>()
                // .filter(|r| (r - 0.3).abs() < 0.1)
                // .count() as f32
                / sim.organisms.len() as f32;
            let stdev_response_sum = sim
                .organisms
                .iter()
                .map(|o| {
                    o.body
                        .stimulus_response_vector
                        .iter()
                        .map(|e| *e)
                        .reduce(|p, e| p + e)
                        .unwrap_or(0.0)
                })
                .map(|l| (l - avg_response_sum).powi(2))
                .sum::<f32>()
                .sqrt()
                / sim.organisms.len() as f32;

            let avg_response_vec = sim
                .organisms
                .iter()
                .map(|o| o.body.stimulus_response_vector)
                .fold([0.0, 0.0], |a, b| [a[0] + (b[0] / sim.organisms.len() as f32), a[1] + (b[1] / sim.organisms.len() as f32)]);
                // .filter(|r| (r - 0.3).abs() < 0.1)
                // .count() as f32

            let avg_learning_factor = sim
                .organisms
                .iter()
                .map(|o| o.body.learning_factor)
                .sum::<f32>()
                // .filter(|r| (r - 0.3).abs() < 0.1)
                // .count() as f32
                / sim.organisms.len() as f32;

            println!(
                "avg pos {}, avg learning {}, avg response vec [{},{}], avg response vec sum {}, stdev response vec sum {}, in safe zone {}, SZ [{},{}]",
                avg_pos,
                avg_learning_factor,
                avg_response_vec[0],
                avg_response_vec[1],
                avg_response_sum,
                stdev_response_sum,
                fit,
                sim.environment.safe_zone_low,
                sim.environment.safe_zone_high
            );
        }
        sim.run_step();
    }

    println!("Last 5 fit sum: {}", last_5_fit_sum);

    // println!("{:?}", sim.organisms);
}
