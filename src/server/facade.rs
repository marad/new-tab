use std::error;

use crate::common::*;

pub trait ServerFacade {
    fn start_server(&self, app_state: SharedAppState) -> Result<(), Box<error::Error>>;
}
