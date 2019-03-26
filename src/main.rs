#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod api;
mod app;
mod calendar;
mod clients;
mod common;
mod config;
mod feed;

use crate::app::App;
use std::error;

fn main() -> Result<(), Box<error::Error>> {
    App::new().start()
}
