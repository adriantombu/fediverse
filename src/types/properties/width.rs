use crate::types::errors::TypeError;

/// On a [Link](crate::types::core::link::Link), specifies a hint as to the rendering width in
/// device-independent pixels of the linked resource.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-width>
#[derive(Debug, PartialEq, Eq)]
pub struct Width(usize);

impl Width {
    pub fn new(value: usize) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let width = Width::new(42);

        assert!(width.is_ok());
        assert_eq!(width.unwrap(), Width(42));
    }
}
