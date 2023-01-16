use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies an inclusive option for a Question. Use of `anyOf` implies that the Question can have
/// multiple answers. To indicate that a Question can have only one answer, use [oneOf](crate::types::properties::one_of::OneOf).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-anyof>
#[derive(Default, Debug, PartialEq)]
pub struct AnyOf(Vec<Object>);

impl AnyOf {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
