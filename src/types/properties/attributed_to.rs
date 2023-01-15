use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies one or more entities to which this object is attributed. The attributed entities
/// might not be Actors. For instance, an object might be attributed to the completion of another activity.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attributedto>
#[derive(Debug, PartialEq, Eq)]
pub struct AttributedTo(Vec<Object>);

impl AttributedTo {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
