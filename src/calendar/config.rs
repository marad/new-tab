use crate::clients::google::{DiskStorage, GoogleClient};
use crate::config::Config;

use super::facade::*;
use super::google_calendar::Calendar;
use super::memory_calendar::InMemoryCalendar;

pub struct CalendarConfig {}

impl CalendarConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn memory_calendar(&self, events: Vec<Event>) -> impl CalendarFacade {
        InMemoryCalendar::new(events)
    }

    pub fn google_calendar(&self, config: &Config) -> impl CalendarFacade {
        let google_client = GoogleClient::new(
            DiskStorage::new(config.tokens_path.clone()),
            config.google_auth.clone(),
        );

        Calendar::new(config.calendars.clone(), google_client)
    }
}
