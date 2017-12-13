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

use std::{thread, time};

//Static values
static APIKEY: &str = "62fd3c20d21923e7caae5b0bab5771b5";
static APISEC: &str = "5449d3ebbe1fe9dce4a941641c9d1e0e";

//Needs to be inspected once
pub fn list_markets() -> () {
    before_request();
    //Web page to ping for
    let json: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();
    let res = create_request<types::ResponseMarket, Vec<types::SingleMarket> >(json, uri);

}

#[cfg(test)]
mod tests {
    //Needs to be inspected once
    #[test]
    fn test_list_markets() {
        use super::list_markets;
        list_markets();
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

fn before_request() {
    let sleep_time = time::Duration::from_millis(500);
    thread::sleep(sleep_time);
}

fn create_request<ResType, OutType>(json: &str, uri: hyper::Uri) -> Option<OutType> {
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
            let tmp_string: String = String::from_utf8(val).unwrap();
            println!("{}", tmp_string);
            let json: serde_json::Value = Json::from_str(&tmp_string).unwrap();
            let data = json.get("data").unwrap();
            let mut resp: ResType = serde_json::from_value(data.clone()).unwrap();
            let x: OutType = types::convert(resp.0);
            x
        })
    });

    match core.run(post) {
        Ok(response) => {
            Some(response)
        }
        Err(err) => {
            //Log on failure!!!
            debug!("Debug error");
            None
        }
    }
}