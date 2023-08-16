use crate::models::User;
use crate::DbPool;
use actix_web::web;
use async_trait::async_trait;
use auth_z::{ModelPermission, ParsedPath, Permission};
use diesel::prelude::*;

const ANY_FILTER: char = '*';

#[derive(Debug)]
enum Action {
    Create,
    Edit,
    Delete,
    View,
}

impl Action {
    fn from_auth_z<'a>(path: &ParsedPath, method: &'a str) -> Self {
        // Checks the id of path and request method
        match (path.id.clone(), method) {
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
    //type Output = Result<(), RouterError>;
    async fn check(&self, subject: String, path: ParsedPath, method: String) -> bool {
        use crate::schema::app_permission_conditions::dsl::{
            app_permission_conditions, name, value,
        };
        use crate::schema::app_permissions::dsl::{
            action as permission_action, app_permissions, object as permission_object,
            subject as permission_subject,
        };

        let subject = subject.clone();
        let mut conn = self.db_pool.get().unwrap();

        let result = web::block(move || {
            // Default subject query
            let subject_query = vec![subject, ANY_FILTER.to_string()];

            // Foundout the requested Action
            let calculated_action = Action::from_auth_z(&path, method.as_str());

            // Check the permissions and get the conditions
            let conditions: Vec<(String, Option<String>)> = app_permissions
                .filter(permission_subject.eq_any(subject_query))
                .filter(permission_object.eq(path.controller.unwrap().clone()))
                .filter(permission_action.eq::<&str>(calculated_action.into()))
                .inner_join(app_permission_conditions)
                .select((name, value))
                .load(&mut conn)
                .unwrap();

            if conditions.is_empty() {
                return false;
            }

            // Now Check the conditions
            true
        })
        .await
        .unwrap();

        result
    }

    async fn get_model<T, A, M>(&self, resource_name: &str, condition_name: &str) -> M
    where
        M: ModelPermission<T, A> + Sized,
    {
        match (resource_name, condition_name) {
            ("user", "owner") => todo!(),
            _ => todo!(),
        }
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
