pub mod middleware;

use async_trait::async_trait;
use std::error::Error;

#[async_trait]
/// Model Permission
pub trait ModelPermission<T, A>
where
    T: Sized,
{
    /// Returns the value of the attr
    ///
    /// if its exists
    async fn get_attr(&self, name: T) -> Option<A>
    where
        A: Sized;
}

#[async_trait]
pub trait Permission {
    type Output;

    /// Check if the permissions are valid
    async fn check<E>(
        &self,
        subject: String,
        path: &'static ParsedPath,
        method: String,
    ) -> Self::Output;

    /// Get model
    ///
    /// Name defines wich model we should check
    /// for attrs
    async fn get_model<T, A, M>(&self, name: &str) -> M
    where
        M: ModelPermission<T, A> + Sized;
}

#[derive(Debug, Clone)]
pub struct ParsedPath<'a> {
    pub controller: Option<&'a str>,
    pub action: Option<&'a str>,
    pub id: Option<&'a str>,
}

impl Default for ParsedPath<'_> {
    fn default() -> Self {
        Self {
            controller: None,
            action: None,
            id: None,
        }
    }
}

impl<'a> From<&'a str> for ParsedPath<'a> {
    /// value must start with '/'
    ///
    /// `/controller/action/id`
    ///
    /// `/controller/id`
    ///
    /// `/controller`
    fn from(value: &'a str) -> Self {
        let mut splited = value.split('/').skip_while(|s| s.is_empty());

        if splited.clone().count() >= 3 {
            return Self {
                controller: splited.next(),
                action: splited.next(),
                id: splited.next(),
            };
        } else {
            return Self {
                controller: splited.next(),
                action: None,
                id: splited.next(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_path_from_str() {
        let parsed_path = ParsedPath::from("/controller/action/id");

        assert_eq!(parsed_path.controller, Some("controller"));
        assert_eq!(parsed_path.action, Some("action"));
        assert_eq!(parsed_path.id, Some("id"));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_action() {
        let parsed_path = ParsedPath::from("/controller/id");

        assert_eq!(parsed_path.controller, Some("controller"));
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, Some("id"));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_id_and_action() {
        let parsed_path = ParsedPath::from("/controller");

        assert_eq!(parsed_path.controller, Some("controller"));
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, None);
    }

    #[test]
    fn test_empty_url() {
        let parsed_path = ParsedPath::from("/");

        assert_eq!(parsed_path.controller, None);
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, None);
    }
}
