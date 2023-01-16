use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies one or more objects used (or to be used) in the completion of an [Activity](crate::types::core::activity::Activity).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-instrument>
#[derive(Debug, PartialEq)]
pub struct Instrument(Vec<Object>);

impl Instrument {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
