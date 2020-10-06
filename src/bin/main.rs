extern crate diesel;
extern crate rust_followup;

use rust_followup::infrastructure::web_api::web_server;

fn main() {
    web_server::start_server()
}
