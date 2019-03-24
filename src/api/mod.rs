use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::AllowedOrigins;

use crate::calendar::Event;
use crate::clients::hackernews::Item;
use crate::common::*;
use std::sync::{Arc, RwLock};

pub struct Api {}

#[get("/events")]
fn events(app_state: State<SharedAppState>) -> Json<Vec<Event>> {
    // TODO: zamiast error::Error powinien pewnie zwracać jakiś Json<RestError>
    let app_state = app_state.read().unwrap();
    Json(app_state.events.clone())
}

#[get("/feed")]
fn feed(app_state: State<SharedAppState>) -> Json<Vec<Item>> {
    let app_state = app_state.read().unwrap();
    Json(app_state.feed.clone())
}

impl Api {
    pub fn run_server(app_state: Arc<RwLock<AppState>>) {
        let options = rocket_cors::Cors {
            allowed_origins: AllowedOrigins::all(),
            ..Default::default()
        };

        rocket::ignite()
            .manage(app_state)
            .mount("/static/", StaticFiles::from("static/"))
            .mount("/", routes![events, feed])
            .attach(options)
            .launch();
    }
}
