use serde_json::{Value, Error};
use hyper::client::Client;
use hyper::header::Headers;
use std::thread::sleep;
use std::time;

//Static values
static APIKEY: &str = "62fd3c20d21923e7caae5b0bab5771b5";
static APISEC: &str = "5449d3ebbe1fe9dce4a941641c9d1e0e";

//Needs to be inspected once
pub fn list_markets() -> () {
    let mut client = Client::new();
    let res = client.post("https://example.domain/path")
        .body("foo=bar")
        .send();
    match res {
        Ok(res) => println!("Response: {}", res.status),
        Err(e) => println!("Err: {:?}", e)
    }

}

pub fn list_exchanges() -> () {

}

pub fn get_order_types() -> () {

}

//Needs to be called once
pub fn get_auth_id() -> () {

}

//Needs to be called frequently
pub fn update_balance() -> () {

}

pub fn add_order() -> () {

}

pub fn cancel_order() -> () {

}

pub fn get_user_info() -> () {

}

pub fn list_orders() -> () {

}

pub fn list_balances() -> () {

}

//Util functions
pub fn before_request() -> () {
    let mut headers = Headers::new();
    headers.set_raw("Content-Type", vec![b"application/json".to_vec()]);
    headers.set_raw("X-API-KEY", vec![b"62fd3c20d21923e7caae5b0bab5771b5".to_vec()]);
    headers.set_raw("X-API-SEC", vec![b"5449d3ebbe1fe9dce4a941641c9d1e0e".to_vec()]);

    let wait_time = time::Duration::from_millis(1000);
    sleep(wait_time);

    return

}