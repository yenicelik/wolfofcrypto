use failure::Error;

use super::super::database::types;
use super::super::database::db;

pub fn get_all_entries_between(
    start_unixtime: i32,
    end_unixtime: i32) -> Result<Vec<types::Record>, Error> {
    //Move this one out of the function!
    let conn = match db::establish_connection() {
        Ok(x) => x,
        Err(err) => return Err(format_err!("Something went wrong establishing a connection with \
        the DB! {:?}", err))
    };

    let selection_tuple = (true, true, true);

    let results_btw = match db::get_entries_between(
        &conn,
        selection_tuple,
        start_unixtime,
        end_unixtime) {
            Ok(x) => x,
            Err(err) => return Err(format_err!("Something went wrong btc_entries_between {:?}", err))
    };

    Ok(results_btw)
}

#[cfg(test)]
mod test_historical_simulation {
    #[test]
    fn test_mod() {
        println!("Testing works");
    }
}