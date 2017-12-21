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

const RATE_LIMIT_CENTISECONDS: u64 = 10;
// https://graphs.coinmarketcap.com/currencies/bitcoin/1367131641000/1367218041000/
const BTC_BASE: (&str, i64, i64) = ("bitcoin", 1367131641000, 1367218041000 - 1367131641000);
// https://graphs.coinmarketcap.com/currencies/ethereum/1438958970000/1439500431288/
const ETH_BASE: (&str, i64, i64) = ("ethereum", 1438958970000, 1439500431288 - 1438958970000);
// https://graphs.coinmarketcap.com/currencies/litecoin/1367174842000/1367261242000/
const LTC_BASE: (&str, i64, i64) = ("litecoin", 1367174842000, 1367261242000 - 1367174842000);

//TODO: implement method to repair corrupt database (download missing entries)

// TODO: do this up until now
// TODO: if stuck n times, skip n offsets
// TODO: come back to skipped offsets (see difference between two pairwise dates)
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