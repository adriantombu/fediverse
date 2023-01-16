use crate::types::core::link::Link;
use crate::types::errors::TypeError;
use url::Url;

/// In a paged [Collection](crate::types::core::collection::Collection), indicates the furthest
/// preceeding page of items in the collection.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first>
#[derive(Debug, PartialEq, Eq)]
pub struct First(FirstType);

impl First {
    pub fn new(value: FirstType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FirstType {
    Url(Url),
    Link(Link),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = First::new(FirstType::Url(
            Url::try_from("https://example.org/foo").unwrap(),
        ));

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            First(FirstType::Url(
                Url::try_from("https://example.org/foo").unwrap(),
            ))
        );
    }
}
