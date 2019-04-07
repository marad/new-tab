#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod app;
mod calendar;
mod common;
mod config;
mod feed;
mod server;

use crate::app::App;
use failure::Fallible;

fn main() -> Fallible<()> {
    App::new().start()
}
