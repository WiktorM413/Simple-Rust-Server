use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User
{
	pub id:       u32,
	pub username: String,
	pub age:      u32
}

#[derive(Deserialize)]
pub struct CreateUser
{
	pub username: String,
	pub age:      u32
}