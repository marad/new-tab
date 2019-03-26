use super::facade::{FeedFacade, FeedItem};
use super::hackernews_feed::Hackernews;
use super::memory_feed::MemoryFeed;

pub struct FeedConfig {}

impl FeedConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hackernews_feed(&self) -> impl FeedFacade {
        Hackernews::new("https://hacker-news.firebaseio.com/v0".to_string(), 10)
    }

    pub fn memory_feed(&self, data: Vec<FeedItem>) -> impl FeedFacade {
        MemoryFeed::new(data)
    }
}
