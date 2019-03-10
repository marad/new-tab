use serde::{Serialize, Deserialize};
use crate::clients::google::GoogleClient;
use std::error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attendee {
    pub display_name: String,
    pub email: String,
    pub response_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organizer {
    pub display_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarTime {
    pub date_time: String,
    pub time_zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: String,
    pub status: String,
    pub html_link: String,
    pub summary: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub organizer: Organizer,
    pub start: CalendarTime,
    pub end: CalendarTime,
    pub attendees: Option<Vec<Attendee>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvents {
    pub summary: String,
    pub time_zone: String,
    pub next_sync_token: String,
    pub items: Vec<CalendarEvent>,
}

pub trait GoogleCalendar {
    fn get_events(&mut self, calendar: &str, time_min: &str, time_max: &str) -> Result<CalendarEvents, Box<error::Error>>;
}

impl GoogleCalendar for GoogleClient {
    fn get_events(&mut self, calendar: &str, time_min: &str, time_max: &str) -> Result<CalendarEvents, Box<error::Error>> {
        let token = self.get_access_token(vec![
            "https://www.googleapis.com/auth/calendar.events.readonly".to_string()
        ])?;

        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}&timeMax={}",
            calendar, time_min, time_max
        );

        let mut result = reqwest::Client::builder()
            .build()?
            .get(&url)
            .header("Authorization", format!("Bearer {}", &token.access_token))
            .send()?;

        Ok(result.json()?)
    }
}
