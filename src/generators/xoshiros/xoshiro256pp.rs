use crate::generators::seeds::Xoshiro256Seed;
use crate::generators::xoshiros::core::Xoshiro256Core;
use crate::traits::{Generator, SeedableGenerator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Xoshiro256PlusPlus {
    core: Xoshiro256Core,
}

impl Xoshiro256PlusPlus {
    const ROT: u32 = 23;
}

impl Generator for Xoshiro256PlusPlus {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        let out = self.core.state[0]
            .wrapping_add(self.core.state[3])
            .rotate_left(Self::ROT)
            .wrapping_add(self.core.state[0]);
        self.core.advance();
        out
    }
}

impl SeedableGenerator for Xoshiro256PlusPlus {
    type Seed = Xoshiro256Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            core: Xoshiro256Core {
                state: seed.words(),
            },
        }
    }
}
