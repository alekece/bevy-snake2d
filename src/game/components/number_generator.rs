use std::ops::Range;

use bevy::prelude::*;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Standard};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Component)]
pub struct NumberGenerator<T> {
    rng: SmallRng,
    range: Option<Range<T>>,
}

impl<T> Default for NumberGenerator<T> {
    fn default() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            range: None,
        }
    }
}

impl<T> NumberGenerator<T>
where
    T: SampleUniform + PartialOrd + Copy,
    Standard: Distribution<T>,
{
    pub fn from_range(range: Range<T>) -> Self {
        Self {
            range: Some(range),
            ..Default::default()
        }
    }

    // pub fn with_range(mut self, range: Range<T>) -> Self {
    //     self.range = Some(range);

    //     self
    // }

    // pub fn from_seed(seed: u64) -> Self {
    //     Self {
    //         rng: SmallRng::seed_from_u64(seed),
    //         ..Default::default()
    //     }
    // }

    // pub fn with_seed(mut self, seed: u64) -> Self {
    //     self.rng = SmallRng::seed_from_u64(seed);

    //     self
    // }

    pub fn generate(&mut self) -> T {
        if let Some(range) = &self.range {
            self.rng.gen_range(range.start..range.end)
        } else {
            self.rng.gen()
        }
    }
}
