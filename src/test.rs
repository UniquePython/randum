use crate::traits::SeedableGenerator;

pub(crate) fn check_u32<G: SeedableGenerator>(seed: G::Seed, outcomes: &[u32]) {
    let mut generator = G::from_seed(seed);
    for (idx, &outcome) in outcomes.iter().enumerate() {
        assert_eq!(generator.next_u32(), outcome, "outcome {idx}");
    }
}

pub(crate) fn check_u64<G: SeedableGenerator>(seed: G::Seed, outcomes: &[u64]) {
    let mut generator = G::from_seed(seed);
    for (idx, &outcome) in outcomes.iter().enumerate() {
        assert_eq!(generator.next_u64(), outcome, "outcome {idx}");
    }
}
