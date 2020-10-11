use std::error::Error;
use std::time::SystemTime;

use serde::Deserialize;

use crate::domain::employee::dao::employee_dao_port::EmployeeDaoTrait;
use crate::domain::employee::model::employee::Employee;

pub struct EmployeeInteractor {
    pub employee_dao: Box<dyn EmployeeDaoTrait>
}

impl EmployeeInteractor {
    pub fn create_employee(&self, new_employee: NewEmployeeRequest) -> Result<Employee, Box<dyn Error>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewEmployeeRequest {
    pub email: String,
    pub firsname: String,
    pub lastname: String,
    pub address: Option<String>,
    pub post_code: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub dob: Option<SystemTime>,
    pub salary: Option<i32>,
    pub company_id: Option<i32>,
}
