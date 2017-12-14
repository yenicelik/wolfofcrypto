extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate iron;
pub extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate rustc_serialize;
extern crate log;
extern crate fern;
#[macro_use]
extern crate failure;


mod db_worker;
mod coinigy_hist;
mod coinigy_live;
mod types;
mod server;

fn main() {

    println!("Hello, world!");

    server::serve();

}
