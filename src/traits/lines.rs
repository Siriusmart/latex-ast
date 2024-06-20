/// Members implementing this trait can be checked for line number without converting into string.
pub trait Lines {
    /// Returns the number of lines (min 1).
    fn lines(&self) -> u32;
}
