use crate::{
    models::{Organization, User},
    select_model, DbPool,
};

// select_model macro requirements
use actix_web::web::block;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait SelectModel {
    async fn from_id(conn: DbPool, id: i32) -> Self
    where
        Self: Sized;
}

select_model!(Organization, app_organizations);
select_model!(User, app_users);
