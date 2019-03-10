use crate::clients::google::calendar::*;
use crate::clients::google::GoogleClient;
use chrono::prelude::*;
use std::convert::From;
use std::error;
use time::Duration;

#[derive(Debug, Clone)]
pub struct Event {
    pub summary: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
}

impl From<&CalendarEvent> for Event {
    fn from(ce: &CalendarEvent) -> Self {
        Event {
            summary: ce.summary.clone(),
            location: ce.location.clone(),
            description: ce.description.clone(),
            start_time: ce.start.date_time.clone(),
            end_time: ce.end.date_time.clone(),
        }
    }
}

pub struct Calendar {
    calendars: Vec<String>,
    google_client: GoogleClient,
}

impl Calendar {
    pub fn new(calendars: Vec<String>, google_client: GoogleClient) -> Self {
        Self {
            calendars,
            google_client,
        }
    }

    pub fn get_events(&mut self) -> Result<Vec<Event>, Box<error::Error>> {
        let start = Utc::now();
        let end = start + Duration::weeks(1);

        self.google_client
            .get_events(
                self.calendars.first().unwrap(), // TODO: fetch events from multiple calendars
                &start.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                &end.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            )
            .map(|result| result.items.iter().map(|e| Event::from(e)).collect())
    }
}
