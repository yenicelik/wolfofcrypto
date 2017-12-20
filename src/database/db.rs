use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use database::types;

use failure::Error;
use serde_json::Value;
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use diesel;
use std::i32;
use diesel::associations::HasTable;
use std::time::{SystemTime, UNIX_EPOCH};
use database;

//Define the following struct to cover all options
pub fn establish_connection() -> Result<SqliteConnection, Error> {
    match SqliteConnection::establish("/Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db") {
        Ok(x) => Ok(x),
        Err(err) => Err(format_err!("data was not found within the struct! {:?}", err))
    }
}

pub fn get_entries_between(conn: &SqliteConnection, currencies: types::CurrencySelectionTuple,
                           start_time: i32, end_time: i32) -> Result<Vec<types::Record>, Error> {
    //TODO: iterate over all booleans in currencies and create appropriate filter clauses
    let res = types::currency_table::table
        .filter(types::currency_table::time.ge(start_time))
        .filter(types::currency_table::time.lt(end_time))
        .order(types::currency_table::time.desc())
        .load::<types::Record>(&*conn);

    match res {
        Ok(x) => {
            Ok(x)
        },
        Err(err) => Err(format_err!("Something went wrong retrieving the most recently inserted \
        coin record! {:?}", err))
    }
}

pub fn get_most_recent_entry(conn: &SqliteConnection, currencies: types::CurrencySelectionTuple) ->
                                                                                           Result<i32, Error> {

    //TODO: iterate over all booleans in currencies and create appropriate filter clauses
    let res = types::currency_table::table
        .order(types::currency_table::time.desc())
        .limit(1)
        .load::<types::Record>(&*conn);

    match res {
        Ok(x) => {
            if x.len() > 0 {
                Ok(x.get(0).unwrap().time)
            } else {
                Ok(0)
            }
        },
        Err(err) => Err(format_err!("Something went wrong retrieving the most recently inserted \
        coin record! {:?}", err))
    }
}

pub fn insert_into_db(conn: &SqliteConnection, ins: types::InsertionType, currency: &str) ->
                      Result<usize, Error> {
    let mut out: usize = 0;

    for i in 0..(ins.1.len()) {
        // Check if there's anything funky with the vector and tuples
        if ins.0.get(i).unwrap().unixtime != ins.1.get(i).unwrap().unixtime ||
            ins.1.get(i).unwrap().unixtime != ins.2.get(i).unwrap().unixtime ||
            ins.2.get(i).unwrap().unixtime != ins.3.get(i).unwrap().unixtime {
            return Err(format_err!("Something is funky with the structs and arrays: {:?}", ins));
        }

        let to_be_inserted: types::Record = types::Record {
            currency: currency.to_owned(),
            time: (ins.0.get(i).unwrap().unixtime / 1000) as i32,
            market_cap: ins.0.get(i).unwrap().intfield as f32,
            price_btc: ins.1.get(i).unwrap().floatfield,
            price_usd: ins.2.get(i).unwrap().floatfield,
            vol_usd: ins.3.get(i).unwrap().floatfield
        };

        out = match diesel::insert_into(types::currency_table::table)
            .values(&to_be_inserted)
            .execute(&*conn) {
                Ok(x) => x,
                Err(err) => {
                    //TODO: log the currency pair here
                    warn!("Something went wrong inserting the currency pair! {:?}", err);
                    continue;
                }
            }
    }

    Ok(out)
}


#[cfg(tests)]
mod db_tests {
    //Building blocks
    #[test]
    fn test_create_request() {
        use super::establish_connection;
        match establish_connection() {
            Ok(_) => {}
            Err(err) => panic!(err)
        };
    }
}

