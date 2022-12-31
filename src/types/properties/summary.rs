use crate::types::errors::TypeError;

/// A natural language summarization of the object encoded as HTML. Multiple language tagged summaries MAY be provided.
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-summary>
#[derive(Debug, PartialEq, Eq)]
pub struct Summary(String);

impl Summary {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let summary = Summary::new("A <em>simple</em> note");

        assert!(summary.is_ok());
        assert_eq!(
            summary.unwrap(),
            Summary("A <em>simple</em> note".to_string())
        );
    }
}
