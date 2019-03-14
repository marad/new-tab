#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod calendar;
pub mod clients;
mod config;
use calendar::Calendar;
use rocket_contrib::serve::StaticFiles;

use crate::clients::google::token_storage::{DiskStorage, TokenStorage};
use crate::clients::google::GoogleClient;
use rocket::State;
use std::str;
use std::sync::Mutex;

//unsafe impl<T:TokenStorage> std::marker::Sync for GoogleClient<T> {}
//unsafe impl<T:TokenStorage> std::marker::Send for GoogleClient<T> {}
//unsafe impl std::marker::Sync for DiskStorage {}
//unsafe impl std::marker::Send for DiskStorage {}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<test>")]
fn hello(test: i32) -> String {
    format!("Hello, {}!", test)
}

#[get("/gcal")]
fn gcal(state: State<Mutex<GoogleClient<DiskStorage>>>) -> String {
    let google_client: GoogleClient<DiskStorage> = state.lock().unwrap().clone();
    let cal = Calendar::new(vec!["moriturius@gmail.com".to_string()], google_client);
    let events = cal.get_events().unwrap();
    println!("Events: ");
    for event in events {
        println!("{:?}", event);
    }
    "hello".to_string()
}

fn main() {
    let config = config::Config::load();

    let google_client = GoogleClient::new(DiskStorage::new(config.tokens_path), config.google_auth);

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
    use std::sync::Arc;

    #[test]
    fn test() -> Result<(), Box<std::error::Error>> {
        let config = config::Config::load();
        let google_client =
            GoogleClient::new(DiskStorage::new(config.tokens_path), config.google_auth);

        let cal = Calendar::new(vec!["moriturius@gmail.com".to_string()], google_client);
        let events = cal.get_events()?;

        println!("Events: ");
        for event in events {
            println!("{:?}", event);
        }

        Ok(())
    }

    fn index2(req: &HttpRequest<Arc<GoogleClient<DiskStorage>>>) -> &'static str {
        //        let  mutex: Mutex<GoogleClient<DiskStorage>> = req.state();
        //        let cal = Calendar::new(vec!["moriturius@gmail.com".to_string()], mutex.lock());
        //        let events= cal.get_events();

        //        for event in events {
        //            println!("{:?}", event);
        //        }

        "Hello World!!!"
    }

    fn index(req: &HttpRequest<Arc<Test>>) -> &'static str {
        "Hello World!!!"
    }

    struct Test {
        pub value: i32,
    }

    //    #[test]
    //    fn actix()  {
    //        let config = config::Config::load();
    //        let google_client = GoogleClient::new(
    //            DiskStorage::new(config.tokens_path),
    //            config.google_auth,
    //        );
    //
    //        let app = App::with_state(Arc::new(Test { value: 32 }))
    //            .resource("/", |r| r.f(index))
    //            .finish();
    //
    //        server::new(|| app)
    //            .bind("127.0.0.1:8088")
    //            .unwrap()
    //            .run()
    //    }
}
