use axum::routing::MethodRouter;
use sqlx::{Pool, Sqlite};

pub struct RouteType<'a>
{
	pub path:   &'a str,
	pub method: MethodRouter<Pool<Sqlite>>
}

impl<'a> RouteType<'a>
{
	pub fn new(path: &'a str, method: MethodRouter<Pool<Sqlite>>) -> Self
	{
		RouteType{path, method}
	}
}