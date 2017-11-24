extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate hyper;


mod db_worker;
mod coinigy_hist;
mod coinigy_live;

fn main() {
    println!("Hello, world!");
}
