pub mod parts;
pub mod build;

use parts::Parts;


pub struct Response<T> {
    head: Parts,
    body: T
}


impl Response<()> {
    
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }

}
