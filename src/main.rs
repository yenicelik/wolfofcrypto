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

extern crate rocket;
extern crate rocket_contrib;

mod db_worker;
mod coinigy_live;
mod types;
mod server;
mod database;
mod historical;

fn main() {
    server::server::serve();
}
