use crate::domain::company::interactor::company_interactor::CompanyInteractor;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use rocket::request::{FromRequest, Request, Outcome};
use crate::infrastructure::storage::company::dao::DieselCompanyDao;
use crate::infrastructure::web_api::web_server::DbCon;

pub struct Context {
    pub company_interactor: CompanyInteractor
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let db_con: DbCon = request.guard::<DbCon>()?;
        Outcome::Success(Context::new(db_con.0))
    }
}

impl Context {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Context {
        let diesel_company_dao = DieselCompanyDao::new(connection);
        let my_comp_interactor= CompanyInteractor {
            dao: Box::new(diesel_company_dao)
        };
        Context {
            company_interactor: my_comp_interactor
        }
    }
}
