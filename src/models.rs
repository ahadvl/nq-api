use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct VerifyCode {
    id: i32,
    status: Option<String>,
    code: Option<i32>,
    email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
