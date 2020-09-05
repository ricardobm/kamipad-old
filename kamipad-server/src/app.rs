//! Main application state for the server.

use crate::logging;
use crate::util::{Cache, CacheKey, CacheMap, CacheVal};

/// Wraps the entire application state. The singleton instance for this can
/// be retrieved through the `App::get()` method.
pub struct App {
	pub log: slog::Logger,
	ring_log: logging::RingLogger,

	cache_map: CacheMap,

	// This just resets the global logging when the App instance is discarded.
	_compat_log_guard: slog_scope::GlobalLoggerGuard,
}

impl App {
	/// Retrieves the global application state instance. This will initialize
	/// the instance the first time is called.
	pub fn get() -> &'static App {
		lazy_static! {
			static ref APP: App = {

				// Compatibility with libraries using `log`

				// Logging schema
				// ==============
				//
				//     ┌─────┐
				//     │     │  ← ← ←  [filter > info]     ┌───────────────────┐
				//     │  T  │                ↑            │                   │
				//     │  E  │              [dup]        ← │ log compatibility │
				//     │  R  │                ↓            │                   │
				//     │  M  │        ┌───────────────┐    └───────────────────┘
				//     │  I  │        │ App::ring_log │
				//     │  N  │        └───────────────┘
				//     │  A  │                ↑            ┌───────────────────┐
				//     │  L  │        ┌───────────────┐    │                   │
				//     │     │  ← ← ← │   App::log    │  ← │ application logs  │
				//     └─────┘        └───────────────┘    │                   │
				//                            ↑            └───────────────────┘
				//                            ↑
				//                            ↑            ┌───────────────────┐
				//  ┌───────────┐     ┌───────────────┐    │                   │
				//  │ Log Cache │ ← ← │ RequestLogger │  ← │   request logs    │
				//  └───────────┘     └───────────────┘    │                   │
				//                                         └───────────────────┘
				//

				use slog::Drain;

				// Root drain that outputs to the terminal
				let term = slog_term::term_compact();
				let term = std::sync::Mutex::new(term);

				// The root logger, outputting to the terminal
				let term = slog::Logger::root(term.fuse(), o!());

				// Ring drain that keep all entries for `/api/logs`
				let ring_log = logging::RingLogger::new(1000);

				// Filter out debug/trace entries from libraries
				let filter = slog::LevelFilter::new(term.clone(), slog::Level::Info);

				// The compatibility logs go filtered to `term` and unfiltered
				// to the ring logger.
				let compat_log = slog::Duplicate::new(ring_log.clone(), filter);

				// Setup the compatibility logger
				let compat_log = slog::Logger::root(compat_log.fuse(), o!("library" => true));
				let compat_log_guard = slog_scope::set_global_logger(compat_log);
				slog_stdlog::init().unwrap();

				// Application logs go to the ring logger and `term`.
				let app_log = slog::Duplicate::new(ring_log.clone(), term);
				let app_log = slog::Logger::root(app_log.fuse(), o!());

				time!(t_init);
				info!(app_log, "starting application");

				let app = App {
					log: app_log,
					ring_log: ring_log,
					cache_map: CacheMap::new(),

					_compat_log_guard: compat_log_guard,
				};

				trace!(app.log, "application initialized"; t_init);

				app
			};
		}
		&APP
	}

	/// Returns a global cache instance for a given key and value types.
	pub fn cache<K: CacheKey + 'static, V: CacheVal + 'static>(&self) -> Cache<K, V> {
		self.cache_map.get()
	}

	/// Creates a new [Logger] for a request.
	///
	/// A request logger will still log entries globally, but will also store
	/// entries in the [RequestLogStore.]
	///
	/// Returns the created logger
	pub fn request_log<T>(
		&self,
		values: slog::OwnedKV<T>,
	) -> (slog::Logger, logging::RequestLogStore)
	where
		T: slog::SendSyncRefUnwindSafeKV + 'static,
	{
		let logger = logging::RequestLogger::new(self.log.clone());
		let store = logger.store();
		(slog::Logger::root(logger, values), store)
	}

	/// Return the latest log entries for the application.
	pub fn all_logs(&self) -> Vec<logging::LogEntry> {
		self.ring_log.entries()
	}
}
