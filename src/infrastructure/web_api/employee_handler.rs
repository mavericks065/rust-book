use rocket_contrib::json::Json;

use crate::infrastructure::web_api::context::Context;
use crate::infrastructure::web_api::dtos::EmployeeResponse;
use crate::domain::employee::interactor::employee_interactor::NewEmployeeRequest;


#[post("/employees", format = "json", data = "<employee>")]
pub fn create_employee(employee: Json<NewEmployeeRequest>, context: Context) -> Json<EmployeeResponse> {
    println!("HANDLER new employee {}", employee.email);
    unimplemented!()
}
