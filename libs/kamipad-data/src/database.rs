use std::fmt;
use std::fs;
use std::path::PathBuf;

/// Root type for a Database.
pub struct Database {
	/// This is the top-level directory for the database.
	pub path: PathBuf,

	read_only: bool,

	// We keep this tied to the Database instance, so that the database file
	// lock is released when the instance is dropped.
	_lock_file: fs::File,
}

pub(crate) struct InitConfig {
	pub path: PathBuf,
	pub read_only: bool,
	pub lock_file: fs::File,
}

impl Database {
	/// Returns a new instance of the database. This is used internally by the
	/// library to construct a new instance.
	pub(crate) fn new(config: InitConfig) -> Database {
		let db = Database {
			path: config.path,
			read_only: config.read_only,
			_lock_file: config.lock_file,
		};
		db
	}

	/// Returns true if the database has been opened in read-only mode.
	pub fn is_read_only(&self) -> bool {
		self.read_only
	}
}

impl fmt::Display for Database {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Database({}{})",
			if self.read_only { "[read-only] " } else { "" },
			self.path.to_string_lossy()
		)
	}
}

impl fmt::Debug for Database {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(self, f)
	}
}
