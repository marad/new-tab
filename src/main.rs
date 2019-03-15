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

use crate::clients::google::token_storage::DiskStorage;
use crate::clients::google::GoogleClient;
use rocket::State;
use std::error;
use std::sync::Mutex;

#[get("/events")]
fn events(
    state: State<Mutex<Calendar<DiskStorage>>>,
) -> Result<Json<Vec<Event>>, Box<error::Error>> {
    // TODO: zamiast error::Error powinien pewnie zwracać jakiś Json<RestError>
    match state.lock() {
        Ok(calendar) => Ok(Json(calendar.get_events()?)),
        _ => Err(From::from("Error while getting events...")),
    }
}

fn main() {
    let config = config::Config::load();

    let google_client = GoogleClient::new(
        DiskStorage::new(config.tokens_path.clone()),
        config.google_auth.clone(),
    );
    let calendar = Calendar::new(config.calendars.clone(), google_client);

    rocket::ignite()
        .manage(Mutex::new(calendar))
        .manage(config)
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/", routes![events])
        .launch();
}
