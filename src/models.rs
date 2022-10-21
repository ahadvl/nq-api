use crate::schema::app_verify_code;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct VerifyCode {
    pub id: i32,
    pub code: i32,
    pub email: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_verify_code)]
pub struct NewVerifyCode<'a> {
    pub status: &'a String,
    pub code: &'a i32,
    pub email: &'a String,
}
