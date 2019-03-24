use crate::calendar::{Calendar, Event};
use crate::clients::google::token_storage::DiskStorage;
use crate::clients::hackernews::Item;
use crate::config;
use std::sync::{Arc, RwLock};

pub type Shared<T> = Arc<RwLock<T>>;

pub struct AppState {
    pub events: Vec<Event>,
    pub feed: Vec<Item>,
}
pub type SharedAppState = Shared<AppState>;

pub struct AppContext {
    pub config: config::Config,
    pub calendar: Calendar<DiskStorage>,
}
