use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub async fn ConnectDb() -> SqlitePool
{
	let options = SqliteConnectOptions::from_str("sqlite://database.db").unwrap().create_if_missing(true);

	SqlitePool::connect_with(options).await.expect("Failed to connect to db")
}