use crate::types::errors::TypeError;
use std::collections::HashMap;

/// A natural language summarization of the object encoded as HTML. I18n language tagged summaries
/// MAY be provided.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-summary>
#[derive(Debug, PartialEq, Eq)]
pub struct Summary(SummaryType);

impl Summary {
    pub fn new(value: SummaryType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SummaryType {
    Default(String),
    I18n(HashMap<String, String>), // TODO: check proper key language?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single() {
        let summary = Summary::new(SummaryType::Default("A <em>simple</em> note".to_string()));

        assert!(summary.is_ok());
        assert_eq!(
            summary.unwrap().0,
            SummaryType::Default("A <em>simple</em> note".to_string())
        );
    }

    #[test]
    fn test_new_i18n() {
        let mut values = HashMap::new();
        values.insert("en".to_string(), "A <em>simple</em> note".to_string());
        values.insert("es".to_string(), "Una nota <em>sencilla</em>".to_string());
        values.insert("zh-Hans".to_string(), "一段<em>简单的</em>笔记".to_string());

        let summaries = Summary::new(SummaryType::I18n(values.clone()));

        assert!(summaries.is_ok());
        assert_eq!(summaries.unwrap().0, SummaryType::I18n(values));
    }
}
