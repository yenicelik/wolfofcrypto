#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate iron;
#[macro_use]
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate hyper_tls;
extern crate rustc_serialize;


mod db_worker;
mod coinigy_hist;
mod coinigy_live;
mod types;

fn main() {
    println!("Hello, world!");
}