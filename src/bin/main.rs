extern crate rust_book;
use self::rust_book::infrastructure::webapi::handler;

extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use warp::{self, http, Filter, Rejection, reject};

fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let status_route = warp::path("status")
        .and(
            warp::get().map(move || {
                establish_connection(&database_url);
                handler::health_handler()
            })
        );

    let hello_route = warp::path("hello")
        .map(|| "Hello, World!");

    let routes = status_route.or(hello_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
