use crate::types::errors::TypeError;
use std::collections::HashMap;
use voca_rs::Voca;

/// A simple, human-readable, plain-text name for the object. HTML markup MUST NOT be included. The name MAY be expressed using multiple language-tagged values.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-name
#[derive(Debug, PartialEq, Eq)]
pub struct NameMap(HashMap<String, String>);

impl NameMap {
    fn new(values: HashMap<&str, &str>) -> Result<Self, TypeError> {
        let mut names: HashMap<String, String> = HashMap::new();

        for (k, v) in values {
            names.insert(k.to_string(), v.to_string()._strip_tags());
        }

        Ok(Self(names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut values = HashMap::new();
        values.insert("en", "A simple note");
        values.insert("es", "Una nota sencilla");
        values.insert("zh-Hans", "一段简单的笔记");

        let names = NameMap::new(values);

        assert!(names.is_ok());
        assert_eq!(names.unwrap().0.len(), 3);
    }
}
