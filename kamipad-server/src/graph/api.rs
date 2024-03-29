//! Implementation for the GraphQL endpoints.

use rocket::response::content::Html;
use rocket::State;

use crate::app::App;
use crate::graph;
use crate::logging::RequestLog;

/// This endpoint just servers the static HTML for the GraphiQL interface.
#[get("/graphiql")]
pub fn ide() -> Html<String> {
	Html(graphiql_source("Kamipad - GraphiQL", "/api/graphql"))
}

/// This endpoint is responsible for executing a GraphQL query.
#[post("/graphql", data = "<request>")]
pub fn query(
	app: State<&App>,
	log: RequestLog,
	request: juniper_rocket::GraphQLRequest,
	schema: State<graph::Schema>,
) -> juniper_rocket::GraphQLResponse {
	let context = graph::Context { app: &app, log };
	request.execute(&schema, &context)
}

// spell-checker: disable

fn graphiql_source(title: &str, url: &str) -> String {
	return format!(
		r#"
			<!DOCTYPE html>
			<html>
				<head>
					<title>{title}</title>
					{style}
					<script src="https://cdn.jsdelivr.net/es6-promise/4.0.5/es6-promise.auto.min.js"></script>
					<script src="https://cdn.jsdelivr.net/fetch/0.9.0/fetch.min.js"></script>
					<script src="https://cdn.jsdelivr.net/react/15.4.2/react.min.js"></script>
					<script src="https://cdn.jsdelivr.net/react/15.4.2/react-dom.min.js"></script>
					<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/graphiql/0.11.11/graphiql.min.css" />
					<script src="https://cdnjs.cloudflare.com/ajax/libs/graphiql/0.11.11/graphiql.min.js"></script>
				</head>
				<body>
				<div id="graphiql">Loading...</div>
				<script>var GRAPHQL_URL = '{url}';</script>
				{script}
				</body>
			</html>
		"#,
		title = title,
		url = url,
		style = STYLE,
		script = SCRIPT,
	);
}

const STYLE: &'static str = r#"
	<style>
	body {
		height: 100%;
		margin: 0;
		width: 100%;
		overflow: hidden;
	}
	#graphiql {
		height: 100vh;
	}
	</style>
"#;

const SCRIPT: &'static str = r#"
	<script>
		/**
		 * This GraphiQL example illustrates how to use some of GraphiQL's props
		 * in order to enable reading and updating the URL parameters, making
		 * link sharing of queries a little bit easier.
		 *
		 * This is only one example of this kind of feature, GraphiQL exposes
		 * various React params to enable interesting integrations.
		 */
		// Parse the search string to get url parameters.
		var search = window.location.search;
		var parameters = {};
		search.substr(1).split('&').forEach(function (entry) {
			var eq = entry.indexOf('=');
			if (eq >= 0) {
				parameters[decodeURIComponent(entry.slice(0, eq))] =
				decodeURIComponent(entry.slice(eq + 1));
			}
		});
		// If variables was provided, try to format it.
		if (parameters.variables) {
			try {
				parameters.variables =
				JSON.stringify(JSON.parse(parameters.variables), null, 2);
			} catch (e) {
				// Do nothing, we want to display the invalid JSON as a string, rather
				// than present an error.
			}
		}
		// When the query and variables string is edited, update the URL bar so
		// that it can be easily shared.
		function onEditQuery(newQuery) {
			parameters.query = newQuery;
			updateURL();
		}
		function onEditVariables(newVariables) {
			parameters.variables = newVariables;
			updateURL();
		}
		function onEditOperationName(newOperationName) {
			parameters.operationName = newOperationName;
			updateURL();
		}
		function updateURL() {
			var newSearch = '?' + Object.keys(parameters).filter(function (key) {
				return Boolean(parameters[key]);
			}).map(function (key) {
				return encodeURIComponent(key) + '=' +
				encodeURIComponent(parameters[key]);
			}).join('&');
			history.replaceState(null, null, newSearch);
		}
		// Defines a GraphQL fetcher using the fetch API. You're not required to
		// use fetch, and could instead implement graphQLFetcher however you like,
		// as long as it returns a Promise or Observable.
		function graphQLFetcher(graphQLParams) {
			// When working locally, the example expects a GraphQL server at the path /graphql.
			// In a PR preview, it connects to the Star Wars API externally.
			// Change this to point wherever you host your GraphQL server.
			const isDev = !window.location.hostname.match(/(^|\.)netlify\.com$|(^|\.)graphql\.org$/)
			const api = GRAPHQL_URL
			return fetch(api, {
				method: 'post',
				headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json',
				},
				body: JSON.stringify(graphQLParams),
				credentials: 'include',
			}).then(function (response) {
				return response.text();
			}).then(function (responseBody) {
				try {
				return JSON.parse(responseBody);
				} catch (error) {
				return responseBody;
				}
			});
		}
		// Render <GraphiQL /> into the body.
		// See the README in the top level of this module to learn more about
		// how you can customize GraphiQL by providing different values or
		// additional child elements.
		ReactDOM.render(
			React.createElement(GraphiQL, {
				fetcher: graphQLFetcher,
				query: parameters.query,
				variables: parameters.variables,
				operationName: parameters.operationName,
				onEditQuery: onEditQuery,
				onEditVariables: onEditVariables,
				defaultVariableEditorOpen: true,
				onEditOperationName: onEditOperationName
			}),
			document.getElementById('graphiql')
		);
	</script>
"#;
