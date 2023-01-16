use crate::types::errors::TypeError;
use std::collections::HashMap;
use voca_rs::strip;

/// A simple, human-readable, plain-text name for the object. HTML markup MUST NOT be included.
/// The name MAY be expressed using multiple language-tagged values.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-name>
#[derive(Debug, PartialEq, Eq)]
pub struct Name(NameType);

impl Name {
    pub fn new(value: NameType) -> Result<Self, TypeError> {
        Ok(Self(match value {
            NameType::Default(value) => NameType::Default(strip::strip_tags(&value)),
            NameType::I18n(values) => {
                let mut val = HashMap::new();
                for (k, v) in values {
                    val.insert(k, strip::strip_tags(&v));
                }

                NameType::I18n(val)
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NameType {
    Default(String),
    I18n(HashMap<String, String>), // TODO: check proper key language?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single() {
        let content = Name::new(NameType::Default("A simple note".to_string()));

        assert!(content.is_ok());
        assert_eq!(
            content.unwrap(),
            Name(NameType::Default("A simple note".to_string()))
        );
    }

    #[test]
    fn test_new_single_stripped_tags() {
        let content = Name::new(NameType::Default("A <em>simple</em> note".to_string()));

        assert!(content.is_ok());
        assert_eq!(
            content.unwrap(),
            Name(NameType::Default("A simple note".to_string()))
        );
    }

    #[test]
    fn test_new_i18n() {
        let mut values = HashMap::new();
        values.insert("en".to_string(), "A simple note".to_string());
        values.insert("es".to_string(), "Una nota sencilla".to_string());
        values.insert("zh-Hans".to_string(), "一段简单的笔记".to_string());

        let contents = Name::new(NameType::I18n(values.clone()));

        assert!(contents.is_ok());
        assert_eq!(contents.unwrap().0, NameType::I18n(values));
    }
}
