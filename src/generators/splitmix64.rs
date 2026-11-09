use crate::traits::{Generator, SeedableGenerator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const GAMMA: u64 = 0x9E3779B97F4A7C15;
    const MUL1: u64 = 0xBF58476D1CE4E5B9;
    const MUL2: u64 = 0x94D049BB133111EB;
}

impl Generator for SplitMix64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(Self::GAMMA);

        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(Self::MUL1);
        z = (z ^ (z >> 27)).wrapping_mul(Self::MUL2);
        z ^ (z >> 31)
    }
}

impl SeedableGenerator for SplitMix64 {
    type Seed = u64;

    fn from_seed(seed: Self::Seed) -> Self {
        Self { state: seed }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::check_u64;

    const TEST_VECTOR_SEED: u64 = 1234567;
    const TEST_VECTOR: &[u64] = &[
        6457827717110365317,
        3203168211198807973,
        9817491932198370423,
        4593380528125082431,
        16408922859458223821,
    ];

    #[test]
    fn test_vector() {
        check_u64::<SplitMix64>(TEST_VECTOR_SEED, TEST_VECTOR);
    }
}
