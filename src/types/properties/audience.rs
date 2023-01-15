use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies one or more entities that represent the total population of entities for which the
/// object can considered to be relevant.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audience>
#[derive(Debug, PartialEq, Eq)]
pub struct Audience(Vec<Object>);

impl Audience {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
