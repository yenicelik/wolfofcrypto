use std::{thread, time};
use tokio_core::reactor::Core;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde_json;
use serde_json::Value;
use failure::Error;
use futures::{Future, Stream};

const RATE_LIMIT_CENTISECONDS: u64 = 10;

pub fn send_request(currency: &str, start: i64, offset: i64) -> Result<Value, Error> {
    let wait_time = time::Duration::from_millis(RATE_LIMIT_CENTISECONDS * 100); //1sec
    thread::sleep(wait_time);

    let query_string = format!("{}{}{}{}{}{}{}",
                               "https://graphs.coinmarketcap.com/currencies/",
                               currency, "/",
                               start, "/",
                               start + offset, "/"
    );
    println!("Calling: {} ...", query_string);

    //Setting up the request shit
    let mut core: Core = Core::new().unwrap(); //TODO: move this out of this function
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = query_string.parse().unwrap(); //Should i do error handling here?

    let work = client.get(uri).and_then(|res| {
        res.body().concat2().map(|body| {
            let val: Vec<u8> = body.to_vec();
            let json_string: String = String::from_utf8(val).unwrap();
            let json: Value = serde_json::from_str(&json_string.to_string()).unwrap();
            json
        })
    });

    match core.run(work) {
        Ok(x) => Ok(x),
        Err(err) => Err(format_err!("Get failed with message {:?}", err))
    }
}