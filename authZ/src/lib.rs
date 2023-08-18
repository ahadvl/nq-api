pub mod middleware;

use async_trait::async_trait;

#[async_trait]
/// Model Permission
pub trait ModelPermission<T, A> : Sync + Send
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
pub trait CheckPermission {
    /// Check if the permissions are valid
    async fn check(&self, subject: String, path: ParsedPath, method: String) -> bool;
}

#[async_trait]
pub trait GetModel<T, A>
where
    T: Sized,
    A: Sized,
{
    /// Get model
    ///
    /// Name defines wich model we should check
    /// for attrs
    ///
    /// TODO: maybe Resource Id is not always Int(u32) ?
    async fn get_model(
        &self,
        resource_name: &str,
        resource_id: u32,
    ) -> Box<dyn ModelPermission<T, A>>;
}

#[derive(Debug, Clone)]
pub struct ParsedPath {
    pub controller: Option<String>,
    pub action: Option<String>,
    pub id: Option<String>,
}

impl Default for ParsedPath {
    fn default() -> Self {
        Self {
            controller: None,
            action: None,
            id: None,
        }
    }
}

impl<'a> From<&'a str> for ParsedPath {
    /// value must start with '/'
    ///
    /// `/controller/action/id`
    ///
    /// `/controller/id`
    ///
    /// `/controller`
    fn from(value: &'a str) -> Self {
        let mut splited = value
            .split('/')
            .skip_while(|s| s.is_empty())
            .map(|c| c.to_string());

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

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
        assert_eq!(parsed_path.action, Some("action".to_string()));
        assert_eq!(parsed_path.id, Some("id".to_string()));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_action() {
        let parsed_path = ParsedPath::from("/controller/id");

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, Some("id".to_string()));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_id_and_action() {
        let parsed_path = ParsedPath::from("/controller");

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
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
