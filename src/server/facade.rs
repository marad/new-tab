use failure::Fallible;

use crate::common::*;

pub trait ServerFacade: Send + Sync {
    fn start_server(&self, app_state: SharedAppState) -> Fallible<()>;
}
