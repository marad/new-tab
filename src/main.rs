#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod calendar;
pub mod clients;
mod config;
use calendar::{Calendar, Event};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::AllowedOrigins;

use crate::clients::google::token_storage::DiskStorage;
use crate::clients::google::GoogleClient;
use rocket::State;
use std::error;
use std::sync::{Arc, RwLock};

use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;

struct AppState {
    pub events: Vec<Event>,
}
type SharedAppState = Arc<RwLock<AppState>>;

#[get("/events")]
fn events(app_state: State<SharedAppState>) -> Result<Json<Vec<Event>>, Box<error::Error>> {
    // TODO: zamiast error::Error powinien pewnie zwracać jakiś Json<RestError>
    let app_state = app_state.read().unwrap();
    Ok(Json(app_state.events.clone()))
}

fn main() -> Result<(), Box<error::Error>> {
    let config = config::Config::load();

    let google_client = GoogleClient::new(
        DiskStorage::new(config.tokens_path.clone()),
        config.google_auth.clone(),
    );
    let calendar = Calendar::new(config.calendars.clone(), google_client);

    let app_state = AppState {
        events: calendar.get_events()?,
    };

    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    };

    let shared_calendar = Arc::new(RwLock::new(calendar));
    let shared_app_state = Arc::new(RwLock::new(app_state));

    let lambda_calendar = shared_calendar.clone();
    let lambda_app_state = shared_app_state.clone();
    let mut scheduler = Scheduler::new();
    scheduler.every(5.minutes()).run(move || {
        let mut app_state = lambda_app_state.write().unwrap();
        let calendar = lambda_calendar.read().unwrap();

        app_state.events = calendar.get_events().unwrap();
    });

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    rocket::ignite()
        .manage(shared_app_state)
        .manage(config)
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/", routes![events])
        .attach(options)
        .launch();

    thread_handle.stop();
    Ok(())
}
