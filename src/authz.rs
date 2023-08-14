use crate::DbPool;
use crate::{error::RouterError, models::User};
use actix_web::web;
use async_trait::async_trait;
use auth_z::{ModelPermission, ParsedPath, Permission};
use diesel::prelude::*;

enum Action {
    Create,
    Edit,
    Delete,
    View,
}

impl Action {
    fn from_auth_z<'a>(path: &ParsedPath<'a>, method: &'a str) -> Self {
        match (path.id, method) {
            (Some(_), "GET") => Self::View,
            (None, "POST") => Self::Create,
            (Some(_), "POST") => Self::Edit,
            (Some(_), "DELETE") => Self::Delete,
            (None, _) => Self::View,
            (Some(_), _) => Self::View,
        }
    }
}

impl Into<&str> for Action {
    fn into(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Edit => "edit",
            Self::Delete => "delete",
            Self::View => "view",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthZController {
    db_pool: DbPool,
}

impl AuthZController {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Permission for AuthZController {
    type Output = Result<(), RouterError>;
    async fn check<E>(
        &self,
        subject: String,
        path: &'static ParsedPath, // Is this a good thing ?
        method: String,
    ) -> Self::Output {
        use crate::models::Permission;
        use crate::schema::app_permissions::dsl::{
            action as permission_action, app_permissions, object as permission_object,
            subject as permission_subject,
        };

        let subject = subject.clone();
        let mut conn = self.db_pool.get().unwrap();

        let result = web::block(move || {
            let subject_query = vec![subject, "*".to_string()];

            let calculated_action = Action::from_auth_z(path, method.as_str());

            let permission: Vec<Permission> = app_permissions
                .filter(permission_subject.eq_any(subject_query))
                .filter(permission_object.eq(path.controller.unwrap().clone()))
                .filter(permission_action.eq::<&str>(calculated_action.into()))
                .load(&mut conn)?;

            if permission.is_empty() {
                return Err(RouterError::Unauth(
                    "You don't have access to this resource or this action.".to_string(),
                ));
            }

            return Ok(());
        })
        .await
        .unwrap();

        result
    }

    async fn get_model<T, A, M>(&self, name: &str) -> M
    where
        M: ModelPermission<T, A> + Sized,
    {
        todo!()
    }
}

enum ModelAttrib {
    Owner,
}

#[async_trait]
impl ModelPermission<ModelAttrib, i32> for User {
    async fn get_attr(&self, name: ModelAttrib) -> Option<i32> {
        match name {
            ModelAttrib::Owner => Some(self.account_id),
        }
    }
}
