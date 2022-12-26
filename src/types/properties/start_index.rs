use crate::types::errors::TypeError;

/// A non-negative integer value identifying the relative position within the logical view of a strictly ordered collection.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-startindex
#[derive(Debug, PartialEq, Eq)]
pub struct StartIndex(usize);

impl StartIndex {
    pub fn new(value: usize) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let start_index = StartIndex::new(42);

        assert!(start_index.is_ok());
        assert_eq!(start_index.unwrap(), StartIndex(42));
    }
}
