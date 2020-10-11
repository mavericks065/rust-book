use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompanyResponse {
    pub name: String,
    pub description: String,
    pub full_address: String,
    pub abn: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeResponse {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}
