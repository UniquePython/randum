use crate::traits::Generator;

pub(crate) trait GeneratorExtras: Generator {
    fn next_bool(&mut self) -> bool;
}

impl<G: Generator + ?Sized> GeneratorExtras for G {
    fn next_bool(&mut self) -> bool {
        (self.next_u32() >> 31) == 1
    }
}
