use crate::types::errors::TypeError;
use url::Url;

/// The target resource pointed to by a Link.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-href
#[derive(Debug, PartialEq, Eq)]
pub struct PartOf(Url);

impl PartOf {
    fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let partOf = PartOf::new("http://example.org/abc");

        assert!(partOf.is_ok());
        assert_eq!(
            partOf.unwrap(),
            PartOf(Url::parse("http://example.org/abc").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let partOf = PartOf::new("example/abc");

        assert!(partOf.is_err());
    }
}
