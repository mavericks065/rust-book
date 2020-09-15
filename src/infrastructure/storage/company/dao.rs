use crate::domain::company::dao::dao_port::CompanyDao;
use crate::domain::company::model::company::Company;
use std::error::Error;
use chrono::Utc;

impl CompanyDao {
    pub fn create_company(&self, company: Company) -> Result<Company, Box<dyn Error>> {
        Ok(Company {
            id: Option::Some(1),
            name: String::from("name"),
            description: String::from("description"),
            address: String::from("address"),
            post_code: 2000,
            city: String::from("Sydney"),
            country: String::from("Aus"),
            abn: Option::None,
            ceo_id: Option::None,
            created_at: Utc::now().naive_utc()
        })
    }
}

