use diesel;
use diesel::prelude::*;
use diesel::sqlite;
use diesel::sqlite::SqliteConnection;

fn main() {

    let connection = SqliteConnection::establish("sqlite.db").unwrap();
    let qr = connection.execute("SELECT name, age FROM people;").unwrap();
}