use crate::traits::{Generator, SeedableGenerator};
use std::num::NonZeroU64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    const SHL1: u64 = 13;
    const SHR: u64 = 7;
    const SHL2: u64 = 17;
}

impl Generator for XorShift64 {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << Self::SHL1;
        self.state ^= self.state >> Self::SHR;
        self.state ^= self.state << Self::SHL2;
        self.state
    }
}

impl SeedableGenerator for XorShift64 {
    type Seed = NonZeroU64;

    fn from_seed(seed: Self::Seed) -> Self {
        Self { state: seed.get() }
    }
}
