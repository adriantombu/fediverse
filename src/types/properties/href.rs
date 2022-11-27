use crate::types::errors::TypeError;
use url::Url;

/// The target resource pointed to by a Link.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-href
#[derive(Debug, PartialEq, Eq)]
pub struct Href(Url);

impl Href {
    fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let href = Href::new("http://example.org/abc");

        assert!(href.is_ok());
        assert_eq!(
            href.unwrap(),
            Href(Url::parse("http://example.org/abc").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let href = Href::new("example/abc");

        assert!(href.is_err());
    }
}
