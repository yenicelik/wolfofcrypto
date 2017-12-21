#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="4096"]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel_infer_schema;
extern crate iron;
pub extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate failure;

extern crate r2d2_diesel;
extern crate r2d2;


extern crate rocket;
extern crate rocket_contrib;

//Packages
mod coinigy;
mod coinmarketcap;
mod database;
mod historical;
mod server;

// Files
mod utils;
mod types;

// Functionality
use std::time::Duration;
use std::thread;
use historical::collect_historical_data;

static SERVE_ONLY: bool = true;

fn main() {

    if SERVE_ONLY {
        server::server::serve();
    } else {

        let server = thread::Builder::new().name("Server".to_string());
        server.spawn(move || {
            server::server::serve();
        });

        /*
        let dataCollector = thread::Builder::new().name("Data Collector".to_string());
        dataCollector.spawn(move || {
            loop {
                println!("Looping through!");
                thread::sleep(Duration::from_millis(5000));
                collect_historical_data::get_website_data();
            }
        });
        */
    }

}
