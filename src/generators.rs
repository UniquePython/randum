mod pcgs;
mod splitmix64;
mod xorshift64;
mod xoshiros;

pub(crate) mod seeds {
    pub(crate) use crate::generators::pcgs::PcgSeed;
    pub(crate) use crate::generators::xoshiros::Xoshiro256Seed;
}

pub(crate) mod bit32 {
    pub(crate) use crate::generators::pcgs::Pcg32;
}

pub(crate) mod bit64 {
    pub(crate) use crate::generators::splitmix64::SplitMix64;
    pub(crate) use crate::generators::xorshift64::XorShift64;

    pub(crate) mod xoshiro {
        pub(crate) use crate::generators::xoshiros::Xoshiro256PlusPlus;
        pub(crate) use crate::generators::xoshiros::Xoshiro256StarStar;
    }
}
