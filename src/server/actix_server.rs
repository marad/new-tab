use super::ServerFacade;

use crate::calendar::Event;
use crate::common::*;
use crate::feed::FeedItem;

use actix_web::{web, App, HttpServer};
use failure::Fallible;

fn events(data: web::Data<SharedAppState>) -> web::Json<Vec<Event>> {
    web::Json(data.read().unwrap().events.clone())
}

fn feed(data: web::Data<SharedAppState>) -> web::Json<Vec<FeedItem>> {
    web::Json(data.read().unwrap().feed.clone())
}

pub struct ActixServer;

impl ActixServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerFacade for ActixServer {
    fn start_server(&self, app_state: SharedAppState) -> Fallible<()> {
        HttpServer::new(move || {
            App::new()
                .data(app_state.clone())
                .route("/events", web::get().to(events))
                .route("/feed", web::get().to(feed))
        })
        .bind("0.0.0.0:8000")?
        .run()?;
        Ok(())
    }
}
