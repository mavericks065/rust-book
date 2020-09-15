use rocket::http::Method;
use rocket::{response::content, Rocket, State};

use crate::domain::company::interactor::company_interactor::NewCompanyRequest;
use crate::infrastructure::web_api::dtos::CompanyResponse;

use rocket_contrib::json::Json;
use rocket::handler::Cloneable;
use crate::infrastructure::web_api::web_server::DbCon;

#[post("/companies", format = "json", data = "<company>")]
pub fn create_company(company: Json<NewCompanyRequest>, conn: DbCon) -> Json<CompanyResponse> {
    println!("new company {}", company.name);
    let fake_result = CompanyResponse {
        name: company.name.clone(),
        description: company.description.clone(),
        full_address: company.address.clone(),
        abn: company.abn.clone(),
    };

    Json(fake_result)
}
