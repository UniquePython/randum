use crate::traits::Generator;

pub(crate) trait GeneratorExtras: Generator {
    fn next_bool(&mut self) -> bool;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
}

impl<G: Generator + ?Sized> GeneratorExtras for G {
    fn next_bool(&mut self) -> bool {
        (self.next_u32() >> 31) == 1
    }

    fn next_f32(&mut self) -> f32 {
        let bits = (self.next_u32() >> 8) as f32;
        let scale = 1.0 / ((1u32 << 24) as f32);
        bits * scale
    }

    fn next_f64(&mut self) -> f64 {
        let bits = (self.next_u64() >> 11) as f64;
        let scale = 1.0 / ((1u64 << 53) as f64);
        bits * scale
    }
}
