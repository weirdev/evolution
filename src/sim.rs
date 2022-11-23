use rand::prelude::{SliceRandom, ThreadRng};

use crate::evol_prim::*;

pub struct Simulation<'a, O, E> {
    // Reproduce
    pub R: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> Vec<BaseSeq>,
    // Die
    pub D: &'a dyn Fn(&Organism<O>, &E, &mut ThreadRng) -> bool,
    // Build body from genetic seq
    pub B: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> O,
    // Update organism with a single time step
    pub U: &'a dyn Fn(&mut Organism<O>, &E, &mut ThreadRng),
    // All organisms in simulation
    pub organisms: Vec<Organism<O>>,
    pub environment: E,
    pub max_sequences: usize,
    // Current time step
    pub t: i32,
    pub max_t: i32,
    pub rng: ThreadRng,
}

impl<'a, O: std::fmt::Debug + Clone, E: Environment> Simulation<'a, O, E> {
    pub fn run(&mut self, print_freq: Option<u32>) {
        while self.t < self.max_t {
            self.run_step();
            if print_freq.map_or(false, |f| self.t % f as i32 == 0) {
                println!("{:?}", (&self.organisms).into_iter().map(|o| &o.genes))
            }
        }
    }

    pub fn run_step(&mut self) {
        let mut new_organisms = Vec::new();
        while let Some(org) = self.organisms.pop() {
            // Die?
            if !(self.D)(&org, &self.environment, &mut self.rng) {
                // Reproduce
                let babies = (self.R)(&org.genes, &mut self.rng)
                    .into_iter()
                    .filter(|s| s.len() > 0)
                    .map(|s| {
                        let child_body = (self.B)(&s, &mut self.rng);
                        Organism {
                            genes: s,
                            body: child_body,
                        }
                    });
                new_organisms.extend(babies);

                // Didn't die so add self
                new_organisms.push(org);
            }
        }

        //self.organisms.clear(); // Should already be empty
        if new_organisms.len() > self.max_sequences {
            // TODO: Do this without copying
            new_organisms
                .choose_multiple(&mut self.rng, self.max_sequences)
                .for_each(|o| self.organisms.push(o.clone()));
        } else {
            self.organisms.append(&mut new_organisms);
        }

        // Update env and orgs for next cycle
        
        // Env must tick before org updates, otherwise organisms always appear out phase with env after each step
        self.environment.update();

        for org in &mut self.organisms {
            // Update the state of this organism
            (self.U)(org, &self.environment, &mut self.rng);
        }

        self.t += 1;
    }
}

pub trait Reproduce {
    fn reproduce(&self, s: BaseSeq);
}
