use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// On a [Profile](crate::types::extended::profile::Profile) object, the `describes` property
/// identifies the object described by the [Profile](crate::types::extended::profile::Profile).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-describes>
#[derive(Default, Debug, PartialEq)]
pub struct Describes(Object);

impl Describes {
    pub fn new(value: Object) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
