use crate::config::Config;

use super::facade::*;
use super::google_calendar::client::{DiskStorage, GoogleClient};
use super::google_calendar::Calendar;
use super::memory_calendar::InMemoryCalendar;

pub fn memory_calendar(events: Vec<Event>) -> impl CalendarFacade {
    InMemoryCalendar::new(events)
}

pub fn google_calendar(config: &Config) -> impl CalendarFacade {
    let google_client = GoogleClient::new(
        DiskStorage::new(config.tokens_path.clone()),
        config.google_auth.clone(),
    );

    Calendar::new(config.calendars.clone(), google_client)
}
