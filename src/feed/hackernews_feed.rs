use super::{FeedFacade, FeedItem};
use failure::Fallible;
use rayon::prelude::*;
use std::convert::From;
use std::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub title: String,
    pub time: i64,
    pub r#type: String,
    pub url: String,
    pub kids: Vec<u32>,
    pub score: i32,
    pub id: u32,
}

impl From<&Item> for FeedItem {
    fn from(item: &Item) -> Self {
        Self::new(&item.url, &item.title)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hackernews {
    base_url: String,
    item_limit: u32,
}

impl Hackernews {
    #[allow(dead_code)]
    pub fn new(base_url: String, item_limit: u32) -> Self {
        Self {
            base_url,
            item_limit,
        }
    }

    pub fn top_stories(&self) -> Fallible<Vec<Item>> {
        let url = format!("{}/topstories.json", &self.base_url);
        let mut result = reqwest::Client::builder().build()?.get(&url).send()?;
        let mut item_ids: Vec<u32> = result.json()?;
        item_ids.truncate(self.item_limit as usize);
        Ok(item_ids
            .par_iter()
            .filter_map(|id| Result::ok(self.get_item(id)))
            .collect())
    }

    fn get_item(&self, item_id: &u32) -> Result<Item, Box<error::Error>> {
        let url = dbg!(format!("{}/item/{}.json", &self.base_url, item_id));
        let mut result = reqwest::Client::builder().build()?.get(&url).send()?;
        Ok(result.json()?)
    }
}

impl FeedFacade for Hackernews {
    fn get_items(&self) -> Fallible<Vec<FeedItem>> {
        Ok(self
            .top_stories()?
            .iter()
            .map(|item: &Item| From::from(item))
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hn() {
        let hn = Hackernews::new("https://hacker-news.firebaseio.com/v0".to_string(), 10);
        let _ = dbg!(hn.top_stories());
    }
}
