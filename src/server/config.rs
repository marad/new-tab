use super::facade::ServerFacade;
use super::rocket_server::RocketServer;
use crate::common::*;

pub struct ServerConfig {}

impl ServerConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rocket_server(&self, app_state: &SharedAppState) -> impl ServerFacade {
        RocketServer::new(app_state)
    }
}
