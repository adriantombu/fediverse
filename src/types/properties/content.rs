use crate::types::errors::TypeError;

/// The content or textual representation of the Object encoded as a JSON string. By default, the value of content is HTML. The mediaType property can be used in the object to indicate a different content type.
/// The content MAY be expressed using multiple language-tagged values.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-content
#[derive(Debug, PartialEq, Eq)]
pub struct Content(String);

impl Content {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let content = Content::new("A <em>simple</em> note");

        assert!(content.is_ok());
        assert_eq!(
            content.unwrap(),
            Content("A <em>simple</em> note".to_string())
        );
    }
}
