mod config;
mod facade;
mod google_calendar;
mod memory_calendar;

pub use config::*;
pub use facade::CalendarFacade;
pub use facade::Event;

#[cfg(test)]
mod test {
    #[test]
    fn in_memory_calendar() {
        // given the in-memory calendar with some events
    }
}
