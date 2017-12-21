use database::types;

use failure::Error;
use std::{thread, time};
use serde_json;
use serde_json::Value;
use hyper_tls::HttpsConnector;

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use std::time::{SystemTime, UNIX_EPOCH};

use database::db;



#[cfg(test)]
mod tests_populate_db {
    /** Does not require a request_body **/


    #[test]
    fn test_send_request() {
        use super::send_request;
        use super::BTC_BASE;
        match send_request(BTC_BASE.0, BTC_BASE.1, BTC_BASE.2) {
            Ok(_) => {}
            Err(err) => panic!(err)
        }
    }

    #[test]
    fn test_parse_values() {
        use super::parse_values;
        use super::send_request;
        use super::BTC_BASE;
        use serde_json::Value;

        let val: Value = match send_request(BTC_BASE.0, BTC_BASE.1, BTC_BASE.2) {
            Ok(x) => x,
            Err(err) => panic!(err)
        };

        match parse_values(val) {
            Ok(x) => x,
            Err(err) => panic!(err)
        };
    }

    #[test]
    fn get_web_data() {
        use super::get_website_data;

        get_website_data();
    }


    /*
    #[test]
    fn test_insert_into_db() {
        use super::parse_values;
        use super::send_request;
        use super::BTC_BASE;
        use super::db::insert_into_db;
        use serde_json::Value;
        use super::establish_connection;

        let val: Value = match send_request(BTC_BASE.0, BTC_BASE.1, BTC_BASE.2) {
            Ok(x) => x,
            Err(err) => panic!(err)
        };

        let tmp = match parse_values(val) {
            Ok(x) => x,
            Err(err) => panic!(err)
        };

        let conn = match establish_connection() {
            Ok(x) => x,
            Err(err) => panic!(err)
        };

        insert_into_db(conn, tmp, "bitcoin");
    }
    */

}