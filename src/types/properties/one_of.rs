use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies an exclusive option for a Question. Use of `oneOf` implies that the Question can have
/// only a single answer. To indicate that a Question can have multiple answers, use [anyOf](crate::types::properties::any_of::AnyOf).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-oneof>
#[derive(Debug, PartialEq)]
pub struct OneOf(Vec<Object>);

impl OneOf {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
