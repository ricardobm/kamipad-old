const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
	std::process::exit(run())
}

fn run() -> i32 {
	println!("\nStarting Kamipad server - v{}...", VERSION);
	0
}
