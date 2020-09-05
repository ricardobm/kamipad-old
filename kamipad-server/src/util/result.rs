/// Simple custom string error.
#[derive(Debug)]
pub struct Error(String);

/// Result using an [Error].
pub type Result<T> = std::result::Result<T, Error>;

//
// Error implementation
//

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Error {
	pub fn from<T: std::fmt::Display>(value: T) -> Error {
		Error(format!("{}", value))
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		&self.0
	}
}

pub trait ToError {
	fn to_err(self) -> Error;
}

impl From<String> for Error {
	fn from(v: String) -> Self {
		Error(v)
	}
}

impl<T: Into<String>> ToError for T {
	fn to_err(self) -> Error {
		Error::from(self.into())
	}
}

//
// Implement default conversions for the Error type:
//

macro_rules! error_from {
	($from: ty) => {
		impl From<$from> for Error {
			#[inline]
			fn from(v: $from) -> Self {
				Error::from(v)
			}
		}
	};
}

error_from!(std::io::Error);
error_from!(uuid::Error);
error_from!(serde_json::Error);
error_from!(std::fmt::Error);

// error_from!(reqwest::header::ToStrError);
// error_from!(reqwest::Error);
// error_from!(reqwest::UrlError);
