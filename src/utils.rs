extern crate log;
extern crate fern;
extern crate chrono;
use std;

#[allow(unused)]
pub fn configure_logger() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!("{}[{}][{}] {}",
                                    chrono::Local::now()
                                        .format("[%Y-%m-%d][%H:%M:%S]"),
                                    record.target(),
                                    record.level(),
                                    message))
        })
        .level(log::LogLevelFilter::Debug)
        // Output to stdout, files, and other Dispatch configs
        .chain(std::io::stdout())
        //       .chain(fern::log_file("output.log"))
        .apply();
}