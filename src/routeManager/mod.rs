use crate::routeManager::routeType::RouteType;

pub mod routeType;

pub struct RouteManager<'a>
{
	routes: Vec<RouteType<'a>>
}

impl<'a> RouteManager<'a>
{
	pub fn new() -> Self
	{
		RouteManager{routes: vec![]}
	}

	pub fn from(routes: Vec<RouteType<'a>>) -> Self
	{
		RouteManager{routes}
	}

	pub fn addRoute(&mut self, route: RouteType<'a>)
	{
		self.routes.push(route);
	}

	// Consumes self
	pub fn intoVec(self) -> Vec<RouteType<'a>>
	{
		self.routes
	}
}