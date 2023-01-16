use crate::types::core::object::ObjectProperties;

/// Represents a document of any kind.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document>
#[derive(Default, Debug, PartialEq)]
pub struct Document {
    pub r#type: DocumentType,
    pub object_properties: ObjectProperties,
}

impl Document {
    pub fn new(r#type: DocumentType, object_properties: ObjectProperties) -> Self {
        Self {
            r#type,
            object_properties,
        }
    }
}

// TODO: export again as standalone types
#[derive(Default, Debug, PartialEq, Eq)]
pub enum DocumentType {
    /// Represents a document of any kind.
    #[default]
    Document,

    /// Represents an audio document of any kind.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audio>
    Audio,

    /// An image document of any kind
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image>
    Image,

    /// Represents a Web Page.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page>
    Page,

    /// Represents a video document of any kind.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-video>
    Video,
}
