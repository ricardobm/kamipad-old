//! Contains miscellaneous utilities for the code.

#[macro_use]
pub mod time;

mod result;
pub use self::result::Error;
pub use self::result::Result;

mod cache;
pub use self::cache::{Cache, CacheKey, CacheMap, CacheVal};
