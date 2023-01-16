use crate::types::errors::TypeError;
use crate::types::extended::place::Place;

/// Indicates one or more physical or logical locations associated with the object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-location>
#[derive(Debug, PartialEq)]
pub struct Location(Place);

impl Location {
    pub fn new(value: Place) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
