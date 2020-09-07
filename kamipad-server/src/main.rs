#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate slog;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket;

// This needs to come first because of the `time!` macro.
#[macro_use]
mod util;

mod app;
mod common;
mod graph;
mod logging;
mod server;

fn main() {
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	let exit_code = rt.block_on(run());
	std::process::exit(exit_code);
}

async fn run() -> i32 {
	println!("\nStarting Kamipad server - v{}...\n", common::VERSION);
	println!("Initialized {}", kamipad_data::hello());

	let (mut tx, mut rx) = tokio::sync::mpsc::channel::<i32>(16);

	tokio::spawn(async move {
		if let Ok(_) = tokio::signal::ctrl_c().await {
			println!("\nReceived interrupt signal...\n");
			tx.send(0).await.ok();
		}
	});

	tokio::spawn(async {
		let app = app::App::get();
		server::launch(app);
	});

	let exit_code = rx.recv().await;
	println!("Shutdown complete!");

	exit_code.unwrap_or(0)
}
