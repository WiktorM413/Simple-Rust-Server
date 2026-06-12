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

	// Consumes self
	pub fn intoVec(self) -> Vec<RouteType<'a>>
	{
		self.routes
	}

	pub fn mount(mut self, route: RouteType<'a>) -> Self
	{
		self.routes.push(route);
		
		return self;
	}
}