use super::{FeedFacade, FeedItem};
use failure::Fallible;

pub struct MemoryFeed {
    feed: Vec<FeedItem>,
}

impl MemoryFeed {
    pub fn new(feed: Vec<FeedItem>) -> Self {
        Self { feed }
    }
}

impl FeedFacade for MemoryFeed {
    fn get_items(&self) -> Fallible<Vec<FeedItem>> {
        Ok(self.feed.clone())
    }
}
