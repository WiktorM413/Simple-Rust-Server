use axum::{Router};
use tokio::net::TcpListener;

use crate::{config::{self, Config}, db, routeManager::routeType::RouteType};

pub struct ConnManager
{
	app:      Router,
	listener: TcpListener
}

impl ConnManager
{
	pub async fn connect(config: &Config, routes: Vec<RouteType<'_>>) -> Self
	{
		let pool = db::ConnectDb(config).await;

		let app: Router = routes.into_iter().fold(
			Router::new(),
			|router, route| router.route(route.path, route.method)
		).with_state(pool);

		let apiLink = format!("{}:{}", config.host, config.port);
		
		let listener = TcpListener::bind(apiLink).await.unwrap();

		ConnManager{app, listener}
	}

	pub async fn serve(self)
	{
		axum::serve(self.listener, self.app.clone().into_make_service()).await.unwrap();
	}
}