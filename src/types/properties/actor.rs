use crate::types::core::object::Object;
use crate::types::errors::TypeError;
use url::Url;

/// Describes one or more entities that either performed or are expected to perform the activity.
/// Any single activity can have multiple `actor`s. The `actor` MAY be specified using an indirect [Link](crate::types::core::link::Link).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-actor>
#[derive(Debug, PartialEq)]
pub struct Actor(ActorType);

impl Actor {
    pub fn new(value: ActorType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum ActorType {
    Url(Url),
    Object(Object),
    Values(Vec<ActorTypeValues>),
}

#[derive(Debug, PartialEq)]
pub enum ActorTypeValues {
    Url(Url),
    Object(Object),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_link() {
        let t = Actor::new(ActorType::Url(
            Url::try_from("https://example.org/foo").unwrap(),
        ));

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Actor(ActorType::Url(
                Url::try_from("https://example.org/foo").unwrap()
            ))
        );
    }
}
