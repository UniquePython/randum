use crate::traits::Generator;

pub(crate) trait SeedableGenerator: Generator + Sized {
    type Seed;

    fn from_seed(seed: Self::Seed) -> Self;
}
