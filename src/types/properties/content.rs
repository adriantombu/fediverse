use crate::types::errors::TypeError;
use std::collections::HashMap;

/// The content or textual representation of the Object encoded as a JSON string. By default, the
/// value of `content` is HTML. The [mediaType](crate::types::properties::media_type) property can
/// be used in the object to indicate a different content type.
///
/// The content MAY be expressed using multiple language-tagged values.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-content>
#[derive(Debug, PartialEq, Eq)]
pub struct Content(ContentType);

impl Content {
    pub fn new(value: ContentType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContentType {
    Default(String),
    I18n(HashMap<String, String>), // TODO: check proper key language?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single() {
        let content = Content::new(ContentType::Default("A <em>simple</em> note".to_string()));

        assert!(content.is_ok());
        assert_eq!(
            content.unwrap(),
            Content(ContentType::Default("A <em>simple</em> note".to_string()))
        );
    }

    #[test]
    fn test_new_i18n() {
        let mut values = HashMap::new();
        values.insert("en".to_string(), "A <em>simple</em> note".to_string());
        values.insert("es".to_string(), "Una nota <em>sencilla</em>".to_string());
        values.insert("zh-Hans".to_string(), "一段<em>简单的</em>笔记".to_string());

        let contents = Content::new(ContentType::I18n(values.clone()));

        assert!(contents.is_ok());
        assert_eq!(contents.unwrap().0, ContentType::I18n(values));
    }
}
