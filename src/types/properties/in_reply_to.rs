use crate::types::core::object::Object;
use crate::types::errors::TypeError;
use url::Url;

/// Indicates one or more entities for which this object is considered a response.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-inreplyto>
#[derive(Debug, PartialEq)]
pub struct InReplyTo(Vec<InReplyToType>);

impl InReplyTo {
    pub fn new(value: Vec<InReplyToType>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum InReplyToType {
    Url(Url),
    Object(Object),
}
