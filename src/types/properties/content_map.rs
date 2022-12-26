use crate::types::errors::TypeError;
use std::collections::HashMap;

/// The content or textual representation of the Object encoded as a JSON string. By default, the value of content is HTML. The mediaType property can be used in the object to indicate a different content type.
/// The content MAY be expressed using multiple language-tagged values.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-content
#[derive(Debug, PartialEq, Eq)]
pub struct ContentMap(HashMap<String, String>);

impl ContentMap {
    pub fn new(values: HashMap<&str, &str>) -> Result<Self, TypeError> {
        let mut contents: HashMap<String, String> = HashMap::new();

        for (k, v) in values {
            contents.insert(k.to_string(), v.to_string());
        }

        Ok(Self(contents))
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

        let contents = ContentMap::new(values);

        assert!(contents.is_ok());
        assert_eq!(contents.unwrap().0.len(), 3);
    }
}
