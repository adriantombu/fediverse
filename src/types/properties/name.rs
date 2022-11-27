use crate::types::errors::TypeError;
use voca_rs::Voca;

/// A simple, human-readable, plain-text name for the object. HTML markup MUST NOT be included. The name MAY be expressed using multiple language-tagged values.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-name
#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(value.to_string()._strip_tags()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = Name::new("A simple note");

        assert!(name.is_ok());
        assert_eq!(name.unwrap(), Name("A simple note".to_string()));
    }
}
