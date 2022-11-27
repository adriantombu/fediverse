use crate::types::errors::TypeError;
use url::Url;

/// Provides the globally unique identifier for an Object or Link.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-id
#[derive(Debug, PartialEq, Eq)]
pub struct Id(Url);

impl Id {
    fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = Id::new("http://example.org/foo");

        assert!(id.is_ok());
        assert_eq!(
            id.unwrap(),
            Id(Url::parse("http://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let id = Id::new("example/foo");

        assert!(id.is_err());
    }
}
