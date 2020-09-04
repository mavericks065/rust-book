pub struct Company {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: Option<String>,
    pub ceo_id: Option<i32>,
    pub created_at: NaiveDateTime,
}
