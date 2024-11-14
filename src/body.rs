// TODO: Create a macro that implements the trait trivially to
// types that have a `len` method.

pub trait Body {
    /// Returns the number of bytes of the body.
    fn content_len(&self) -> usize;
}

impl Body for String {
    fn content_len(&self) -> usize {
        self.len()
    }
}

impl Body for &[u8] {
    fn content_len(&self) -> usize {
        self.len()
    }
}

