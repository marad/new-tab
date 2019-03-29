use super::facade::{CalendarFacade, Event};
use std::error;

pub struct InMemoryCalendar {
    events: Vec<Event>,
}

impl InMemoryCalendar {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }
}

impl CalendarFacade for InMemoryCalendar {
    fn get_events(&self) -> Result<Vec<Event>, Box<error::Error>> {
        Ok(self.events.clone())
    }
}
