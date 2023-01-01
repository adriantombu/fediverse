use crate::types::errors::TypeError;

/// A non-negative integer specifying the total number of objects contained by the logical view of
/// the collection. This number might not reflect the actual number of items serialized within the
/// [Collection](crate::types::core::collection::Collection) object instance.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-totalitems>
#[derive(Debug, PartialEq, Eq)]
pub struct TotalItems(usize);

impl TotalItems {
    pub fn new(value: usize) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let total_items = TotalItems::new(42);

        assert!(total_items.is_ok());
        assert_eq!(total_items.unwrap(), TotalItems(42));
    }
}
