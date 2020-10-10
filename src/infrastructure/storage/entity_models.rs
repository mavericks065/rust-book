use crate::infrastructure::storage::schema::{check_points, companies, employees, follow_ups};
use diesel::sql_types::Date;
use std::collections::LinkedList;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "companies"]
pub struct CompanyEntity{
    pub id: i32,
    pub name: String,
    pub description:String,
    pub address: String,
    pub post_code: i32,
    pub city: String,
    pub country: String,
    pub abn: Option<String>,
    pub ceo_id: Option<i32>,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "companies"]
pub struct NewCompanyEntity<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub address: &'a str,
    pub post_code: &'a i32,
    pub city: &'a str,
    pub country: &'a str,
}

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "employees"]
pub struct EmployeeEntity {
    pub id: i32,
    pub email: String,
    pub firsname: String,
    pub lastname: String,
    pub address: String,
    pub post_code: i32,
    pub city: String,
    pub country: String,
    pub dob: SystemTime,
    pub salary: i32,
    pub company_id: i32,
    pub follow_up_ids: Vec<i32>,
    pub created_at: SystemTime,
}

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "follow_ups"]
pub struct FollowUpEntity {
    pub id: i32,
    pub managee_id: i32,
    pub check_points_ids: LinkedList<i32>,
}

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "check_points"]
pub struct CheckPointEntity {
    pub id: i32,
    pub comments: String,
    pub manager_actions: String,
    pub managee_actions: String,
    pub highlights: String,
    pub mood: u8,
    pub goals: String,
    pub previous_actions_status: String,
    pub check_point_date: SystemTime,
}
