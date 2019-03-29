use std::error;

pub trait ServerFacade {
    fn start_server(&self) -> Result<(), Box<error::Error>>;
}
