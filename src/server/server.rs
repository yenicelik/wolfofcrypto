use rocket;

use historical::historical_simulation;

use coinigy_live as coinlive;

use server::types;
use database;

use rocket_contrib::Json;

use database::db;

static ERR_STATUS: i32 = 111;
static OK_STATUS: i32 = 999;

#[get("/")]
fn test_landing_page() -> &'static str {
    "The server connection is working..."
}

//TODO: if vectors are different size, auto-append respectively by the 0-tuples
//Endpoint to register a vote
#[post("/get-historical-data-from-to", format = "application/json", data = "<post_historical_json>")]
fn endpoint_get_historical_data(
    post_historical_json: Json<types::PostHistoricalData>,
    conn: &db::DbConn) -> Json<types::Response<Vec<database::types::Record>>> {
    trace!("Accessing get-historical-data-from-to");
    debug!("{:?}", post_historical_json);

    // Parse query json
    let post_historical: types::PostHistoricalData = post_historical_json.into_inner();

    // Prepare output format
    let mut out: types::Response<Vec<database::types::Record>>;

    //TODO: how to deal with connection objects? spawn a new one? singleton somehow?
    let response = historical_simulation::get_all_entries_between(
            post_historical.start_unixtime,
            post_historical.end_unixtime
    );

    match response {
        Ok(x) => {
            out = types::Response {
                status: OK_STATUS,
                result: x,
                error: "".to_owned()
            }
        },
        Err(err) => {
            out = types::Response {
                status: ERR_STATUS,
                result: vec![],
                error: format!("Error: {:?}", err),
            };
        }
    }

    return Json(out);
}

pub fn serve() {

    // Retrieve all needed information, including auth_id
    let auth_id = match coinlive::get_auth_id() {
        Ok(x) => (&*(&x[0]).auth_id).to_owned(),
        Err(err) => panic!("Already failed while getting id! {:?}", err)
    };

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![test_landing_page, endpoint_get_historical_data]).launch();


    println!("Using auth_id : {}", auth_id);

    // Ping and download data every few minutes
    println!("Placeholder for pinging loop");

}
