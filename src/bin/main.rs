extern crate rust_followup;

extern crate diesel;

use self::rust_followup::infrastructure::webapi::handler;
use rust_followup::infrastructure::webapi::dtos::NewCompany;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

use warp::{self, http, reject, Filter, Rejection};

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    let pool = establish_connection();
    let companies_route = warp::path("companies").and(warp::post().and(warp::body::json()).map(
        |new_company: NewCompany| {
            let connection: &PgPooledConnection = &pool.get().expect("Cannot get connection");
            handler::create_new_company(new_company, connection)
        },
    ));
    let status_route = warp::path("status").and(warp::get().map(move || {
        let _connection: &PgPooledConnection = &pool.get().expect("Cannot get connection");
        handler::health_handler()
    }));

    let hello_route = warp::path("hello").map(|| "Hello, World!");

    let routes = status_route.or(hello_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
