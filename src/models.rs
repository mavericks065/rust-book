#[derive(Queryable)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub address: bool,
    pub postCode: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
}
