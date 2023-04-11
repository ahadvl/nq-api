use crate::schema::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Identifiable, Queryable, Debug, Serialize)]
#[diesel(table_name = app_accounts)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub account_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = app_accounts)]
pub struct NewAccount<'a> {
    pub username: &'a String,
    pub account_type: &'a String,
}

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

#[derive(Associations, Identifiable, Queryable, Debug, Clone, Serialize)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_users)]
pub struct User {
    pub id: i32,
    pub account_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = app_users)]
pub struct NewUser {
    pub account_id: i32,
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

#[derive(Queryable, Insertable)]
#[diesel(table_name = app_tokens)]
pub struct NewToken<'a> {
    pub account_id: &'a i32,
    pub token_hash: &'a String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_emails)]
pub struct Email {
    pub id: i32,
    pub account_id: i32,
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
    pub account_id: i32,
    pub email: &'a String,
    pub verified: bool,
    pub primary: bool,
    pub deleted: bool,
}

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Serialize, Clone)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_organizations)]
pub struct Organization {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = app_organizations)]
pub struct NewOrganization {
    pub account_id: i32,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

#[derive(Queryable, Deserialize, Validate)]
#[diesel(table_name = app_employees)]
pub struct Employee {
    pub id: i32,
    pub org_account_id: i32,
    pub employee_account_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = app_employees)]
pub struct NewEmployee {
    pub org_account_id: i32,
    pub employee_account_id: i32,
}

#[derive(
    Selectable,
    Insertable,
    Deserialize,
    Validate,
    Queryable,
    Identifiable,
    Serialize,
    Associations,
    Clone,
    Debug,
)]
#[diesel(belongs_to(QuranSurah, foreign_key = surah_id))]
#[diesel(table_name = quran_ayahs)]
pub struct QuranAyah {
    pub id: i32,
    #[serde(skip_serializing)]
    pub surah_id: i32,

    pub ayah_number: i32,
    pub sajdeh: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Selectable, Identifiable, Associations, Queryable, PartialEq, Debug, Serialize)]
#[diesel(belongs_to(QuranAyah, foreign_key = ayah_id))]
#[diesel(table_name = quran_words)]
pub struct QuranWord {
    pub id: i32,

    #[serde(skip_serializing)]
    pub ayah_id: i32,

    pub word: String,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(
    Serialize, Clone, Insertable, Deserialize, Validate, Identifiable, Queryable, Selectable, Debug,
)]
#[diesel(table_name = quran_surahs)]
pub struct QuranSurah {
    pub id: i32,

    pub name: String,
    pub period: Option<String>,
    pub number: i32,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(
    Serialize, Clone, Insertable, Deserialize, Validate, Identifiable, Queryable, Selectable, Debug,
)]
#[diesel(table_name = mushafs)]
pub struct QuranMushaf {
    pub id: i32,

    pub name: Option<String>,
    pub source: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}
