use crate::types::core::link::Link;
use crate::types::errors::TypeError;
use crate::types::extended::document::Document;

/// Indicates an entity that describes an image for this object. Unlike the icon property, there are
/// no aspect ratio or display size limitations assumed.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image>
#[derive(Debug, PartialEq)]
pub struct Image(Vec<ImageType>);

impl Image {
    pub fn new(value: Vec<ImageType>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum ImageType {
    Image(Document), // TODO: Document of type DocumentType::Image?
    Link(Box<Link>),
}
