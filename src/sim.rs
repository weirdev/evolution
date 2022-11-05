use rand::prelude::{SliceRandom, ThreadRng};

use crate::evol_prim::*;

pub struct Simulation<'a, O> {
    // Reproduce
    pub R: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> Vec<BaseSeq>,
    // Die
    pub D: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> bool,
    // Build body from genetic seq
    pub B: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> O,
    // All organisms in simulation
    pub organisms: Vec<Organism<O>>,
    pub max_sequences: usize,
    // Current time step
    pub t: i32,
    pub max_t: i32,
    pub rng: ThreadRng,
}

impl<'a, O: std::fmt::Debug + Clone> Simulation<'a, O> {
    pub fn run(&mut self, print_freq: Option<u32>) {
        while self.t < self.max_t {
            self.run_step();
            if print_freq.map_or(false, |f| self.t % f as i32 == 0) {
                println!("{:?}", (&self.organisms).into_iter().map(|o| &o.genes))
            }
        }
    }

    pub fn run_step(&mut self) {
        let mut i: isize = 0;
        let len = self.organisms.len();
        let mut new_organisms = Vec::new();
        while (i as usize) < len {
            let org = &self.organisms[i as usize];
            let babies: Vec<Organism<O>> = (self.R)(&org.genes, &mut self.rng)
                .into_iter()
                .filter(|s| s.len() > 0)
                .map(|s| {
                    let child_body = (self.B)(&s, &mut self.rng);
                    Organism {
                        genes: s,
                        body: child_body,
                    }
                })
                .collect();
            if babies.len() > 0 {
                new_organisms.extend(babies.into_iter());
            } else if !(self.D)(&org.genes, &mut self.rng) {
                new_organisms.push(org.clone());
            }

            i += 1;
        }

        self.organisms.clear();
        if new_organisms.len() > self.max_sequences {
            new_organisms
                .choose_multiple(&mut self.rng, self.max_sequences)
                .for_each(|s| self.organisms.push(s.clone()));
        } else {
            self.organisms.append(&mut new_organisms);
        }

        self.t += 1;
    }
}

pub trait Reproduce {
    fn reproduce(&self, s: BaseSeq);
}
