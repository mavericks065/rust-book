use crate::domain::company::port::ports::NewCompany;
use crate::infrastructure::webapi::dtos::CompanyResponse;

use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::PgConnection;
use tokio::macros::support::Future;
use warp::{http, reject, reply::json, Rejection, Reply, http::StatusCode};
use warp::reply::Response;

pub async fn health_handler(_: Pool<ConnectionManager<PgConnection>>) -> Result<impl Reply, Rejection> {
    let result = "health check: all good";
    Ok(warp::reply::with_status(result, http::StatusCode::OK))
}

pub async fn create_new_company(
    new_company: NewCompany,
    connection: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Rejection> {
    let fake_result = CompanyResponse {
        name: String::from("YO"),
        description: String::from(" "),
        full_address: String::from(""),
        abn: String::from(""),
    };

    Ok(json(&fake_result))
}
