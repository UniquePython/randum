mod pcgs;
mod splitmix64;
mod xorshift64;

pub(crate) use pcgs::Pcg32;
pub(crate) use splitmix64::SplitMix64;
pub(crate) use xorshift64::XorShift64;

pub(crate) use pcgs::PcgSeed;

pub(crate) mod bit32 {
    pub(crate) use crate::generators::Pcg32;
}

pub(crate) mod bit64 {
    pub(crate) use crate::generators::SplitMix64;
    pub(crate) use crate::generators::XorShift64;
}
