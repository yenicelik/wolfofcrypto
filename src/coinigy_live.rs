use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request, Body, Response};
use hyper::header::{ContentLength, ContentType};
use hyper;
use hyper_tls::HttpsConnector;
use rustc_serialize::json::Json;

use serde_json::Value;
use futures;

use std::io;


use std::result;
use serde_json;
use serde_json::Value;
use serde_json::value::from_value;

use types;

use std::{thread, time};
use labrador;

//Static values
static APIKEY: &str = "62fd3c20d21923e7caae5b0bab5771b5";
static APISEC: &str = "5449d3ebbe1fe9dce4a941641c9d1e0e";

/** PREPARATORY FUNCTIONS **/
pub fn default_request(json: &str, uri: hyper::Uri) -> Request<Body> {

    //Add body and json
    let mut req: Request<Body> = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());

    req.headers_mut().append_raw("X-API-KEY", APIKEY.to_owned());
    req.headers_mut().append_raw("X-API-SECRET", APISEC.to_owned());

    req.set_body(json.to_owned());

    return req;

}

pub fn list_markets(client: labrador::Client) -> () { //Vec<types::SingleMarket>
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();
    let request = default_request(json, uri);

    let res: Response<Body> = client.execute(request).wait().unwrap().unwrap();

    println!("Response: {}", res.status());
    println!("Headers: \n{}", res.headers());

    res.body().concat2().and_then(move |body: hyper::Chunk| {
        let v: types::ResponseMarket = serde_json::from_slice(&body).unwrap();
        println!("Got response: {:?}", v);
        //let x: Vec<types::SingleMarket> = types::convert(res.0);
        //println!("Got response2: {:?}", x);
        Ok(())
    });

    /*
    res.body().concat2().map(|body| {
        let val: Vec<u8> = body.to_vec();
        let json_string = String::from_utf8(val).unwrap(); //TODO: get rid of this

<<<<<<< Updated upstream
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
        println!("{:?}", res);
    }
}


/** Request Functions **/


fn before_request() {
    let sleep_time = time::Duration::from_millis(500);
    thread::sleep(sleep_time);
}

fn create_request<ResType, TypeTemp, Type>(json: &str, uri: hyper::Uri) -> Option<Type> {
    //Preparatory statements
    let mut core: Core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

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
            println!("{}", tmp_string);
            let json: Value = Json::from_str(&tmp_string).unwrap(); //TODO: get rid of this unwrap!
            let data = json.get("data").unwrap();
            let mut resp: ResType = from_value(data.clone()).unwrap();
            let x: Type = types::convert(resp.0);
            println!("{:?}", x);
            x
        })
    });
=======
        let json: Value = serde_json::from_str(json_string.as_str()).unwrap();
        let data = json.get("data").unwrap();
        let mut resp: types::ResponseMarket = serde_json::from_value(data.clone()).unwrap();
        println!("{:?}", resp);
        all_markets = types::convert(resp.0);
    });
    */
/*
    println!("Stage 3");
    println!("{:?}", all_markets);
    return all_markets; */
}

#[cfg(test)]
mod tests {
    use labrador::{Client, ClientBuilder};


    //Needs to be inspected once
    #[test]
    fn test_list_markets() {
        use super::list_markets;

        println!("Running list_markets tests");

        let client: Client = ClientBuilder::default().ssl(true).rate(Some(1f32))
            .build();
        let res = list_markets(client);
        println!("{:?}", res);
    }
}