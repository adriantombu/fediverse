use crate::types::errors::TypeError;
use url::Url;

/// Identifies the context within which the object exists or an activity was performed.
/// The notion of "context" used is intentionally vague. The intended function is to serve as a means
/// of grouping objects and activities that share a common originating context or purpose. An example
/// could be all activities relating to a common project or event.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context>
#[derive(Debug, PartialEq, Eq)]
pub struct Context(Url);

impl Context {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let context = Context::new("https://example.org/foo");

        assert!(context.is_ok());
        assert_eq!(
            context.unwrap(),
            Context(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let context = Context::new("example/foo");

        assert!(context.is_err());
    }
}
