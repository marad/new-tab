use super::facade::ServerFacade;
use super::rocket_server::RocketServer;

pub struct ServerConfig {}

impl ServerConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rocket_server(&self) -> impl ServerFacade {
        RocketServer::new()
    }
}
