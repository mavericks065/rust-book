use crate::domain::company::dao::dao_port::CompanyDaoTrait;
use crate::domain::company::model::company::Company;
use std::error::Error;
use chrono::Utc;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::infrastructure::storage::schema::companies::dsl::companies;
use diesel::associations::HasTable;
use crate::infrastructure::storage::entity_models::{CompanyEntity, NewCompanyEntity};
use diesel::RunQueryDsl;
use std::time::SystemTime;

pub struct DieselCompanyDao {
    connection: PooledConnection<ConnectionManager<PgConnection>>
}

impl DieselCompanyDao {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> DieselCompanyDao {
        DieselCompanyDao { connection: connection }
    }
}

impl CompanyDaoTrait for DieselCompanyDao {
    fn create_company(&self, company: Company) -> Result<Company, Box<dyn Error>> {
        // let company_entity = CompanyEntity {
        //     id: Option::None,
        //     name: company.name,
        //     description: company.description,
        //     address: company.address,
        //     post_code: company.post_code,
        //     city: company.city,
        //     country: company.country,
        //     abn: company.abn,
        //     ceo_id: company.ceo_id,
        //     created_at: company.created_at,
        // };
        let company_entity = NewCompanyEntity {
            name: &company.name,
            description: &company.description,
            address: &company.address,
            post_code: &(company.post_code as i32),
            city: &company.city,
            country: &company.country,
        };
        let insert_entity: Result<CompanyEntity, diesel::result::Error> = diesel::insert_into(companies::table())
            .values(company_entity)
            .get_result(&self.connection);
        Ok(Company {
            id: Option::Some(2),
            name: String::from("SUPER COMPANY"),
            description: String::from("description"),
            address: String::from("address"),
            post_code: 2000,
            city: String::from("Sydney"),
            country: String::from("Aus"),
            abn: Option::None,
            ceo_id: Option::None,
            created_at: Utc::now().naive_utc(),
        })
    }
}

