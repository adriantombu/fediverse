use crate::types::errors::TypeError;
use std::collections::HashMap;

///  A natural language summarization of the object encoded as HTML. Multiple language tagged summaries MAY be provided.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-summary
#[derive(Debug, PartialEq, Eq)]
pub struct SummaryMap(HashMap<String, String>);

impl SummaryMap {
    pub fn new(values: HashMap<&str, &str>) -> Result<Self, TypeError> {
        let mut summaries: HashMap<String, String> = HashMap::new();

        for (k, v) in values {
            summaries.insert(k.to_string(), v.to_string());
        }

        Ok(Self(summaries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut values = HashMap::new();
        values.insert("en", "A <em>simple</em> note");
        values.insert("es", "Una nota <em>sencilla</em>");
        values.insert("zh-Hans", "一段<em>简单的</em>笔记");

        let summaries = SummaryMap::new(values);

        assert!(summaries.is_ok());
        assert_eq!(summaries.unwrap().0.len(), 3);
    }
}
