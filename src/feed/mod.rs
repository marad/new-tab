mod config;
mod facade;
mod hackernews_feed;
mod memory_feed;

pub use config::*;
pub use facade::FeedFacade;
pub use facade::FeedItem;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memory_feed_works() -> Result<(), Box<std::error::Error>> {
        // given example items and the in-memory feed
        let item1 = FeedItem::new(&"url1", &"title1");
        let item2 = FeedItem::new(&"url2", &"title2");
        let feed = memory(vec![item1.clone(), item2.clone()]);

        // when fetching items
        let result = dbg!(feed.get_items())?;

        // example items are returned
        assert_eq!(&result[0], &item1);
        assert_eq!(&result[1], &item2);
        Ok(())
    }
}
