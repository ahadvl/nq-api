use crate::error::RouterError;
use crate::models::{Organization, User};
use crate::select_model::SelectModel;
use crate::DbPool;
use actix_web::web;
use async_trait::async_trait;
use auth_z::{CheckPermission, GetModel, ModelPermission, ParsedPath};
use diesel::prelude::*;

const ANY_FILTER: char = '*';

#[derive(Debug)]
/// Request Action
enum Action {
    /// Create or POST request to a controller
    Create,

    /// Edit or POST request with id to a controller
    Edit,

    /// Delete or DELETE request with id to a controller
    Delete,

    /// View or GET request to a controller, id is not required
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
/// Actual Context of AuthZ
pub struct AuthZController {
    db_pool: DbPool,
}

impl AuthZController {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CheckPermission for AuthZController {
    //type Output = Result<(), RouterError>;
    async fn check(&self, subject: String, path: ParsedPath, method: String) -> bool {
        use crate::schema::app_permission_conditions::dsl::{
            app_permission_conditions, name, value,
        };
        use crate::schema::app_permissions::dsl::{
            action as permission_action, app_permissions, id as permission_id,
            object as permission_object, subject as permission_subject,
        };

        let subject_copy = subject.clone();
        let mut conn = self.db_pool.get().unwrap();

        let path_copy = path.clone();
        let select_result: Result<(Vec<i32>, Vec<(String, Option<String>)>), RouterError> =
            web::block(move || {
                // Default subject query
                let subject_query = vec![subject_copy, ANY_FILTER.to_string()];

                // Foundout the requested Action
                let calculated_action = Action::from_auth_z(&path_copy, method.as_str());

                // Check the permissions and get the conditions
                let permissions_filter = app_permissions
                    .filter(permission_subject.eq_any(subject_query))
                    .filter(permission_object.eq(path_copy.controller.unwrap().clone()))
                    .filter(permission_action.eq::<&str>(calculated_action.into()));

                let permissions = permissions_filter
                    .clone()
                    .select(permission_id)
                    .load(&mut conn)?;

                let conditions = permissions_filter
                    .inner_join(app_permission_conditions)
                    .select((name, value))
                    .load(&mut conn)?;

                Ok((permissions, conditions))
            })
            .await
            .unwrap();

        let Ok(select_result) = select_result else {
            return false;
        };

        if select_result.0.is_empty() {
            return false;
        }

        // No need to Checking the conditions
        // there is no condition
        if select_result.1.is_empty() {
            return true;
        }

        // *Now Check the conditions*

        // First get the required Resource as Model
        let model = self
            .get_model(
                &path.controller.unwrap().clone(),
                path.id.unwrap().clone().parse().unwrap(),
            )
            .await;

        // TODO: Better Way ?
        //
        let mut result = false;
        // We Got the model now we check every condition
        for (cond_name, cond_value) in select_result.1 {
            let attr = model.get_attr(ModelAttrib::from(cond_name.as_str())).await;

            let res = match cond_value {
                Some(_v) => matches!(attr, Some(_)) && attr.unwrap().to_string() == subject,
                None => true,
            };

            result = res;
        }

        result
    }
}

#[async_trait]
impl GetModel<ModelAttrib, i32> for AuthZController {
    async fn get_model(
        &self,
        resource_name: &str,
        resource_id: u32,
    ) -> Box<dyn ModelPermission<ModelAttrib, i32>> {
        //let mut conn = self.db_pool.get().unwrap();
        let resource_id = resource_id as i32;

        // Resource must have been impl the Model permission trait
        let model: Box<dyn ModelPermission<ModelAttrib, i32>> = match resource_name {
            "user" => Box::new(User::from_id(self.db_pool.clone(), resource_id).await),

            "organization" => {
                Box::new(Organization::from_id(self.db_pool.clone(), resource_id).await)
            }

            _ => todo!(),
        };

        model
    }
}

enum ModelAttrib {
    Owner,
}

// Maybe we can use TryFrom
impl From<&str> for ModelAttrib {
    fn from(value: &str) -> Self {
        match value {
            "owner" => Self::Owner,

            _ => panic!(),
        }
    }
}

#[async_trait]
impl ModelPermission<ModelAttrib, i32> for User {
    async fn get_attr(&self, name: ModelAttrib) -> Option<i32> {
        match name {
            ModelAttrib::Owner => Some(self.account_id),
        }
    }
}

#[async_trait]
impl ModelPermission<ModelAttrib, i32> for Organization {
    async fn get_attr(&self, name: ModelAttrib) -> Option<i32> {
        match name {
            ModelAttrib::Owner => Some(self.account_id),
        }
    }
}
