use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use database::types;

use failure::Error;
use std::{thread, time};
use serde_json;
use serde_json::Value;
use hyper_tls::HttpsConnector;

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use diesel;
use std::i32;

const RATE_LIMIT_CENTISECONDS: u64 = 10;
// https://graphs.coinmarketcap.com/currencies/bitcoin/1367131641000/1367218041000/
const BTC_BASE: (&str, i64, i64) = ("bitcoin", 1367131641000, 1367218041000 - 1367131641000);
// https://graphs.coinmarketcap.com/currencies/ethereum/1438958970000/1439500431288/
const ETH_BASE: (&str, i64, i64) = ("ethereum", 1438958970000, 1439500431288 - 1438958970000);
// https://graphs.coinmarketcap.com/currencies/litecoin/1367174842000/1367261242000/
const LTC_BASE: (&str, i64, i64) = ("litecoin", 1367174842000, 1367261242000 - 1367174842000);


pub fn establish_connection() -> Result<SqliteConnection, Error> {
    match SqliteConnection::establish("/Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db") {
        Ok(x) => Ok(x),
        Err(err) => Err(format_err!("data was not found within the struct! {:?}", err))
    }
}

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

pub fn get_most_recent_btc_entry(conn: &SqliteConnection) -> Result<i32, Error> {
    let res = types::bitcoin::table
        .order(types::bitcoin::time.desc())
        .limit(1)
        .load::<types::BTCRecord>(&*conn);
    match res {
        Ok(x) => {
            if x.len() > 0 {
                Ok(x.get(0).unwrap().time)
            } else {
                Ok(0)
            }
        }
        Err(err) => Err(format_err!("Something went wrong retrieving the most recently inserted \
        bitcoin record! {:?}", err))
    }
}

pub fn get_most_recent_eth_entry(conn: &SqliteConnection) -> Result<i32, Error> {
    let res = types::ethereum::table
        .order(types::ethereum::time.desc())
        .limit(1)
        .load::<types::ETHRecord>(&*conn);
    match res {
        Ok(x) => {
            if x.len() > 0 {
                Ok(x.get(0).unwrap().time)
            } else {
                Ok(0)
            }
        }
        Err(err) => Err(format_err!("Something went wrong retrieving the most recently inserted \
        ethereum record! {:?}", err))
    }
}

pub fn get_most_recent_ltc_entry(conn: &SqliteConnection) -> Result<i32, Error> {
    let res = types::litecoin::table
        .order(types::litecoin::time.desc())
        .limit(1)
        .load::<types::LTCRecord>(&*conn);
    match res {
        Ok(x) => {
            if x.len() > 0 {
                Ok(x.get(0).unwrap().time)
            } else {
                Ok(0)
            }
        }
        Err(err) => Err(format_err!("Something went wrong retrieving the most recently inserted \
        litecoin record! {:?}", err))
    }
}


pub fn insert_into_db(conn: &SqliteConnection, ins: (Vec<types::IntRecord>,
                                                     Vec<types::FloatRecord>,
                                                     Vec<types::FloatRecord>,
                                                     Vec<types::FloatRecord>), currency: &str) ->
                      Result<(), Error> {
    for i in 0..(ins.1.len()) {
        // Check if there's anything funky with the vector and tuples
        if ins.0.get(i).unwrap().unixtime != ins.1.get(i).unwrap().unixtime ||
            ins.1.get(i).unwrap().unixtime != ins.2.get(i).unwrap().unixtime ||
            ins.2.get(i).unwrap().unixtime != ins.3.get(i).unwrap().unixtime {
            return Err(format_err!("Something is funky with the structs and arrays: {:?}", ins));
        }

        // TODO: This is really dirty and should probably be changed into an enum
        if currency == "bitcoin" {
            let to_be_inserted = types::BTCRecord {
                time: (ins.0.get(i).unwrap().unixtime / 1000) as i32,
                market_cap: ins.0.get(i).unwrap().intfield as f32,
                price_btc: ins.1.get(i).unwrap().floatfield,
                price_usd: ins.2.get(i).unwrap().floatfield,
                vol_usd: ins.3.get(i).unwrap().floatfield
            };

            let out = match diesel::insert_into(types::bitcoin::table)
                .values(&to_be_inserted)
                .execute(&*conn) {
                Ok(x) => Ok(x),
                Err(err) => Err(format_err!("Something went wrong inserting btc! {:?}", err))
            };

        } else if currency == "ethereum" {
            let to_be_inserted = types::ETHRecord {
                time: (ins.0.get(i).unwrap().unixtime / 1000) as i32,
                market_cap: ins.0.get(i).unwrap().intfield as f32,
                price_btc: ins.1.get(i).unwrap().floatfield,
                price_usd: ins.2.get(i).unwrap().floatfield,
                vol_usd: ins.3.get(i).unwrap().floatfield
            };

            let out = match diesel::insert_into(types::ethereum::table)
                .values(&to_be_inserted)
                .execute(&*conn) {
                Ok(x) => Ok(x),
                Err(err) => Err(format_err!("Something went wrong inserting eth! {:?}", err))
            };

        } else if currency == "litecoin" {
            let to_be_inserted = types::LTCRecord {
                time: (ins.0.get(i).unwrap().unixtime / 1000) as i32,
                market_cap: ins.0.get(i).unwrap().intfield as f32,
                price_btc: ins.1.get(i).unwrap().floatfield,
                price_usd: ins.2.get(i).unwrap().floatfield,
                vol_usd: ins.3.get(i).unwrap().floatfield
            };

            let out = match diesel::insert_into(types::litecoin::table)
                .values(&to_be_inserted)
                .execute(&*conn) {
                Ok(x) => Ok(x),
                Err(err) => Err(format_err!("Something went wrong inserting ltc! {:?}", err))
            };

        } else {
            return Err(format_err!("Something went wrong! Name is not a table! {}", currency));
        }
    }

    Ok(())
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


pub fn get_website_data() {
    let currencies = vec![BTC_BASE, ETH_BASE, LTC_BASE];

    let conn = match establish_connection() {
        Ok(x) => x,
        Err(err) => panic!("Error establishing database connection! {:?}", err)
    };

    for currency in &currencies {
        let mut recent_time: i64;
        let table_name = currency.0;
        let index: i64 = currency.1;
        let offset = currency.2;

        println!("{:?}", currency);

        loop {
            match currency.0 {
                "bitcoin" => {
                    recent_time = match get_most_recent_btc_entry(&conn) {
                        Ok(x) => (x as i64) * 1000,
                        Err(err) => panic!("Something went wrong while getting the most recent \
                        btc entry: {:?}", err)
                    };
                }
                "ethereum" => {
                    recent_time = match get_most_recent_eth_entry(&conn) {
                        Ok(x) => (x as i64) * 1000,
                        Err(err) => panic!("Something went wrong while getting the most recent \
                        eth entry: {:?}", err)
                    };
                }
                "litecoin" => {
                    recent_time = match get_most_recent_ltc_entry(&conn) {
                        Ok(x) => (x as i64) * 1000,
                        Err(err) => panic!("Something went wrong while getting the most recent \
                        ltc entry: {:?}", err)
                    };
                }
                _ => panic!("Wrong currency pair!")
            }

            if index > recent_time {
                recent_time = index;
            }

            // Successful up until now

            let res: serde_json::Value = match send_request(table_name, recent_time, offset) {
                Ok(x) => x,
                Err(err) => {
                    println!("Something went wrong when sending the request!: {:?}", err);
                    warn!("Something went wrong when sending the request!: {:?}", err);
                    continue;
                }
            };

            // Successful up until now

            let ins = match parse_values(res) {
                Ok(x) => x,
                Err(err) => {
                    println!("Something wen wrong parsing the value! {:?}", err);
                    warn!("Something wen wrong parsing the value! {:?}", err);
                    continue;
                }
            };

            // Successful up until now

            match insert_into_db(&conn, ins, currency.0) {
                Ok(_) => {}
                Err(err) => {
                    warn!("Something went wrong inserting the request to the database! {:?}", err);
                    continue;
                }
            };

        }
    }
}

#[cfg(test)]
mod tests_populate_db {
    /** Does not require a request_body **/
    //Building blocks
    #[test]
    fn test_create_request() {
        use super::establish_connection;
        match establish_connection() {
            Ok(_) => {}
            Err(err) => panic!(err)
        };
    }

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
        use super::insert_into_db;
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

    /*
    #[test]
    fn test_get_website_data() {
        use super::get_website_data;
        get_website_data();
    }
    */
}

//TODO: implement method to repair corrupt database (download missing entries)