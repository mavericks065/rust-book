
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};
use tokio::macros::support::Future;

pub fn health_handler() -> String {
    let result =  "health check: all good";
    result.to_string()
}
