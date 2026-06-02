use axum::{
	extract::State,
	Json
};

use sqlx::SqlitePool;

use crate::models::users::{CreateUser, User};

pub async fn CreateUser(State(pool): State<SqlitePool>, Json(payload): Json<CreateUser>) -> String
{
	sqlx::query("INSERT INTO users(username, age) VALUES (?, ?)").bind(payload.username).bind(payload.age).execute(&pool).await.unwrap();

	return "User created".to_string();
}

pub async fn GetUsers(State(pool): State<SqlitePool>) -> Json<Vec<User>>
{
	let users = sqlx::query_as::<_, User>("SELECT id, username, age FROM users").fetch_all(&pool).await.unwrap();

	Json(users)
}