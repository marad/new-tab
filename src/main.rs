#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod calendar;
pub mod clients;
mod config;
use rocket_contrib::serve::StaticFiles;

use crate::clients::google::token_storage::DiskStorage;
use crate::clients::google::GoogleClient;
use rocket::State;
use std::str;
use std::sync::Mutex;

unsafe impl std::marker::Sync for GoogleClient {}
unsafe impl std::marker::Send for GoogleClient {}
unsafe impl std::marker::Sync for DiskStorage {}
unsafe impl std::marker::Send for DiskStorage {}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<test>")]
fn hello(test: i32) -> String {
    format!("Hello, {}!", test)
}

#[get("/gcal")]
fn gcal(_state: State<Mutex<GoogleClient>>) -> String {
//    let mut google_client = state.lock().unwrap();
//    calendar::playground(&mut google_client);
    "hello".to_string()
}

fn main() {
    let config = config::Config::load();

    let google_client = GoogleClient::new(
        Box::new(DiskStorage::new(config.tokens_path)),
        config.google_auth,
    );

    let mutex = Mutex::new(google_client);

    rocket::ignite()
        .manage(mutex)
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/", routes![index, hello, gcal])
        .launch();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::clients::google::calendar::*;

    use calendar::Calendar;

    use actix_web::{server, App, HttpRequest};

    #[test]
    fn test() -> Result<(), Box<std::error::Error>> {
        let config = config::Config::load();
        let google_client = GoogleClient::new(
            Box::new(DiskStorage::new(config.tokens_path)),
            config.google_auth,
        );

        let cal = Calendar::new(vec!["moriturius@gmail.com".to_string()], google_client);
        let events = cal.get_events();

        for event in events {
            println!("{:?}", event);
        }

        Ok(())
    }

    fn index(_req: &HttpRequest) -> &'static str {
        "Hello World!!!"
    }

    #[test]
    fn actix()  {
        server::new(|| App::new().resource("/", |r| r.f(index)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run()

    }
}
