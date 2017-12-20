use diesel::prelude::*;
//use diesel::sqlite::SqliteConnection;
use database::types;

use failure::Error;
use std::{thread, time};
use serde_json;
use serde_json::Value;
use hyper_tls::HttpsConnector;

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

//use diesel;
use diesel::associations::HasTable;

use std::time::{SystemTime, UNIX_EPOCH};

use database::db;


const RATE_LIMIT_CENTISECONDS: u64 = 10;
// https://graphs.coinmarketcap.com/currencies/bitcoin/1367131641000/1367218041000/
const BTC_BASE: (&str, i64, i64) = ("bitcoin", 1367131641000, 1367218041000 - 1367131641000);
// https://graphs.coinmarketcap.com/currencies/ethereum/1438958970000/1439500431288/
const ETH_BASE: (&str, i64, i64) = ("ethereum", 1438958970000, 1439500431288 - 1438958970000);
// https://graphs.coinmarketcap.com/currencies/litecoin/1367174842000/1367261242000/
const LTC_BASE: (&str, i64, i64) = ("litecoin", 1367174842000, 1367261242000 - 1367174842000);


pub fn parse_values(value: Value) -> Result<(Vec<types::IntRecord>, Vec<types::FloatRecord>,
                                             Vec<types::FloatRecord>, Vec<types::FloatRecord>), Error> {
    let raw_market_cap = match value.get("market_cap_by_available_supply") {
        Some(val) => Ok(val),
        None => Err(format_err!("market_cap_by_available_supply was not found within the struct!"))
    }?;
    let market_cap: Vec<types::IntRecord> = serde_json::from_value(raw_market_cap.clone())?;

    let raw_price_btc = match value.get("price_btc") {
        Some(val) => Ok(val),
        None => Err(format_err!("price_btc was not found within the struct!"))
    }?;
    let price_btc: Vec<types::FloatRecord> = serde_json::from_value(raw_price_btc.clone())?;

    let raw_price_usd = match value.get("price_usd") {
        Some(val) => Ok(val),
        None => Err(format_err!("price_usd was not found within the struct!"))
    }?;
    let price_usd: Vec<types::FloatRecord> = serde_json::from_value(raw_price_usd.clone())?;

    let raw_vol_usd = match value.get("volume_usd") {
        Some(val) => Ok(val),
        None => Err(format_err!("volume_usd was not found within the struct!"))
    }?;
    let vol_usd: Vec<types::FloatRecord> = serde_json::from_value(raw_vol_usd.clone())?;


    Ok((market_cap, price_btc, price_usd, vol_usd))
}

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


// TODO: do this up until now
// TODO: if stuck n times, skip n offsets
// TODO: come back to skipped offsets (see difference between two pairwise dates)

//TODO: put this into an util file
pub fn str_to_currency_selection(currencies: String) -> types::CurrencySelectionTuple{
    /** The input should be comma-separated values of the names of coinmarketcap **/
    let mut currencies: Vec<&str> = currencies.split(", ").collect();
    println!("The currency string {:?}", currencies);

    // We want to make damn sure there's not error, so we panic if the pair is not existent
    let mut out: types::CurrencySelectionTuple = (false, false, false);

    // Test if this actually does re-assignment
    for currency in currencies {
        match currency {
            "bitcoin" => {
                out.0 = true
            },
            "ethereum" => {
                out.1 = true;
            },
            "litecoin" => {
                out.2 = true;
            },
            _ => {
                panic!("No valid currency pair is given. Because this relies on string \
                comparison, we panicked!");
            }
        };
    }

    return out;
}

pub fn get_website_data() {
    //Skipping functionality
    let mut skip_counter = 0;

    let currencies = vec![BTC_BASE, ETH_BASE, LTC_BASE];

    let conn = match db::establish_connection() {
        Ok(x) => x,
        Err(err) => panic!("Error establishing database connection! {:?}", err)
    };

    for currency in &currencies {
        let mut recent_time: i64;
        let table_name = currency.0;
        let index: i64 = currency.1;
        let offset = currency.2;

        println!("{:?}", currency);

        let selection_tuple: types::CurrencySelectionTuple = str_to_currency_selection(currency.0
            .to_owned());

        loop {
            recent_time = match db::get_most_recent_entry(&conn, selection_tuple) {
                Ok(x) => (x as i64) * 1000,
                Err(err) => panic!("Something went wrong while getting the most recent entry \
                time: {:?}", err)
            };

            //If all data is collected, skip to next coin
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            if recent_time + offset >= (since_the_epoch.as_secs() as i64) * 1000 {
                break;
            }

            //Start from scratch if needed
            if index > recent_time {
                recent_time = index;
            }

            let res: serde_json::Value = match send_request(table_name, recent_time + skip_counter * offset, offset) {
                Ok(x) => x,
                Err(err) => {
                    println!("Something went wrong when sending the request!: {:?}", err);
                    warn!("Something went wrong when sending the request!: {:?}", err);
                    continue;
                }
            };

            let ins = match parse_values(res) {
                Ok(x) => x,
                Err(err) => {
                    println!("Something wen wrong parsing the value! {:?}", err);
                    warn!("Something wen wrong parsing the value! {:?}", err);
                    continue;
                }
            };

            println!("Skip counter: {:?}", skip_counter);

            let updated_vals = match db::insert_into_db(&conn, ins, currency.0) {
                Ok(x) => x,
                Err(err) => {
                    //
                    println!("Something went wrong inserting the request to the database! {:?}", err);
                    warn!("Something went wrong inserting the request to the database! {:?}", err);
                    skip_counter += 1;
                    continue;
                }
            };

            skip_counter = 0;
        }
    }

    println!("Oleee, done for today!");
}

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

//TODO: implement method to repair corrupt database (download missing entries)