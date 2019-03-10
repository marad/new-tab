#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod config;
mod calendar;
pub mod clients;
use rocket_contrib::serve::StaticFiles;

use std::str;
use crate::clients::google::token_storage::DiskStorage;
use rocket::State;
use std::sync::Mutex;
use crate::clients::google::GoogleClient;

unsafe impl std::marker::Sync for GoogleClient {}
unsafe impl std::marker::Send for GoogleClient {}
unsafe impl std::marker::Sync for DiskStorage{}
unsafe impl std::marker::Send for DiskStorage{}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<test>")]
fn hello(test: i32) -> String {
    format!("Hello, {}!", test)
}

#[get("/gcal")]
fn gcal(state: State<Mutex<GoogleClient>>) -> String {
    let mut google_client = state.lock().unwrap();
    calendar::playground(&mut google_client);
    "hello".to_string()
}

fn main() {
    let config = config::Config::load();

    let google_client = GoogleClient::new(
        Box::new(DiskStorage::new(config.tokens_path)),
        config.google_auth
    );

    let mutex = Mutex::new(google_client);

    rocket::ignite()
        .manage(mutex)
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/", routes![index, hello, gcal])
        .launch();
}
