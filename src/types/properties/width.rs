use crate::types::errors::TypeError;

/// On a Link, specifies a hint as to the rendering width in device-independent pixels of the linked resource.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-width
#[derive(Debug, PartialEq, Eq)]
pub struct Width(usize);

impl Width {
    fn new(value: usize) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let acc = Width::new(42);

        assert!(acc.is_ok());
        assert_eq!(acc.unwrap(), Width(42));
    }
}
