use rocket::http::Method;
use rocket::{response::content, Rocket, State};

use crate::infrastructure::web_api::handler::static_rocket_route_info_for_health_check;
use crate::infrastructure::web_api::company_handler::static_rocket_route_info_for_create_company;

#[database("master")]
pub struct DbCon(diesel::PgConnection);

pub fn get_server() -> Rocket {
    rocket::ignite()
        .mount(
            "/",
            rocket::routes![health_check, create_company],
        )
        .attach(DbCon::fairing())
}

pub fn start_server() {
    get_server().launch();
}
