pub struct Config
{
	pub databaseUrl: String,
	pub host:        String,
	pub port:        u16
}

pub fn init() -> Result<Config, Box<dyn std::error::Error>>
{
	let config = Config{
		databaseUrl: std::env::var("DATABASE_URL")?,
		host:        std::env::var("HOST")?,
		port:        std::env::var("PORT")?.parse()?
	};

	Ok(config)
}