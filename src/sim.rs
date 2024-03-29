use rand::prelude::{SliceRandom, ThreadRng};

use crate::evol_prim::*;

pub struct Simulation<'a, O, E> {
    // Reproduce
    pub R: &'a dyn Fn(&Organism<O>, &E, &mut ThreadRng) -> Vec<BaseSeq>,
    // Die
    pub D: &'a dyn Fn(&Organism<O>, &E, &mut ThreadRng) -> bool,
    // Build body from genetic seq
    pub B: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> O,
    // Update organism with a single time step
    pub U: &'a dyn Fn(&mut Organism<O>, &E, &mut ThreadRng),
    // Learn
    pub L: &'a dyn Fn(&mut Organism<O>, &E, &mut ThreadRng),
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
        let mut all_children = Vec::new();
        while let Some(org) = self.organisms.pop() {
            // Die?
            if !(self.D)(&org, &self.environment, &mut self.rng) {
                // Reproduce
                let babies = (self.R)(&org, &self.environment, &mut self.rng)
                    .into_iter()
                    .filter(|s| s.len() > 0)
                    .map(|s| {
                        let child_body = (self.B)(&s, &mut self.rng);
                        Organism {
                            genes: s,
                            body: child_body,
                        }
                    });
                all_children.extend(babies);

                // Didn't die so add self
                new_organisms.push(org);
            }
        }

        // Limit addition of children so that we don't sample between chidren and parents below
        // NOTE: Using the sampling below produces extreme genetic swings
        if new_organisms.len() < self.max_sequences {
            let current_size = new_organisms.len();
            new_organisms.extend(&mut all_children.into_iter().take(self.max_sequences - current_size));
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
        self.environment.update(&mut self.rng);

        for org in &mut self.organisms {
            // Learn
            (self.L)(org, &self.environment, &mut self.rng);
            // Update the state of this organism
            (self.U)(org, &self.environment, &mut self.rng);
        }

        self.t += 1;
    }
}

impl <'a, O, E> Clone for Simulation<'a, O, E> where E: Clone, O: Clone {
    fn clone(&self) -> Self {
        Simulation {
            R: self.R,
            D: self.D,
            B: self.B,
            U: self.U,
            L: self.L,
            organisms: self.organisms.clone(),
            environment: self.environment.clone(),
            max_sequences: self.max_sequences,
            t: self.t,
            max_t: self.max_t,
            rng: self.rng.clone(),
        }
    }
}
