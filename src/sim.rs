use rand::{prelude::{ThreadRng, SliceRandom}};

use crate::evol_prim::BaseSeq;

pub struct Simulation<'a> {
    pub R: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> Vec<BaseSeq>,
    pub D: &'a dyn Fn(&BaseSeq, &mut ThreadRng) -> bool,
    pub sequences: Vec<BaseSeq>,
    pub max_sequences: usize,
    pub t: i32,
    pub max_t: i32,
    pub rng: ThreadRng
}

impl <'a> Simulation<'a> {
    pub fn run(&mut self, print_freq: Option<u32>) {
        while self.t < self.max_t {
            self.run_step();
            if print_freq.map_or(false, |f| self.t % f as i32 == 0) {
                println!("{:?}", self.sequences)
            }
        }
    }

    pub fn run_step(&mut self) {
        let mut i: isize = 0;
        let len = self.sequences.len();
        let mut new_sequences = Vec::new();
        while (i as usize) < len {
            let seq = &self.sequences[i as usize];
            let babies = (self.R)(seq, &mut self.rng);
            if babies.len() > 0 {
                new_sequences.extend(babies.into_iter()
                        .filter(|s| s.len() > 0));
            } else if !(self.D)(seq, &mut self.rng) {
                new_sequences.push(seq.clone());
            }

            i += 1;
        }

        self.sequences.clear();
        if new_sequences.len() > self.max_sequences {
            new_sequences.choose_multiple(&mut self.rng, self.max_sequences)
                    .for_each(|s| self.sequences.push(s.clone()));
        } else {
            self.sequences.append(&mut new_sequences);
        }

        self.t += 1;
    }
}

pub trait Reproduce {
    fn reproduce(&self, s: BaseSeq);
}