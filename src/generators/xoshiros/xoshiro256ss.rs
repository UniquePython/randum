use crate::generators::seeds::Xoshiro256Seed;
use crate::generators::xoshiros::core::Xoshiro256Core;
use crate::traits::{Generator, SeedableGenerator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Xoshiro256StarStar {
    core: Xoshiro256Core,
}

impl Xoshiro256StarStar {
    const MUL1: u64 = 5;
    const ROT: u32 = 7;
    const MUL2: u64 = 9;
}

impl Generator for Xoshiro256StarStar {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        let out = self.core.state[1]
            .wrapping_mul(Self::MUL1)
            .rotate_left(Self::ROT)
            .wrapping_mul(Self::MUL2);
        self.core.advance();
        out
    }
}

impl SeedableGenerator for Xoshiro256StarStar {
    type Seed = Xoshiro256Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            core: Xoshiro256Core {
                state: seed.words(),
            },
        }
    }
}
