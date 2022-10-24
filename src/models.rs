use crate::schema::{app_users, app_verify_codes};
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};

#[derive(Identifiable, Queryable, Debug)]
#[diesel(table_name = app_verify_codes)]
pub struct VerifyCode {
    pub id: i32,
    pub code: i32,
    pub email: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_verify_codes)]
pub struct NewVerifyCode<'a> {
    pub status: &'a String,
    pub code: &'a i32,
    pub email: &'a String,
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = app_users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_users)]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub email: &'a String,
}
