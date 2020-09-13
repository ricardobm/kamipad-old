mod error;
pub use error::Error;

mod database;
pub use database::Database;

/// Result type for the data library.
pub type Result<T> = std::result::Result<T, Error>;

mod open;
pub use open::{open, OpenFlags};
