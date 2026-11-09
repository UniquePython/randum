#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Xoshiro256Core {
    pub(super) state: [u64; 4],
}

impl Xoshiro256Core {
    const SHIFT: u64 = 17;
    const ROT: u32 = 45;
}

impl Xoshiro256Core {
    pub(super) fn advance(&mut self) {
        let t = self.state[1] << Self::SHIFT;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(Self::ROT);
    }
}
