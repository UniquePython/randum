#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Xoshiro256Seed {
    words: [u64; 4],
}

impl Xoshiro256Seed {
    pub(crate) fn new(words: [u64; 4]) -> Option<Self> {
        if words.iter().all(|&word| word == 0) {
            return None;
        }

        Some(Self { words })
    }

    pub(crate) fn words(&self) -> [u64; 4] {
        self.words
    }
}
