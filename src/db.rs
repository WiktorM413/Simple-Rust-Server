use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::config::Config;

pub async fn ConnectDb(config: &Config) -> SqlitePool
{
	let options = SqliteConnectOptions::from_str(&config.databaseUrl).unwrap().create_if_missing(true);

	SqlitePool::connect_with(options).await.expect("Failed to connect to db")
}