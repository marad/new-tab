use crate::calendar::{CalendarFacade, Event};
use crate::config::Config;
use crate::feed::{FeedFacade, FeedItem};
use crate::server::ServerFacade;
use std::sync::{Arc, RwLock};

pub type Shared<T> = Arc<RwLock<T>>;

pub struct AppState {
    pub events: Vec<Event>,
    pub feed: Vec<FeedItem>,
}
pub type SharedAppState = Shared<AppState>;

pub struct AppContext {
    pub feed: Box<FeedFacade>,
    pub calendar: Box<CalendarFacade>,
    pub server: Box<ServerFacade>,
    pub config: Config,
}

pub type SharedAppContext = Shared<AppContext>;
