extern crate kamipad_data;

use kamipad_data as kd;

fn main() {
	println!("\nSimple database example\n");

	let exe_path = std::env::current_exe().unwrap();
	let exe_path = exe_path.parent().unwrap();
	println!("[Info ] Main directory is {}", exe_path.to_string_lossy());

	let db_path = exe_path.join("simple-database");
	println!(
		"[Info ] Opening database at {}...",
		db_path.to_string_lossy()
	);
	match kd::open(&db_path, Default::default()) {
		kd::Result::Ok(_db) => {
			println!("[Info ] Success!");
			std::thread::sleep(std::time::Duration::from_millis(10000));
		}
		kd::Result::Err(err) => {
			println!("[Error] {}", err);
		}
	}
}
