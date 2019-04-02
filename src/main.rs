#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

mod app;
mod calendar;
mod clients;
mod common;
mod config;
mod feed;
mod server;

use crate::app::App;
use std::error;

fn main() -> Result<(), Box<error::Error>> {
    App::new().start()
}
