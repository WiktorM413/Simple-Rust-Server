use axum::{routing::{get, post}};

use crate::{conn::ConnManager, routeManager::{RouteManager, routeType::RouteType}, routes::users::{CreateUser, GetUsers}};

mod db;
mod models;
mod routes;
mod conn;
mod routeManager;

#[tokio::main]
async fn main()
{
	dotenvy::dotenv().ok();

	let routeManager = RouteManager::from(vec![
		RouteType::new("/users",        get(GetUsers)),
		RouteType::new("/users/create", post(CreateUser))
	]);
	
	let conn = ConnManager::connect(routeManager.intoVec()).await;
	
	conn.serve().await;
}
