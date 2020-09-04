extern crate rust_followup;

extern crate diesel;

use self::rust_followup::infrastructure::webapi::handler;
use rust_followup::infrastructure::webapi::dtos::NewCompany;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::convert::Infallible;

use warp::{self, http, reject, Filter, Rejection, http::Response};

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_pg_connection(pg_pool: PgPool) -> PgPooledConnection {
    pg_pool.get().expect("Cannot get connection")
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn with_db(db_pool: PgPool) -> impl Filter<Extract=(PgPool, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[tokio::main]
async fn main() {
    let pool = establish_connection();
    let companies_route = warp::path("companies")
        .and(warp::post()
            .and(warp::body::json())
            .and(with_db(pool.clone()))
            .and_then(handler::create_new_company)
        );

    let status_route = warp::path("status")
        .and(warp::get()
            .and(with_db(pool.clone()))
            .and_then(handler::health_handler)
        );

    let hello_route = warp::path("hello").map(|| "Hello, World!");

    let routes = status_route.or(hello_route).or(companies_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

