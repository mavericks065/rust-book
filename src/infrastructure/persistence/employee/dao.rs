
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::usecase::dao::employee_dao_port::EmployeeDaoTrait;
use crate::domain::employee::employee::Employee;
use std::error::Error;

pub struct DieselEmployeeDao {
    connection: PooledConnection<ConnectionManager<PgConnection>>
}

impl DieselEmployeeDao {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> DieselEmployeeDao {
        DieselEmployeeDao { connection: connection }
    }
}
impl EmployeeDaoTrait for DieselEmployeeDao {
    fn create_employee(&self, employee: Employee) -> Result<Employee, Box<dyn Error>> {
        unimplemented!()
    }
}
