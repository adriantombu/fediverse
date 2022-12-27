use crate::types::errors::TypeError;
use url::Url;

/// Identifies an Object that is part of the private primary audience of this Object.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-bto
#[derive(Debug, PartialEq, Eq)]
pub struct Bto(Vec<Url>);

impl Bto {
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
        let bto = Bto::new(vec!["https://example.org/foo"]);

        assert!(bto.is_ok());
        assert_eq!(
            bto.unwrap(),
            Bto(vec![Url::parse("https://example.org/foo").unwrap()])
        );
    }

    #[test]
    fn test_new_error() {
        let bto = Bto::new(vec!["example/foo"]);

        assert!(bto.is_err());
    }
}
