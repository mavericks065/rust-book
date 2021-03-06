use crate::domain::employee::employee::Employee;
use std::error::Error;

pub trait EmployeeDaoTrait {
    fn create_employee(&self, employee: Employee) -> Result<Employee, Box<dyn Error>>;
}
