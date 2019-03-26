use super::{FeedFacade, FeedItem};
use std::error;

pub struct MemoryFeed {
    feed: Vec<FeedItem>,
}

impl MemoryFeed {
    pub fn new(feed: Vec<FeedItem>) -> Self {
        Self { feed }
    }
}

impl FeedFacade for MemoryFeed {
    fn get_items(&self) -> Result<Vec<FeedItem>, Box<error::Error>> {
        Ok(self.feed.clone())
    }
}
