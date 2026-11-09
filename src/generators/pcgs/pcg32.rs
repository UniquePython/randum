use crate::generators::seeds::PcgSeed;
use crate::traits::{Generator, SeedableGenerator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    const MUL: u64 = 6364136223846793005;
    const SHIFT1: u64 = 18;
    const SHIFT2: u64 = 27;
    const SHIFT3: u64 = 59;
}

impl Pcg32 {
    fn step(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(Self::MUL).wrapping_add(self.inc);
        let xorshifted = (((old >> Self::SHIFT1) ^ old) >> Self::SHIFT2) as u32;
        let rot = (old >> Self::SHIFT3) as u32;
        xorshifted.rotate_right(rot)
    }
}

impl Generator for Pcg32 {
    fn next_u32(&mut self) -> u32 {
        self.step()
    }

    fn next_u64(&mut self) -> u64 {
        let first = self.step() as u64;
        let second = self.step() as u64;

        (first << 32) | second
    }
}

impl SeedableGenerator for Pcg32 {
    type Seed = PcgSeed<u64>;

    fn from_seed(seed: Self::Seed) -> Self {
        let inc = (seed.stream << 1) | 1;
        let mut this = Pcg32 { state: 0, inc };
        let _ = this.step();
        this.state = this.state.wrapping_add(seed.state);
        let _ = this.step();
        this
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::check_u32;

    const TEST_VECTOR_SEED: PcgSeed<u64> = PcgSeed {
        state: 42,
        stream: 54,
    };
    const TEST_VECTOR: &[u32] = &[2707161783, 2068313097, 3122475824, 2211639955, 3215226955];

    #[test]
    fn test_vector() {
        check_u32::<Pcg32>(TEST_VECTOR_SEED, TEST_VECTOR);
    }
}
