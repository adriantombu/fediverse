use crate::types::core::link::Link;
use crate::types::errors::TypeError;
use url::Url;

/// In a paged [Collection](crate::types::core::collection::Collection), indicates the page that
/// contains the most recently updated member items.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-current>
#[derive(Debug, PartialEq, Eq)]
pub struct Current(CurrentType);

impl Current {
    pub fn new(value: CurrentType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentType {
    Url(Url),
    Link(Link),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Current::new(CurrentType::Url(
            Url::try_from("https://example.org/foo").unwrap(),
        ));

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Current(CurrentType::Url(
                Url::try_from("https://example.org/foo").unwrap(),
            ))
        );
    }
}
