use crate::types::errors::TypeError;
use url::Url;

// TODO: handle Link object & list of mixed url + Link object
/// Describes one or more entities that either performed or are expected to perform the activity.
/// Any single activity can have multiple actors. The actor MAY be specified using an indirect Link.
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-actor>
#[derive(Debug, PartialEq, Eq)]
pub struct Actor(Url);

impl Actor {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Actor::new("https://example.org/foo");

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Actor(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let t = Actor::new("example/foo");

        assert!(t.is_err());
    }
}
