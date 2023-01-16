use crate::types::errors::TypeError;
use chrono::{DateTime, Utc};

/// On a [Tombstone](crate::types::extended::tombstone::Tombstone) object, the `deleted` property
/// is a timestamp for when the object was deleted.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-deleted>
#[derive(Debug, PartialEq, Eq)]
pub struct Deleted(DateTime<Utc>);

impl Deleted {
    pub fn new(value: DateTime<Utc>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
