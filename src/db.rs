use sqlx::SqlitePool;

pub async fn ConnectDb() -> SqlitePool
{
	SqlitePool::connect("sqlite://database.db").await.expect("Failed to connect to db")
}