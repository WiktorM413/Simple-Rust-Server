use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config
{
	pub databaseUrl: String,
	pub host:        String,
	pub port:        u16
}

pub fn init() -> Result<(), Box<dyn std::error::Error>>
{
	let config = Config{
		databaseUrl: std::env::var("DATABASE_URL")?,
		host:        std::env::var("HOST")?,
		port:        std::env::var("PORT")?.parse()?
	};

	CONFIG.set(config).map_err(|_| "Config already initialized")?;

	Ok(())
}

pub fn get() -> &'static Config
{
	CONFIG.get().expect("config::init() was not called")
}