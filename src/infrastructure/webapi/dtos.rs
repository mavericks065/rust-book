
#[derive(Debug, Clone, Deserialize)]
pub struct NewCompany {
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyDto {
    pub name: String,
    pub description: String,
    pub full_address: String,
    pub abn: String,
}
