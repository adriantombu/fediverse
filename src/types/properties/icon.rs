use crate::types::core::link::Link;
use crate::types::errors::TypeError;
use crate::types::extended::document::Document;

/// Indicates an entity that describes an icon for this object. The image should havean aspect ratio
/// of one (horizontal) to one (vertical) and should be suitable for presentation at a small size.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-icon>
#[derive(Debug, PartialEq)]
pub struct Icon(Vec<IconType>);

impl Icon {
    pub fn new(value: Vec<IconType>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum IconType {
    Image(Document), // TODO: Document of type DocumentType::Image?
    Link(Box<Link>),
}
