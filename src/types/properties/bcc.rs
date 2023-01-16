use crate::types::errors::TypeError;
use url::Url;

/// Identifies one or more Objects that are part of the private secondary audience of this Object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-bcc>
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Bcc(Vec<Url>);

impl Bcc {
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
        let bcc = Bcc::new(vec!["https://example.org/foo"]);

        assert!(bcc.is_ok());
        assert_eq!(
            bcc.unwrap(),
            Bcc(vec![Url::parse("https://example.org/foo").unwrap()])
        );
    }

    #[test]
    fn test_new_error() {
        let bcc = Bcc::new(vec!["example/foo"]);

        assert!(bcc.is_err());
    }
}
