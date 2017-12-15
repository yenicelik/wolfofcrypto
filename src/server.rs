use std::{thread, time};

use coinigy_live as coinlive;

pub fn serve() {

    let sleep_interval = time::Duration::from_millis(1 * 1000); // Seconds

    // Retrieve all needed information, including auth_id
    let auth_id = match coinlive::get_auth_id() {
        Ok(x) => (&*(&x[0]).auth_id).to_owned(),
        Err(err) => panic!("Already failed while getting id! {:?}", err)
    };

    println!("Using auth_id : {}", auth_id);

    // Ping and download data every few minutes
    loop {
        println!("Ping...");
        thread::sleep(sleep_interval);

    }

}
