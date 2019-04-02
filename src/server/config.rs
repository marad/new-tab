use super::actix_server::ActixServer;
use super::facade::ServerFacade;
use super::rocket_server::RocketServer;

pub fn rocket() -> impl ServerFacade {
    RocketServer::new()
}

pub fn actix() -> impl ServerFacade {
    ActixServer::new()
}
