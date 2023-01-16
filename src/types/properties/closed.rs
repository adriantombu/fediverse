use crate::types::errors::TypeError;
use chrono::{DateTime, Utc};

/// Indicates that a question has been closed, and answers are no longer accepted.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-closed>
#[derive(Debug, PartialEq, Eq)]
pub struct Closed(ClosedType);

impl Closed {
    pub fn new(value: ClosedType) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClosedType {
    Datetime(DateTime<Utc>),
    Bool(bool),
}
