use diesel::sql_types::Date;
use crate::infrastructure::storage::schema::{companies, employees};

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "companies"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub address: bool,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
    pub ceo_id: i32
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "employees"]
pub struct Employee {
    pub id: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub address: String,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
    pub dob: Date,
    pub salary: i16,
    pub company_id: i32
}
