pub(crate) trait Generator {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
}
