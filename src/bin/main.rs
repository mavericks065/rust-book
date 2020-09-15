extern crate rust_followup;

extern crate diesel;

use rust_followup::domain::company::dao::dao_port::CompanyDao;
use rust_followup::domain::company::interactor::company_interactor::CompanyInteractor;
use rust_followup::infrastructure::web_api::web_server;
use rust_followup::infrastructure::web_api::dtos::CompanyResponse;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::{dotenv};
use std::env;
use std::result::Result;
use std::convert::Infallible;

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> std::result::Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_pg_connection(pg_pool: PgPool) -> PgPooledConnection {
    pg_pool.get().expect("Cannot get connection")
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn main() {
    web_server::start_server()
}
