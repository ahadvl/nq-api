use crate::models::User;
use async_trait::async_trait;
use auth_z::ModelPermission;

enum ModelAttrib {}

#[async_trait]
impl ModelPermission<ModelAttrib, u32> for User {
    async fn get_attr(&self, name: ModelAttrib) -> Option<u32> {
        return Some(1);
    }
}
