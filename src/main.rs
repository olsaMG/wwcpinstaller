mod models;

use dotenv::dotenv;

extern crate dotenv;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Hello");
}
