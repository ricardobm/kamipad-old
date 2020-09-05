use rocket::State;
use rocket_contrib::json::Json;

use crate::app::App;
use crate::common;
use crate::graph;
use crate::logging;

/// Launch the Rocket server.
pub fn launch(app: &'static App) {
	rocket::ignite()
		.attach(logging::ServerLogger {})
		.manage(app)
		.manage(graph::Schema::new(graph::Query, graph::Mutation))
		.mount(
			"/api",
			routes![index, logs, log_by_req, graph::api::ide, graph::api::query],
		)
		.launch();
}

//============================================================================//
// Index
//============================================================================//

#[derive(Serialize)]
struct IndexData {
	name: &'static str,
	version: &'static str,
	description: &'static str,
}

#[get("/")]
fn index() -> Json<IndexData> {
	Json(IndexData {
		name: common::PACKAGE_NAME,
		version: common::VERSION,
		description: common::PACKAGE_DESCRIPTION,
	})
}

//============================================================================//
// Logging
//============================================================================//

#[get("/logs")]
fn logs(app: State<&App>) -> Json<Vec<logging::LogEntry>> {
	Json(app.all_logs())
}

#[get("/log/<req>")]
fn log_by_req(req: logging::RequestId, app: State<&App>) -> Json<Vec<logging::LogEntry>> {
	let cache = app.cache();
	if let Some(entries) = cache.get(&req) {
		let entries: &Vec<logging::LogEntry> = &*entries;
		Json(entries.clone())
	} else {
		Json(vec![])
	}
}
