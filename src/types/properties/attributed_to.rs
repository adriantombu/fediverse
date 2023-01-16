use crate::types::core::object::Object;
use crate::types::errors::TypeError;
use url::Url;

/// Identifies one or more entities to which this object is attributed. The attributed entities
/// might not be Actors. For instance, an object might be attributed to the completion of another activity.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attributedto>
#[derive(Default, Debug, PartialEq)]
pub struct AttributedTo(Vec<AttributedToType>);

impl AttributedTo {
    pub fn new(value: Vec<AttributedToType>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum AttributedToType {
    Url(Url),
    Object(Object),
}
