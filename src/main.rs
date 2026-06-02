use axum::{Router, routing::{get, post}};

use crate::routes::users::{CreateUser, GetUsers};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main()
{
	let pool = db::ConnectDb().await;

	let app: Router = Router::new()
		.route("/users",        get(GetUsers))
		.route("/users/create", post(CreateUser))
		.with_state(pool);

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

	axum::serve(listener, app).await.unwrap();
}
