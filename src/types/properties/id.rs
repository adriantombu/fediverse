use crate::types::errors::TypeError;
use url::Url;

/// Provides the globally unique identifier for an [Object](crate::types::core::object::Object) or [Link](crate::types::core::link::Link).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-id>
#[derive(Debug, PartialEq, Eq)]
pub struct Id(Url);

impl Id {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = Id::new("https://example.org/foo");
        assert!(id.is_ok());

        let value = id.unwrap();
        assert_eq!(value, Id(Url::parse("https://example.org/foo").unwrap()));
        assert_eq!(value.to_string(), "https://example.org/foo");
    }

    #[test]
    fn test_new_error() {
        let id = Id::new("example/foo");

        assert!(id.is_err());
    }
}
