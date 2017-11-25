use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper;
use hyper_tls::HttpsConnector;
use rustc_serialize::json::Json;

use std::result;
use serde_json;
use types;

//Static values
static APIKEY: &str = "62fd3c20d21923e7caae5b0bab5771b5";
static APISEC: &str = "5449d3ebbe1fe9dce4a941641c9d1e0e";

//Needs to be inspected once
pub fn list_markets() -> () {
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();
    let res = create_request(json, uri);
    print!("{:?}", res);
}

pub fn list_exchanges() -> () {
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/exchanges".parse().unwrap();
    let res = create_request(json, uri);
    print!("{:?}", res);
}

pub fn get_order_types() -> () {
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/orderTypes".parse().unwrap();
    let res = create_request(json, uri);
    print!("{:?}", res);
}


//Needs to be called once
pub fn get_auth_id() -> () {
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/accounts".parse().unwrap();
    let res = create_request(json, uri);
    print!("{:?}", res);
}

pub fn list_orders() -> () {
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/orders".parse().unwrap();
    let res = create_request(json, uri);
    print!("{:?}", res);
}

#[cfg(test)]
mod tests {

    //Needs to be inspected once
    #[test]
    fn test_list_markets() {
        use super::list_markets;
        list_markets()
    }

    #[test]
    fn test_list_exchanges() {
        use super::list_exchanges;
        list_exchanges();
    }

    #[test]
    fn test_get_order_types() {
        use super::get_order_types;
        get_order_types();
    }

    //Needs to be called once
    #[test]
    fn test_get_auth_id() {
        use super::get_auth_id;
        get_auth_id();
    }

    //Needs to be called frequently
    #[test]
    fn test_list_orders() {
        use super::list_orders;
        list_orders();
    }


    //Building blocks
    #[test]
    fn test_create_request() {
        use super::create_request;
        use hyper;
        use hyper::Body;

        //Web page to ping for
        let json: &str = r#"{}"#;
        let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();
        let res = create_request(json, uri);
        print!("{:?}", res);
    }
}


fn create_request(json: &str, uri: hyper::Uri) -> Option<Json> {
    //Preparatory statements
    let mut core: Core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    //    let client = Client::new(&core.handle());

    //Add body and json
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());

    req.headers_mut().append_raw("X-API-KEY", APIKEY.to_owned());
    req.headers_mut().append_raw("X-API-SECRET", APISEC.to_owned());


    req.set_body(json.to_owned());

    let post = client.request(req).and_then(|res| {
        res.body().concat2().map(|body| {
            let val: Vec<u8> = body.to_vec();
            let tmp_string: String = String::from_utf8(val).unwrap(); //TODO: get rid of this unwrap
            let json: Json = Json::from_str(&tmp_string).unwrap(); //TODO: get rid of this unwrap!
            json
        })

    });

    match core.run(post) {
        Ok(response) => {
            Some(response)
        }
        Err(err) => {
            None
        }
    }
}

/*
//Needs to be called frequently
pub fn update_balance() -> () {} //Needs auth_id

pub fn add_order() -> () {}  //Needs auth_id

pub fn cancel_order() -> () {}  //Needs auth_id


pub fn list_balances() -> () {}  //Needs auth_id

*/