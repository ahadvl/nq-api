use crate::datetime::parse_date_time_with_format;
use crate::schema::{app_emails, app_organizations_table, app_tokens, app_users, app_verify_codes};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Identifiable, Queryable, Debug, Clone, Serialize)]
#[diesel(table_name = app_users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDateTime>,
    pub profile_image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Clone, Serialize)]
pub struct UserProfile {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDateTime>,
    pub profile_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = app_users)]
pub struct NewUser<'a> {
    pub username: &'a String,
}

// TODO: use belongs to
#[derive(Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = app_tokens)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub terminated: bool,
    pub teminated_by_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_tokens)]
pub struct NewToken<'a> {
    pub user_id: &'a i32,
    pub token_hash: &'a String,
}

#[derive(Queryable, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct QuranText {
    id: i32,
    surah: i32,
    verse: i32,
    text: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = app_emails)]
pub struct Email {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub verified: bool,
    pub primary: bool,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_emails)]
pub struct NewEmail<'a> {
    pub user_id: &'a i32,
    pub email: &'a String,
    pub verified: bool,
    pub primary: bool,
    pub deleted: bool,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Clone)]
#[diesel(table_name = app_organizations_table)]
pub struct Organization {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = app_organizations_table)]
pub struct NewOrganization {
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,

    #[serde(deserialize_with = "parse_date_time_with_format")]
    pub established_date: NaiveDate,

    #[validate(length(equal = 11))]
    pub national_id: String,
}
