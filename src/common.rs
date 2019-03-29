use crate::calendar::{Calendar, Event};
use crate::clients::google::token_storage::DiskStorage;
use crate::config;
use crate::feed::{FeedFacade, FeedItem};
use std::sync::{Arc, RwLock};

pub type Shared<T> = Arc<RwLock<T>>;

pub struct AppState {
    pub events: Vec<Event>,
    pub feed: Vec<FeedItem>,
}
pub type SharedAppState = Shared<AppState>;

pub struct AppContext {
    pub feed: Box<FeedFacade>,
    pub config: config::Config,
    pub calendar: Calendar<DiskStorage>,
}

pub type SharedAppContext = Shared<AppContext>;
