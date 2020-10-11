use std::error::Error;

use chrono::Utc;
use serde::Deserialize;

use crate::domain::company::company::Company;
use crate::usecase::dao::company_dao_port::CompanyDaoTrait;

pub struct CompanyInteractor {
    pub dao: Box<dyn CompanyDaoTrait>
}

impl CompanyInteractor {
    pub fn create_company(&self, new_company: NewCompanyRequest) -> Result<Company, Box<dyn Error>> {
        println!("COMPANYINTERACTOR new company {}", new_company.name);

        let some_company = self.dao
            .create_company(NewCompanyRequest::to_company(new_company));
        some_company
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct NewCompanyRequest {
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: u64,
    pub city: String,
    pub country: String,
    pub abn: String,
}

impl NewCompanyRequest {
    fn to_company(new_company: NewCompanyRequest) -> Company {
        Company {
            id: Option::None,
            name: new_company.name,
            description: new_company.description,
            address: new_company.address,
            post_code: new_company.post_code,
            city: new_company.city,
            country: new_company.country,
            abn: Option::Some(new_company.abn),
            ceo_id: Option::None,
            created_at: Utc::now().naive_utc(),
        }
    }
}
