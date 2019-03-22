/*
impl<T: TokenStorage> GoogleCalendar for GoogleClient<T> {
    fn get_events(
        &self,
        calendar: &str,
        time_min: &str,
        time_max: &str,
    ) -> Result<CalendarEvents, Box<error::Error>> {
        let token = self.get_access_token(vec![
            "https://www.googleapis.com/auth/calendar.readolly".to_string(),
        ])?;

        let url = dbg!(format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}&timeMax={}&singleEvents=true",
            calendar, time_min, time_max
        ));

        let mut result = dbg!(reqwest::Client::builder()
            .build()?
            .get(&url)
            .header("Authorization", format!("Bearer {}", &token.access_token))
            .send())?;

        Ok(result.json()?)
    }
}

export class HnItem {
  title: string;
  time: Number;
  type: string;
  url: string;
  kids: Number[];
  score: Number;
  id: Number;
}


*/

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

pub struct Hackernews {
    base_url: String,
    item_limit: u32,
}

impl Hackernews {
    pub fn new(base_url: String, item_limit: u32) -> Self {
        Self {
            base_url,
            item_limit,
        }
    }

    pub fn top_stories(&self) -> Result<Vec<u32>, Box<error::Error>> {
        let url = dbg!(format!("{}/topstories.json", &self.base_url));
        let mut result = dbg!(reqwest::Client::builder().build()?.get(&url).send())?;
        let mut item_ids: Vec<u32> = result.json()?;
        item_ids.truncate(self.item_limit as usize);
        let result: Vec<_> = dbg!(item_ids.iter().map(|id| self.get_item(id)).collect());
        Ok(item_ids)
        //Ok(item_ids.map(self.get_item))
    }

    pub fn get_item(&self, item_id: &u32) -> Result<Item, Box<error::Error>> {
        Err(From::from("Hello World"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hn() {
        let hn = Hackernews::new("https://hacker-news.firebaseio.com/v0".to_string(), 10);

        dbg!(hn.top_stories());
    }
}
