
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};
use tokio::macros::support::Future;
use diesel::PgConnection;
use crate::infrastructure::webapi::dtos::{NewCompany, CompanyDto};
use std::ptr::null;

pub fn health_handler() -> String {
    let result =  "health check: all good";
    result.to_string()
}

pub fn create_new_company(new_company: NewCompany, connection: PgConnection) -> CompanyDto {

}
