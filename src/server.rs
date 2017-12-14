use std::{thread, time};
use coinigy_live;


pub fn serve() {

    let sleep_interval = time::Duration::from_millis(1 * 1000); // Seconds

    // Ping and download data every few minutes
    while (true) {
        println!("Ping...");
        thread::sleep(sleep_interval);



    }

}

/*
pub fn retrieve_auth_id() {

}
*/