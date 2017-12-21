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

fn main() {
    server::server::serve();
}
