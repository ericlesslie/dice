use rand::prelude::*;
use std::time::Instant;
use std::cell::Cell;

#[derive(Clone, Copy, PartialEq)]
pub enum DieKind {
    Four,
    Six,
    Eight,
    Ten,
    Twelve,
    Twenty
}

#[derive(Clone)]
pub struct Die {
  pub time: Cell<Option<Instant>>,
  pub kind: DieKind,
  pub val: Cell<u32>,
  pub spin_seed: Cell<[u32; 3]>,
}

impl Die {
    pub fn new(kind: DieKind) -> Self {
        let mut rng = thread_rng();
        let val = Self::generate_roll(kind);
        let spin_seed = [
            rng.gen_range(2..=5),
            rng.gen_range(2..=5),
            rng.gen_range(2..=5),
        ];
        Self {
            time: Cell::new(Some(Instant::now())),
            kind,
            val: Cell::new(val),
            spin_seed: Cell::new(spin_seed),
        }
    }

    pub fn roll(&self) {
        let mut rng = thread_rng();
        self.time.set(Some(Instant::now()));
        self.val.set(Self::generate_roll(self.kind));
        self.spin_seed.set([
            rng.gen_range(2..=5),
            rng.gen_range(2..=5),
            rng.gen_range(2..=5),
        ]);
    }

    fn generate_roll(kind: DieKind) -> u32 {
        let mut rng = thread_rng();

        match kind {
            DieKind::Four => rng.gen_range(1..=4),
            DieKind::Six => rng.gen_range(1..=6),
            DieKind::Eight => rng.gen_range(1..=8),
            DieKind::Ten => rng.gen_range(1..=10),
            DieKind::Twelve => rng.gen_range(1..=12),
            DieKind::Twenty => rng.gen_range(1..=20),
        }
    }
}


