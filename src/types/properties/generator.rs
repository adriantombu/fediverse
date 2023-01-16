// TODO: to implement (need Object & Link)

use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies the entity (e.g. an application) that generated the object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-generator>
#[derive(Debug, PartialEq)]
pub struct Generator(Object);

impl Generator {
    pub fn new(value: Object) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
