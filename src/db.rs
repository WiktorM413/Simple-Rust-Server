use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::config;

pub async fn ConnectDb() -> SqlitePool
{
	let config = config::get();

	
	let options = SqliteConnectOptions::from_str(&config.databaseUrl).unwrap().create_if_missing(true);

	SqlitePool::connect_with(options).await.expect("Failed to connect to db")
}