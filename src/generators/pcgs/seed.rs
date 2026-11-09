#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PcgSeed<T> {
    pub(crate) state: T,
    pub(crate) stream: T,
}
