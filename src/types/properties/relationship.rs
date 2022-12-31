use crate::types::errors::TypeError;
use url::Url;

/// On a Relationship object, the relationship property identifies the kind of relationship that exists between subject and object.
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship>
#[derive(Debug, PartialEq, Eq)]
pub struct Relationship(Url);

impl Relationship {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let relationship = Relationship::new("https://purl.org/vocab/relationship/acquaintanceOf");

        assert!(relationship.is_ok());
        assert_eq!(
            relationship.unwrap(),
            Relationship(Url::parse("https://purl.org/vocab/relationship/acquaintanceOf").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let relationship = Relationship::new("purl/vocab/relationship/acquaintanceOf");

        assert!(relationship.is_err());
    }
}
