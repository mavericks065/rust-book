use crate::domain::company::model::company::Company;
use std::error::Error;

pub trait CompanyInteractorTrait {
    fn create_company(new_company: NewCompany) -> Result<Company, Box<dyn Error>>;
}

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
