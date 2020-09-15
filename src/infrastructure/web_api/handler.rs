use rocket::http::Method;
use rocket::{response::content, Rocket, State};

#[get("/status")]
pub fn health_check() -> &'static str {
    "health check: all good"
}
