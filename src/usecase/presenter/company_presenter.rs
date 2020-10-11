#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompanyResponse {
    pub name: String,
    pub description: String,
    pub full_address: String,
    pub abn: Option<String>,
}
