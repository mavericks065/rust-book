#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use warp::{self, http, Filter, Rejection, reject};

mod handler;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:mysecretpassword@localhost:5432/postgres";

    PgConnection::establish(&database_url)
        .unwrap_or_else(
            |err| panic!("Error connecting to {}, {}", database_url, err)
        )
}

#[tokio::main]
async fn main() {
    let status_route = warp::path("status")
        .and(
            warp::get().map(|| {
                establish_connection();
                let response = "health check: all good";
                warp::reply::json(&response)
            })
        );

    let hello_route = warp::path("hello")
        .map(|| "Hello, World!");
    let routes = status_route.or(hello_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
