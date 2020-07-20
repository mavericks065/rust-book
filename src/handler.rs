
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};
use tokio::macros::support::Future;

use diesel::pg::PgConnection;

pub fn health_handler() -> String {
    let result =  "health check: all good";
    result.to_string()
}
