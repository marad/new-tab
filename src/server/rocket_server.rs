use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::AllowedOrigins;

use super::facade::ServerFacade;
use crate::calendar::Event;
use crate::common::*;
use crate::feed::FeedItem;
use std::error;

#[get("/events")]
fn events(app_state: State<SharedAppState>) -> Json<Vec<Event>> {
    // TODO: zamiast error::Error powinien pewnie zwracać jakiś Json<RestError>
    let app_state = app_state.read().unwrap();
    Json(app_state.events.clone())
}

#[get("/feed")]
fn feed(app_state: State<SharedAppState>) -> Json<Vec<FeedItem>> {
    let app_state = app_state.read().unwrap();
    Json(app_state.feed.clone())
}

pub struct RocketServer {}

impl RocketServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerFacade for RocketServer {
    fn start_server(&self, app_state: SharedAppState) -> Result<(), Box<error::Error>> {
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
        Ok(())
    }
}
