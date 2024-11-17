pub mod name;
pub mod value;

use std::collections::HashMap;

pub use name::HeaderName;
pub use value::HeaderValue;
use crate::error::{Error, Result};


#[derive(Debug, Default)]
pub struct HeaderMap {
    pub inner: HashMap<HeaderName, HeaderValue>,
}


impl HeaderMap {

    /// Creates a new, default `HeaderMap`.
    pub fn new() -> Self {
        HeaderMap::default()
    }

    /// Inserts a header in the `HeaderMap`, from name and value expressed 
    /// in slices of bytes.
    pub fn insert<N, V>(&mut self, name: N, value: V) -> Result<Option<HeaderValue>> 
        where
            HeaderValue: From<V>,
            HeaderName: TryFrom<N>,
            <HeaderName as TryFrom<N>>::Error: Into<Error>,
    {
        let name = TryFrom::try_from(name).map_err(Into::into)?;
        let value = From::from(value);
        Ok(self.inner.insert(name, value))
    }


    /// Returns the header-value in the `Request` given a header-name.
    ///
    /// If the header-name is invalid, then the method returns `None`.
    #[inline]
    pub fn get(&self, name: &HeaderName) -> Option<&HeaderValue> {
        self.inner.get(name)
    }
}




