use crate::traits::Generator;

pub(crate) trait GeneratorExtras: Generator {}

impl<G: Generator + ?Sized> GeneratorExtras for G {}
