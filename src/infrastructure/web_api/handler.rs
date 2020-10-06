#[get("/status")]
pub fn health_check() -> &'static str {
    "health check: all good"
}
