use super::facade::{FeedFacade, FeedItem};
use super::hackernews_feed::Hackernews;
use super::memory_feed::MemoryFeed;

pub fn hackernews() -> impl FeedFacade {
    Hackernews::new("https://hacker-news.firebaseio.com/v0".to_string(), 10)
}

pub fn memory(data: Vec<FeedItem>) -> impl FeedFacade {
    MemoryFeed::new(data)
}
