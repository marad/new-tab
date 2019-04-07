pub mod client;

use client::calendar::*;
use client::token_storage::TokenStorage;
use client::GoogleClient;

use chrono::prelude::*;
use failure::Fallible;
use std::convert::From;
use time::Duration;

use super::{CalendarFacade, Event};

#[derive(Debug, Clone)]
pub struct Calendar<T: TokenStorage> {
    calendars: Vec<String>,
    google_client: GoogleClient<T>,
}

impl<T: TokenStorage> Calendar<T> {
    pub fn new(calendars: Vec<String>, google_client: GoogleClient<T>) -> Self {
        Self {
            calendars,
            google_client,
        }
    }
}

impl<T: TokenStorage> CalendarFacade for Calendar<T> {
    fn get_events(&self) -> Fallible<Vec<Event>> {
        let start = dbg!(Utc::now());
        let end = dbg!(start + Duration::weeks(1));

        let mut all_events: Vec<Event> = Vec::new();

        for calendar in &self.calendars {
            let mut events: Vec<Event> = self
                .google_client
                .get_events(
                    calendar,
                    &start.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    &end.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                )
                .map(|result| result.items.iter().map(Event::from).collect())?;
            all_events.append(&mut events);
        }

        Ok(all_events)
    }
}

impl From<&CalendarEvent> for Event {
    fn from(ce: &CalendarEvent) -> Self {
        Event {
            summary: ce.summary.clone(),
            location: ce.location.clone(),
            description: ce.description.clone(),
            start_time: match &ce.start {
                Some(start_time) => start_time.date_time.clone(),
                None => None,
            },
            end_time: match &ce.end {
                Some(end_time) => end_time.date_time.clone(),
                None => None,
            },
        }
    }
}
