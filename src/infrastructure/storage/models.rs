use diesel::sql_types::Date;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
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
    pub cea_id: i32
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
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
    pub salary: i16
}
