use crate::types::core::object::Object;
use crate::types::errors::TypeError;

/// Identifies the items contained in a collection. The items might be ordered or unordered.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-items>
#[derive(Debug, PartialEq)]
pub struct OrderedItems(Vec<Object>);

impl OrderedItems {
    pub fn new(value: Vec<Object>) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}
