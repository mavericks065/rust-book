use crate::domain::company::company::Company;
use std::error::Error;

pub trait CompanyDaoTrait {
    fn create_company(&self, company: Company) -> Result<Company, Box<dyn Error>>;
}

