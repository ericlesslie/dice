use rand::prelude::*;
use std::time::{Duration, Instant};

pub enum DieKind {
    Four,
    Six,
    Ten,
    Twelve,
    Twenty
}

#[derive(Clone)]
pub struct Die {
  pub time: Option<Instant>,
  pub kind: DieKind,
  pub val: u32,
}

impl Die {
    pub fn new(kind: DieKind) {
        let val = generate_roll(kind);
        Self { time: Some(Instant::now()), kind, val }
    }

    pub fn roll(&self) {
        self.time = Some(Instant::now());
        self.val = generate_roll(self.kind);
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


