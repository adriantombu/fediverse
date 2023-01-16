use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies a resource attached or related to an object that potentially requires special handling.
/// The intent is to provide a model that is at least semantically similar to attachments in email.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attachment>
#[derive(Default, Debug, PartialEq)]
pub struct Attachment(Vec<Object>);

impl Attachment {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
