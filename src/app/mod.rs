use std::error;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};

use crate::api::Api;
use crate::calendar::Calendar;
use crate::clients::google::{DiskStorage, GoogleClient};
use crate::common::*;
use crate::config;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> Result<(), Box<error::Error>> {
        let context = self.create_context();

        let app_state = Arc::new(RwLock::new(AppState {
            events: context.calendar.get_events()?,
        }));

        self.start_scheduler(&context, &app_state);
        Api::run_server(app_state);
        Ok(())
    }

    fn create_context(&self) -> AppContext {
        let config = config::Config::load();

        let google_client = GoogleClient::new(
            DiskStorage::new(config.tokens_path.clone()),
            config.google_auth.clone(),
        );
        let calendar = Calendar::new(config.calendars.clone(), google_client);

        AppContext { config, calendar }
    }

    fn start_scheduler(
        &self,
        context: &AppContext,
        app_state: &Shared<AppState>,
    ) -> ScheduleHandle {
        let app_state = app_state.clone();
        let calendar = context.calendar.clone();

        let mut scheduler = Scheduler::new();
        scheduler.every(5.minutes()).run(move || {
            let mut app_state = app_state.write().unwrap();

            app_state.events = calendar.get_events().unwrap();
        });

        scheduler.watch_thread(Duration::from_millis(100))
    }
}
