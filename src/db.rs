use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub async fn ConnectDb() -> SqlitePool
{
	let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

	
	let options = SqliteConnectOptions::from_str(&url).unwrap().create_if_missing(true);

	SqlitePool::connect_with(options).await.expect("Failed to connect to db")
}