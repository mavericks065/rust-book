use crate::infrastructure::webapi::dtos::{CompanyDto, NewCompany};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use tokio::macros::support::Future;
use warp::{http::StatusCode, reject, reply::json, Rejection, Reply};

pub fn health_handler() -> String {
    let result = "health check: all good";
    result.to_string()
}

pub fn create_new_company(
    new_company: NewCompany,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> CompanyDto {
    CompanyDto {
        name: String::from(""),
        description: String::from(" "),
        full_address: String::from(""),
        abn: String::from(""),
    }
}
