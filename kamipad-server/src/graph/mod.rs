//! Contains the GraphQL API support code and resolvers.
//!
//! The main types in this module are `Context`, `Query` and `Mutation`.
//!
//! The submodule `api` contains the API interfaces for resolving GraphQL and
//! the GraphiQL endpoint.

use crate::app::App;
use crate::common;
use crate::logging::RequestLog;

pub mod api;

/// Context for GraphQL. This wraps all the data available to a GraphQL
/// resolver, which basically boils down to the `App` instance and the
/// request log.
///
/// Any resolver in GraphQL can use this by simply receiving a reference to
/// the context as argument.
pub struct Context {
	pub app: &'static App,
	pub log: RequestLog,
}

impl juniper::Context for Context {}

/// Root for GraphQL queries. Any method implemented here will be available
/// to the GraphQL interface.
pub struct Query;

/// Root for GraphQL mutations. Any method implemented here will be available
/// to the GraphQL interface.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Query {
	/// Server application name.
	fn app_name() -> &'static str {
		common::PACKAGE_NAME
	}

	/// Server application description.
	fn app_description() -> &'static str {
		common::PACKAGE_DESCRIPTION
	}

	/// Server version.
	fn app_version() -> &'static str {
		common::VERSION
	}
}

#[juniper::object(Context = Context)]
impl Mutation {
	/// A no-op operation to test mutations.
	fn no_op(context: &Context) -> i32 {
		42
	}
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
