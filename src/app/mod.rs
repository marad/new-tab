use std::error;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};

use crate::calendar::CalendarConfig;
use crate::common::*;
use crate::config;
use crate::feed::FeedConfig;
use crate::server::{ServerConfig, ServerFacade};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> Result<(), Box<error::Error>> {
        let context = self.create_context();

        let app_state = Arc::new(RwLock::new(AppState {
            events: context.calendar.get_events()?,
            feed: context.feed.get_items()?,
        }));

        let context = Arc::new(RwLock::new(context));
        let _scheduler = self.start_scheduler(&context, &app_state);
        ServerConfig::new()
            .rocket_server(&app_state)
            .start_server()?;
        Ok(())
    }

    fn create_context(&self) -> AppContext {
        let config = config::Config::load();

        let calendar = Box::new(CalendarConfig::new().google_calendar(&config));
        let feed = Box::new(FeedConfig::new().hackernews_feed());

        AppContext {
            feed,
            config,
            calendar,
        }
    }

    fn start_scheduler(
        &self,
        context: &Arc<RwLock<AppContext>>,
        app_state: &Arc<RwLock<AppState>>,
    ) -> ScheduleHandle {
        let mut scheduler = Scheduler::new();

        {
            let context = context.clone();
            let state = app_state.clone();
            scheduler.every(5.minutes()).run(move || {
                let calendar = &context.read().unwrap().calendar;
                let mut state = state.write().unwrap();

                println!("Updating calendar events...");

                match calendar.get_events() {
                    Ok(events) => state.events = events,
                    Err(err) => eprintln!("Error while updating calendar events: {}", err),
                }
            });
        }

        {
            let context = context.clone();
            let state = app_state.clone();
            scheduler.every(5.minutes()).run(move || {
                let feed = &context.read().unwrap().feed;
                let mut state = state.write().unwrap();

                println!("Updating feed items...");

                match feed.get_items() {
                    Ok(items) => state.feed = items,
                    Err(err) => eprintln!("Error while updating feed items: {}", err),
                }
            });
        }

        scheduler.watch_thread(Duration::from_millis(100))
    }
}
