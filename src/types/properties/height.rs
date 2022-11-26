use crate::types::errors::TypeError;

/// On a Link, specifies a hint as to the rendering height in device-independent pixels of the linked resource.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-height
#[derive(Debug, PartialEq, Eq)]
pub struct Height(usize);

impl Height {
    fn new(value: usize) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let acc = Height::new(42);

        assert!(acc.is_ok());
        assert_eq!(acc.unwrap(), Height(42));
    }
}
