use diesel::prelude::*;
use database::types;

use failure::Error;
use futures::{Stream};
use diesel;
use std::i32;

use std::ops::Deref;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

use r2d2;

use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::Status;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
static DATABASE_URL: &'static str = env!("DATABASE_URL");
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("db pool")
}

/** Start of actual db actions **/
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

