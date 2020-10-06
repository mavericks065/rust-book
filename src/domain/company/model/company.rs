use chrono::NaiveDateTime;

pub struct Company {
    pub id: Option<u64>,
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: u64,
    pub city: String,
    pub country: String,
    pub abn: Option<String>,
    pub ceo_id: Option<u64>,
    pub created_at: NaiveDateTime,
}
