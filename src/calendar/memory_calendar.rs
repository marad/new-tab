use super::facade::{CalendarFacade, Event};
use failure::Fallible;

pub struct InMemoryCalendar {
    events: Vec<Event>,
}

impl InMemoryCalendar {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }
}

impl CalendarFacade for InMemoryCalendar {
    fn get_events(&self) -> Fallible<Vec<Event>> {
        Ok(self.events.clone())
    }
}
