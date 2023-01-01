use crate::types::errors::TypeError;
use url::Url;

// TODO: handle Link object
/// In a paged [Collection](crate::types::core::collection::Collection), indicates the furthest
/// preceeding page of items in the collection.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first>
#[derive(Debug, PartialEq, Eq)]
pub struct First(Url);

impl First {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = First::new("https://example.org/foo");

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            First(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let t = First::new("example/foo");

        assert!(t.is_err());
    }
}
