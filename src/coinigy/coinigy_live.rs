use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentType};
use hyper;
use hyper_tls::HttpsConnector;

use serde_json;
use serde_json::{Value};
use coinigy::types;

use std::{thread, time};

use failure::Error;


/** PREPARATORY FUNCTIONS **/
//Static values
static APIKEY: &str = "62fd3c20d21923e7caae5b0bab5771b5";
static APISEC: &str = "5449d3ebbe1fe9dce4a941641c9d1e0e";

pub fn create_post(json: &str, uri: hyper::Uri) -> Result<Value, Error> {

    let time2sleep = time::Duration::from_millis(500);
    thread::sleep(time2sleep);

    // Create Request, and add body and json
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().append_raw("X-API-KEY", APIKEY.to_owned());
    req.headers_mut().append_raw("X-API-SECRET", APISEC.to_owned());
    req.set_body(json.to_owned());

    // Create tokio core
    let mut core: Core = Core::new().unwrap(); //TODO: move this out of this function
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    // Execute core with request
    let post = client.request(req).and_then(|res| {
        res.body().concat2().map(|body| {
            let val: Vec<u8> = body.to_vec();
            let json_string: String = String::from_utf8(val).unwrap();
            let json: Value = serde_json::from_str(&json_string.to_string()).unwrap();
            json
        })
    });

    match core.run(post) {
        Ok(val) => Ok(val),
        //Need this to reformat the error into a new type
        Err(err) => Err(format_err!("Post failed with message {:?}", err))
    }

}

/** API CALLS **/
//TODO: could be put into one function with templates

/** Does not require auth_id **/
//Needs to be inspected once
pub fn list_markets() -> Result<Vec<types::SingleMarket>, Error> {

    //Requesting
    let request_body: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseMarket = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleMarket> = types::convert(resp.0);

    Ok(x)
}

pub fn list_exchanges() -> Result<Vec<types::SingleExchange>, Error> {

    //Requesting
    let request_body: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/exchanges".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseExchange = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleExchange> = types::convert(resp.0);

    Ok(x)
}

pub fn get_order_types() -> Result<Vec<types::SingleOrderType>, Error> {

    //Requesting
    let request_body: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/orderTypes".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let order_types = match data.get("order_types") {
        Some(val) => Ok(val),
        None => Err(format_err!("order_types was not found within the struct!"))
    }?;
    let resp: types::ResponseOrderType = serde_json::from_value(order_types.clone())?;
    let x: Vec<types::SingleOrderType> = types::convert(resp.0);

    Ok(x)
}


//Needs to be called once
pub fn get_auth_id() -> Result<Vec<types::SingleAccount>, Error> {

    //Requesting
    let request_body: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/accounts".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseAccount = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleAccount> = types::convert(resp.0);

    Ok(x)
}

pub fn list_orders() -> Result<Vec<types::SingleOrder>, Error> {

    //Requesting
    let request_body: &str = r#"{}"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1/orders".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let open_orders = match data.get("open_orders") {
        Some(val) => Ok(val),
        None => Err(format_err!("open_orders was not found within the struct!"))
    }?;
    let resp: types::ResponseOrder = serde_json::from_value(open_orders.clone())?;
    let x: Vec<types::SingleOrder> = types::convert(resp.0);

    Ok(x)

}

/** Functions that rely on auth_id **/
//Needs to be called frequently
pub fn refresh_balance(inp_auth_id: String) -> Result<Vec<types::SingleRefreshBalance>, Error> {
    //Requesting
    let request_body: &str = &json!({
        "auth_id": inp_auth_id
    }).to_string().to_owned();

    let uri: hyper::Uri = "https://api.coinigy.com/api/v1//refreshBalance".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    println!("{:?}", value);
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseRefreshBalance = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleRefreshBalance> = types::convert(resp.0);

    Ok(x)
}


pub fn list_balances(inp_auth_id: String) -> Result<Vec<types::SingleBalance>, Error> {
    //Requesting
    let request_body: &str = &json!({
        "show_nils": 0,
        "auth_id": inp_auth_id
    }).to_string().to_owned();

    let uri: hyper::Uri = "https://api.coinigy.com/api/v1//balances".parse().unwrap();

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseBalance = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleBalance> = types::convert(resp.0);

    Ok(x)
}

#[cfg(test)]
mod tests {

    /** Does not require a request_body **/
    //Building blocks
    #[test]
    fn test_create_request() {
        use super::create_post;
        use hyper;

        //Web page to ping for
        let request_body: &str = r#"{}"#;
        let uri: hyper::Uri = "https://api.coinigy.com/api/v1/markets".parse().unwrap();
        create_post(request_body, uri).unwrap();
    }

    //Needs to be inspected once
    #[test]
    fn test_list_markets() {
        use super::list_markets;
        match list_markets() {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }

    #[test]
    fn test_list_exchanges() {
        use super::list_exchanges;
        match list_exchanges() {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }

    #[test]
    fn test_get_order_types() {
        use super::get_order_types;
        match get_order_types() {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }

    //Needs to be called once
    #[test]
    fn test_get_auth_id() {
        use super::get_auth_id;
        match get_auth_id() {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };    }

    //Needs to be called frequently
    #[test]
    fn test_list_orders() {
        use super::list_orders;
        match list_orders() {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }

    #[test]
    fn test_list_balances() {
        // TODO: Run again when balances are on!
        use super::list_balances;
        use super::get_auth_id;

        let x = match get_auth_id() {
            Ok(x) => x,
            Err(err) => panic!("Already failed while getting id! {:?}", err)
        };

        println!("Auth id is: {:?}", (&x[0]).auth_id.to_string());

        match list_balances((&*(&x[0]).auth_id).to_owned()) {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }

    /*
    #[test]
    fn test_refresh_balances() {
        // TODO: Run again when balances are on!
        use super::refresh_balance;
        use super::get_auth_id;

        let x = match get_auth_id() {
            Ok(x) => x,
            Err(err) => panic!("Already failed while getting id! {:?}", err)
        };

        println!("Auth id is: {:?}", (&x[0]).auth_id.to_string());

        match refresh_balance((&*(&x[0]).auth_id).to_owned()) {
            Ok(_) => {},
            Err(err) => panic!("{:?}", err)
        };
    }
    */
}


/* Soon to be implemented (when the algorithm is ready) */
/*

pub fn add_order(
    inp_auth_id: String,
    inp_exch_id: i32,
    inp_mkt_id: i32,
    order_type_id: i32,
    price_type_id: i32,
    inp_limit_price: i32,
    inp_order_quantity: f32) -> () {

    //Requesting
    let request_body: &str = r#"{
            "auth_id": inp_auth_id,
            "exch_id": inp_exch_id,
            "mkt_id": inp_mkt_id,
            "order_type_id": order_type_id,
            "price_type_id": price_type_id,
            "limit_price": inp_limit_price,
            "order_quantity": inp_order_quantity
        }"#;
    let uri: hyper::Uri = "https://api.coinigy.com/api/v1//addOrder".parse().unwrap();

    //Watch out!!! This will fill a real order!
    return;

    let value: Value = create_post(request_body, uri)?;
    let data = match value.get("data") {
        Some(val) => Ok(val),
        None => Err(format_err!("data was not found within the struct!"))
    }?;
    let resp: types::ResponseBalance = serde_json::from_value(data.clone())?;
    let x: Vec<types::SingleBalance> = types::convert(resp.0);

    println!("{:?}", x);

    Ok(x)
}
*/