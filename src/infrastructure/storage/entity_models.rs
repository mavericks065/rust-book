use crate::infrastructure::storage::schema::{check_points, companies, employees, follow_ups};
use chrono::NaiveDateTime;
use diesel::sql_types::Date;
use std::collections::LinkedList;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "companies"]
pub struct CompanyEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: u64,
    pub city: String,
    pub country: String,
    pub abn: String,
    pub ceo_id: u64,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "employees"]
pub struct EmployeeEntity {
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub address: String,
    pub post_code: u32,
    pub city: String,
    pub country: String,
    pub abn: String,
    pub dob: Date,
    pub salary: u64,
    pub company_id: u64,
    pub follow_up_ids: Vec<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "follow_ups"]
pub struct FollowUpEntity {
    pub id: u64,
    pub managee_id: u64,
    pub check_points_ids: LinkedList<u64>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "check_points"]
pub struct CheckPointEntity {
    pub id: u64,
    pub comments: String,
    pub manager_actions: String,
    pub managee_actions: String,
    pub highlights: String,
    pub mood: u8,
    pub goals: String,
    pub previous_actions_status: String,
    pub check_point_date: NaiveDateTime,
}
