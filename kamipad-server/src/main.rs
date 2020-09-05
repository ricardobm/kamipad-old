#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate rocket;
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket_contrib;

extern crate percent_encoding;
extern crate rand;
extern crate uuid;

// This needs to come first because of the `time!` macro.
#[macro_use]
mod util;

mod app;
mod common;
mod graph;
mod logging;
mod server;

fn main() {
	std::process::exit(run())
}

fn run() -> i32 {
	println!("\nStarting Kamipad server - v{}...\n", common::VERSION);

	let app = app::App::get();
	server::launch(app);
	0
}
