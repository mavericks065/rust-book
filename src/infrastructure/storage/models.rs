use diesel::sql_types::Date;
use crate::infrastructure::storage::schema::{companies, employees, follow_ups, check_points};
use chrono::NaiveDateTime;
use std::collections::LinkedList;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "companies"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
    pub ceo_id: i32,
    pub created_at: NaiveDateTime
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
    pub company_id: i32,
    pub follow_up_ids: Vec<i64>,
    pub created_at: NaiveDateTime
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "follow_ups"]
pub struct FollowUp {
    pub id: i64,
    pub managee_id: i32,
    pub check_points_ids: LinkedList<i64>
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "check_points"]
pub struct CheckPoint {
    pub id: i64,
    pub comments: String,
    pub manager_actions: String,
    pub managee_actions: String,
    pub highlights: String,
    pub mood: i8,
    pub goals: String,
    pub previous_actions_status: String,
    pub check_point_date: NaiveDateTime
}

