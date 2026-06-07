use axum::{routing::{get, post}};

use crate::{conn::ConnManager, routeManager::routeType::RouteType, routes::users::{CreateUser, GetUsers}};

mod db;
mod models;
mod routes;
mod conn;
mod routeManager;

#[tokio::main]
async fn main()
{
	dotenvy::dotenv().ok();
		
	let routes: Vec<RouteType> = vec![
		RouteType::new("/users",        get(GetUsers)),
		RouteType::new("/users/create", post(CreateUser))
	];
	
	let conn = ConnManager::connect(routes).await;
	
	conn.serve().await;
}
