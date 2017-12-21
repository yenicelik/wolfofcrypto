use rocket;

use historical::historical_simulation;

use database;

use rocket_contrib::Json;

use types;

use server;

static ERR_STATUS: i32 = 111;
static OK_STATUS: i32 = 999;

#[get("/")]
fn test_landing_page() -> &'static str {
    "The server connection is working..."
}

//TODO: if vectors are different size, auto-append respectively by the 0-tuples
//Endpoint to register a vote
#[post("/historical/get-data-from-to", format = "application/json", data =
"<post_historical_json>")]
fn endpoint_get_historical_data(
    post_historical_json: Json<server::types::PostHistoricalData>,
    conn: database::db::DbConn) -> Json<server::types::Response<Vec<types::Record>>> {
    trace!("Accessing get-historical-data-from-to");
    debug!("{:?}", post_historical_json);

    // Parse query json
    let post_historical: server::types::PostHistoricalData = post_historical_json.into_inner();

    // Prepare output format
    let out: server::types::Response<Vec<types::Record>>;

    //TODO: how to deal with connection objects? spawn a new one? singleton somehow?
    let response = historical_simulation::get_all_entries_between(
            post_historical.start_unixtime,
            post_historical.end_unixtime
    );

    match response {
        Ok(x) => {
            out = server::types::Response {
                status: OK_STATUS,
                result: x,
                error: "".to_owned()
            }
        },
        Err(err) => {
            out = server::types::Response {
                status: ERR_STATUS,
                result: vec![],
                error: format!("Error: {:?}", err),
            };
        }
    }

    return Json(out);
}

pub fn serve() {



    rocket::ignite()
        .manage(database::db::init_pool())
        .mount("/", routes![test_landing_page, endpoint_get_historical_data]).launch();

    // Ping and download data every few minutes
    println!("Placeholder for pinging loop");

}
