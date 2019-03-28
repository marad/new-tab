use std::error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    url: String,
    title: String,
}

impl FeedItem {
    pub fn new(url: &impl ToString, title: &impl ToString) -> Self {
        Self {
            url: url.to_string(),
            title: title.to_string(),
        }
    }
}

pub trait FeedFacade: Send + Sync {
    fn get_items(&self) -> Result<Vec<FeedItem>, Box<error::Error>>;
}
