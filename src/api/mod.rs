use std::error;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::AllowedOrigins;

use crate::calendar::Event;
use crate::common::*;

pub struct Api {}

#[get("/events")]
fn events(app_state: State<SharedAppState>) -> Result<Json<Vec<Event>>, Box<error::Error>> {
    // TODO: zamiast error::Error powinien pewnie zwracać jakiś Json<RestError>
    let app_state = app_state.read().unwrap();
    Ok(Json(app_state.events.clone()))
}

impl Api {
    pub fn run_server(app_state: Shared<AppState>) {
        let options = rocket_cors::Cors {
            allowed_origins: AllowedOrigins::all(),
            ..Default::default()
        };

        rocket::ignite()
            .manage(app_state)
            .mount("/static/", StaticFiles::from("static/"))
            .mount("/", routes![events])
            .attach(options)
            .launch();
    }
}
