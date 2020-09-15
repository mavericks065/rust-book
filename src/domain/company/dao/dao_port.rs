use crate::domain::company::model::company::Company;
use std::error::Error;

pub trait CompanyDao {
    fn create_company(&self, company: Company) -> Result<Company, Box<dyn Error>>;
}

