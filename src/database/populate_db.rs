use diesel::prelude::*;
use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
use failure::Error;
use hyper;
use std::{thread, time};
use serde_json;
use serde_json::{Value};
use hyper_tls::HttpsConnector;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use database::types;

const rate_limit_centiseconds: u64 = 10;
// https://graphs.coinmarketcap.com/currencies/bitcoin/1367131641000/1367218041000/
const BTC_BASE: (&str, i64, i64) = ("bitcoin", 1367131641000, 1367218041000-1367131641000);
// https://graphs.coinmarketcap.com/currencies/ethereum/1438958970000/1439500431288/
const ETH_BASE: (&str, i64, i64) = ("ethereum", 1438958970000, 1439500431288-1438958970000);
// https://graphs.coinmarketcap.com/currencies/litecoin/1367174842000/1367261242000/
const LTC_BASE: (&str, i64, i64) = ("litecoin", 1367174842000, 1367261242000-1367174842000);


pub fn establish_connection() -> Result<SqliteConnection, Error> {

    match SqliteConnection::establish("./sqlite_database.db") {
        Ok(x) => Ok(x),
        Err(err) => Err(format_err!("data was not found within the struct!"))
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

pub fn str2table(name: &str) {
    if name == "bitcoin" {
        return types::bitcoin::table;
    } else if name == "ethereum" {
        return types::etheruem::table;
    } else if name == "litecoin" {
        return types::litecoin::table;
    } else {
        panic!("Something went wrong! Name is not a table! {}", name);
    }
}

pub fn insert_into_db(conn: SqliteConnection, ins: (Vec<types::IntRecord>,
                                                           Vec<types::FloatRecord>,
                                                           Vec<types::FloatRecord>,
                                                    Vec<types::FloatRecord>)) -> Result<(), Error>{
    for i in 0..(ins.1.len()) {
        // Check if there's anything funky with the vector and tuples
        if (ins.0.get(i).unwrap().unixtime != ins.1.get(i).unwrap().unixtime ||
            ins.1.get(i).unwrap().unixtime != ins.2.get(i).unwrap().unixtime ||
            ins.2.get(i).unwrap().unixtime != ins.3.get(i).unwrap().unixtime) {
                return Err(format_err!("Something is funky with the structs and arrays: {:?}", ins));
        }

        println!("{:?}", (
            (ins.0.get(i).unwrap().unixtime/1000),
            ins.0.get(i).unwrap().intfield,
            ins.1.get(i).unwrap().floatfield,
            ins.2.get(i).unwrap().floatfield,
            ins.3.get(i).unwrap().floatfield));

        let response = diesel::insert(&skill_vec)
            .into(types::table_preference::table)
            .execute(&**conn)?;



    }
  /*  for i in range(len(market_cap)):
        assert(market_cap[i][0] == price_usd[i][0] and vol_usd[i][0] == price_btc[i][0] and price_usd[i][0] == price_btc[i][0])
    print(int(market_cap[i][0]/1000), market_cap[i][1], price_btc[i][1], price_usd[i][1], vol_usd[i][1])
    */
    Ok(())
}

pub fn send_request(currency: &str, start: i64, offset: i64) -> Result<Value, Error>{
    let wait_time = time::Duration::from_millis(rate_limit_centiseconds * 100); //1sec
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

    for currency in &currencies {
        let table_name = currency.0;
        let mut index: i64 = currency.1;
        let offset = currency.2;

        println!("{:?}", currency);

        loop {
            let res = send_request(table_name, index, offset);
            index += offset;
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
            Ok(x) => {},
            Err(err) => panic!(err)
        };
    }

    #[test]
    fn test_send_request() {
        use super::send_request;
        use super::BTC_BASE;
        match send_request(BTC_BASE.0, BTC_BASE.1, BTC_BASE.2) {
            Ok(x) => {},
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

        insert_into_db(conn, tmp);
    }
    */

    #[test]
    fn save_to_db() {

    }

    /*
    #[test]
    fn test_get_website_data() {
        use super::get_website_data;
        get_website_data();
    }
    */

}


/*

c.execute("INSERT INTO ethereum VALUES (?, ?, ?, ?, ?);", (unixtime, val1, val2, val3, val4))

print("Last value: ", market_cap[i][0])
conn.commit()

base += off


conn.commit()
conn.close()

*/

//TODO: implement method to repair corrupt database (download missing entries)