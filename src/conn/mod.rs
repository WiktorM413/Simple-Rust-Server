use axum::{Router, routing::{get, post}};
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;


use crate::{db, routeType::RouteType, routes::users::{CreateUser, GetUsers}};

pub struct ConnManager
{
	app:      Router,
	listener: TcpListener
}

impl ConnManager
{
	pub async fn connect(routes: Vec<RouteType<'_>>) -> Self
	{
		let pool = db::ConnectDb().await;

		let app: Router = routes.into_iter().fold(
			Router::new(),
			|router, route| router.route(route.path, route.method)
		).with_state(pool);

		let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

		ConnManager{app, listener}
	}

	pub async fn serve(self)
	{
		axum::serve(self.listener, self.app.clone().into_make_service()).await.unwrap();
	}
}