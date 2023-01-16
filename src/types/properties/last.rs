use crate::types::core::link::Link;
use crate::types::errors::TypeError;
use url::Url;

/// In a paged [Collection](crate::types::core::collection::Collection), indicates the furthest
/// proceeding page of the collection..
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-last>
#[derive(Debug, PartialEq, Eq)]
pub struct Last(LastType);

impl Last {
    pub fn new(value: LastType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LastType {
    Url(Url),
    Link(Link),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Last::new(LastType::Url(
            Url::try_from("https://example.org/foo").unwrap(),
        ));

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Last(LastType::Url(
                Url::try_from("https://example.org/foo").unwrap(),
            ))
        );
    }
}
