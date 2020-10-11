use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::request::{FromRequest, Outcome, Request};

use crate::infrastructure::persistence::company::dao::DieselCompanyDao;
use crate::infrastructure::persistence::employee::dao::DieselEmployeeDao;
use crate::infrastructure::web_api::web_server::DbCon;
use crate::usecase::interactor::company_interactor::CompanyInteractor;
use crate::usecase::interactor::employee_interactor::EmployeeInteractor;

pub struct Context {
    pub company_interactor: CompanyInteractor,
    pub employee_interactor: EmployeeInteractor,
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let db_con: DbCon = request.guard::<DbCon>()?;
        let db_con2: DbCon = request.guard::<DbCon>()?;
        Outcome::Success(Context::new(db_con.0, db_con2.0))
    }
}

impl Context {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
        connection2: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Context {
        let diesel_company_dao = DieselCompanyDao::new(connection);
        let company_interactor= CompanyInteractor {
            dao: Box::new(diesel_company_dao)
        };
        let diesel_employee_dao = DieselEmployeeDao::new(connection2);
        let employee_interactor= EmployeeInteractor {
            employee_dao: Box::new(diesel_employee_dao)
        };
        Context {
            company_interactor: company_interactor,
            employee_interactor: employee_interactor,
        }
    }
}
