use crate::types::errors::TypeError;
use url::Url;

/// Identifies an Object that is part of the public secondary audience of this Object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-cc>
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Cc(Vec<Url>);

impl Cc {
    pub fn new(values: Vec<&str>) -> Result<Self, TypeError> {
        let mut urls = vec![];
        for value in values {
            urls.push(Url::parse(value)?);
        }

        Ok(Self(urls))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cc = Cc::new(vec!["https://example.org/foo"]);

        assert!(cc.is_ok());
        assert_eq!(
            cc.unwrap(),
            Cc(vec![Url::parse("https://example.org/foo").unwrap()])
        );
    }

    #[test]
    fn test_new_error() {
        let cc = Cc::new(vec!["example/foo"]);

        assert!(cc.is_err());
    }
}
