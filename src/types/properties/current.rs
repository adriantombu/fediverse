use crate::types::errors::TypeError;
use url::Url;

// TODO: handle Link object
/// In a paged [Collection](crate::types::core::collection::Collection), indicates the page that
/// contains the most recently updated member items.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-current>
#[derive(Debug, PartialEq, Eq)]
pub struct Current(Url);

impl Current {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Current::new("https://example.org/foo");

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Current(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let t = Current::new("example/foo");

        assert!(t.is_err());
    }
}
