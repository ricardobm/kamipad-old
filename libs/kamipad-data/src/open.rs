//! Opening procedure for the database.
//!
//! Since the database is just a loose collection of files on a directory, for
//! the most part we don't keep file handles around until we need to write or
//! read them.
//!
//! The only exception to the above is the database lock file (see `DB_LOCK_FILENAME`)
//! which is kept open. The lock is an empty file that is used only to control
//! shared access to the database.
//!
//! Given the above, opening the database amounts to opening the lock file
//! and acquiring an exclusive lock (if writing is enabled) or a shared lock
//! (if mode is read-only) on it.
//!
//! If the create flag is set, then the open function will also create
//! the database directory structure and lock file.
//!
//! The open function may also perform sanity checks on the database file system
//! to make sure structure is valid and fail early if it's not.
//!
//! Additionally, when opening the database in writing mode, the open function
//! will also flush the transaction log to commit any pending transactions.

use std::fs;
use std::path::PathBuf;

const DB_LOCK_FILENAME: &'static str = "db.lock";

use crate::database::{Database, InitConfig};
use crate::error::{Error, IOError};
use crate::Result;

/// Opens a database, optionally creating it if it does not exist.
///
/// # Arguments
///
/// - `path` - The path to the top-level database directory.
/// - `flags` - Database opening flags.
pub fn open<P: Into<PathBuf>>(path: P, flags: OpenFlags) -> Result<Database> {
	let main_path = path.into();

	// Create the top-level database directory, if necessary.
	if flags.create {
		fs::create_dir_all(&main_path).map_err(|err| {
			Error::Open(IOError::new(
				err,
				format!(
					"creating database directory at `{}`",
					main_path.to_string_lossy()
				),
			))
		})?;
	}

	// Create or open the database lock file.
	let lock_path = main_path.join(DB_LOCK_FILENAME);
	let lock_file = fs::OpenOptions::new()
		.read(true)
		.write(flags.create)
		.create(flags.create)
		.open(&lock_path)
		.map_err(|err| {
			Error::Open(IOError::new(
				err,
				format!(
					"{} lock file `{}`",
					if flags.create {
						"opening or creating"
					} else {
						"opening"
					},
					lock_path.to_string_lossy()
				),
			))
		})?;

	// Acquire a lock on the database lock file. If the database is being
	// opened for writing, we acquire an exclusive lock, otherwise we acquire
	// a shared lock.
	use fs2::FileExt;
	if flags.read_only {
		lock_file.try_lock_shared().map_err(|err| {
			Error::ReadLock(IOError::new(
				err,
				format!("acquiring shared lock on `{}`", lock_path.to_string_lossy()),
			))
		})?;
	} else {
		lock_file.try_lock_exclusive().map_err(|err| {
			Error::WriteLock(IOError::new(
				err,
				format!(
					"acquiring exclusive lock on `{}`",
					lock_path.to_string_lossy()
				),
			))
		})?;
	}

	let db = Database::new(InitConfig {
		path: main_path,
		read_only: flags.read_only,
		lock_file,
	});

	Result::Ok(db)
}

/// Opening flags for a Database.
///
/// The default for this is to create the database if it does not exist and
/// to open in write mode.
pub struct OpenFlags {
	/// If true, will attempt to create the database if it does not exist.
	///
	/// Default: true
	pub create: bool,

	/// Opens the database in read-only mode. This allows multiple consumers
	/// for the database, as long as there in no writer.
	///
	/// Default: false
	pub read_only: bool,
}

impl OpenFlags {
	/// Returns a new default instance of OpenFlags.
	///
	/// # Examples
	///
	/// ```
	/// use kamipad_data::OpenFlags;
	/// let cfg = OpenFlags::default();
	/// assert!(cfg.create);
	/// assert!(!cfg.read_only);
	/// ```
	pub fn default() -> Self {
		Default::default()
	}

	/// Allows a callback to be used to configure a default OpenFlags instance.
	///
	/// This is useful to change the default instance flags inline in a fluent
	/// call chain.
	///
	/// # Examples
	///
	/// ```
	/// use kamipad_data::OpenFlags;
	/// let cfg = OpenFlags::config(|c| {
	///     c.read_only = true;
	/// });
	/// assert!(cfg.read_only);
	/// ```
	pub fn config<T: FnOnce(&mut Self)>(callback: T) -> Self {
		let mut flags = Self::default();
		callback(&mut flags);
		flags
	}

	/// Returns a OpenFlags instance configured for only reading a database.
	///
	/// # Examples
	///
	/// ```
	/// use kamipad_data::OpenFlags;
	/// let cfg = OpenFlags::read_only();
	/// assert!(cfg.read_only);
	/// assert!(!cfg.create);
	/// ```
	pub fn read_only() -> Self {
		Self::config(|f| {
			f.read_only = true;
			f.create = false;
		})
	}
}

impl Default for OpenFlags {
	fn default() -> Self {
		OpenFlags {
			create: true,
			read_only: false,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	use std::fs;
	use tempdir::TempDir;

	#[test]
	fn should_open_new_database() {
		let (db, temp) = create_db(OpenFlags::default());
		let path = db.path.clone();
		drop(db);

		// Make sure the database path was created.
		assert!(fs::metadata(path).unwrap().is_dir());

		// Make sure the database instance cleans up after itself.
		temp.close().unwrap();
	}

	#[test]
	fn should_open_existing_database() {
		let (db, temp) = create_db(OpenFlags::default());
		let path = db.path.clone();
		drop(db);

		// Make sure the second instance is able to be opened.
		let db = open(path, OpenFlags::config(|f| f.create = false)).unwrap();
		drop(db);

		// Make sure the database instance cleans up after itself.
		temp.close().unwrap();
	}

	#[test]
	fn should_allow_multiple_readers() {
		let (db, temp) = create_db(OpenFlags::default());
		let path = db.path.clone();
		drop(db);

		let db1 = open(&path, OpenFlags::read_only()).unwrap();
		let db2 = open(&path, OpenFlags::read_only()).unwrap();
		drop(db1);
		drop(db2);

		// Make sure the database instance cleans up after itself.
		temp.close().unwrap();
	}

	#[test]
	fn should_lock_database_for_writing() {
		let (db, temp) = create_db(OpenFlags::default());
		let path = db.path.clone();

		let err = open(&path, OpenFlags::read_only()).unwrap_err();
		match err {
			Error::ReadLock(_) => (),
			_ => panic!("open error should be Error::ReadLock, but it was {}", err),
		}

		let err = open(&path, OpenFlags::default()).unwrap_err();
		match err {
			Error::WriteLock(_) => (),
			_ => panic!("open error should be Error::WriteLock, but it was {}", err),
		}

		drop(db);

		// Make sure the database instance cleans up after itself.
		temp.close().unwrap();
	}

	fn create_db(flags: OpenFlags) -> (Database, TempDir) {
		let temp = tempdir::TempDir::new("kamipad-data").unwrap();
		let path = temp.path().join("db");
		let db = open(&path, flags).unwrap();
		(db, temp)
	}
}
