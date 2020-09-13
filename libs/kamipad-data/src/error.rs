use std::fmt;
use std::io;

/// The error type associated with Database operations.
pub enum Error {
	Open(IOError),
	ReadLock(IOError),
	WriteLock(IOError),
}

impl Error {
	fn do_fmt(&self, f: &mut std::fmt::Formatter<'_>, _debug: bool) -> std::fmt::Result {
		match self {
			Error::Open(error) => write!(f, "opening the database: {}", error),
			Error::ReadLock(error) => write!(f, "locking the database for reading: {}", error),
			Error::WriteLock(error) => write!(f, "locking the database for writing: {}", error),
		}
	}
}

impl std::error::Error for Error {}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.do_fmt(f, true)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.do_fmt(f, false)
	}
}

// Wrapper for an `std::io::Error` with an additional message used in Error.
pub struct IOError {
	message: String,
	inner: io::Error,
}

impl IOError {
	pub fn new<S: Into<String>>(inner: io::Error, message: S) -> IOError {
		IOError {
			message: message.into(),
			inner,
		}
	}
}

impl fmt::Display for IOError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} -- {}", self.message, self.inner)
	}
}
