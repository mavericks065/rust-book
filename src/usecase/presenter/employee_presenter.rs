use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeResponse {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}
