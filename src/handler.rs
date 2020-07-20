
// use crate::{Result};
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}
