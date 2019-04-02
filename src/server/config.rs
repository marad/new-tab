use super::actix_server::ActixServer;
use super::facade::ServerFacade;

pub fn actix() -> impl ServerFacade {
    ActixServer::new()
}
