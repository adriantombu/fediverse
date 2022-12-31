use crate::types::errors::TypeError;
use url::Url;

/// The target resource pointed to by a Link.
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-href>
#[derive(Debug, PartialEq, Eq)]
pub struct PartOf(Url);

impl PartOf {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let part_of = PartOf::new("https://example.org/abc");

        assert!(part_of.is_ok());
        assert_eq!(
            part_of.unwrap(),
            PartOf(Url::parse("https://example.org/abc").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let part_of = PartOf::new("example/abc");

        assert!(part_of.is_err());
    }
}
