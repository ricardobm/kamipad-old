use regex::Regex;
use std::fmt;
use uuid::Uuid;

lazy_static! {
	static ref RE_ID_FORMAT: Regex =
		Regex::new(r"^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$").unwrap();
}

/// Universally unique ID for the database.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct ID {
	// Out ID is just a thin wrapper around an uuid::Uuid.
	inner: Uuid,
}

impl ID {
	/// Creates a new unique ID.
	pub fn new() -> ID {
		ID {
			inner: Uuid::new_v4(),
		}
	}

	/// Returns a nil ID.
	pub fn nil() -> ID {
		ID { inner: Uuid::nil() }
	}

	/// Parses a string into an ID.
	pub fn parse<S: AsRef<str>>(input: S) -> Option<ID> {
		let input = input.as_ref();
		if RE_ID_FORMAT.is_match(input) {
			if let Ok(inner) = Uuid::parse_str(input) {
				Some(ID { inner })
			} else {
				None
			}
		} else {
			None
		}
	}

	/// Returns true if the ID is nil.
	pub fn is_nil(&self) -> bool {
		self.inner.is_nil()
	}
}

impl fmt::Display for ID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.to_hyphenated_ref().fmt(f)
	}
}

impl fmt::Debug for ID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(self, f)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_id() {
		let id = ID::new();
		assert!(!id.is_nil());

		let id = ID::parse("645a9c23-9590-49d0-879e-250bff5b621a").unwrap();
		assert!(!id.is_nil());
		assert_eq!(id.to_string(), "645a9c23-9590-49d0-879e-250bff5b621a");
	}

	#[test]
	fn test_id_randomness() {
		let id1 = ID::new();
		let id2 = ID::new();
		assert_ne!(id1, id2);
	}

	#[test]
	fn test_nil_id() {
		let id = ID::nil();
		assert!(id.is_nil());
		assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");

		let id = ID::parse("00000000-0000-0000-0000-000000000000").unwrap();
		assert!(id.is_nil());
	}

	#[test]
	fn test_parsing_format() {
		assert!(ID::parse("645a9c23-9590-49d0-879e-250bff5b621a").is_some());
		assert!(ID::parse("645A9C23-9590-49D0-879E-250BFF5B621A").is_none());
		assert!(ID::parse("645a9c23-9590-49d0-879e-250bff5b621aF").is_none());
		assert!(ID::parse("645a9c23-9590-49d0-879e-250bff5b621").is_none());
		assert!(ID::parse("645a9c23959049d0879e250bff5b621a").is_none());
	}
}
