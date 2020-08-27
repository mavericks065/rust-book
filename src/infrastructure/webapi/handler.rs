
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};
use tokio::macros::support::Future;
use diesel::PgConnection;
use crate::infrastructure::webapi::dtos::{NewCompany, CompanyDto};

pub fn health_handler() -> String {
    let result =  "health check: all good";
    result.to_string()
}

pub fn create_new_company(new_company: NewCompany, connection: PgConnection) -> CompanyDto {
    CompanyDto{
        name : String::from(""),
        description : String::from(" "),
        full_address: String::from(""),
        abn : String::from("")
    }
}
