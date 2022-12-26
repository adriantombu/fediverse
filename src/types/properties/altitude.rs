use crate::types::errors::TypeError;

/// Indicates the altitude of a place. The measurement units is indicated using the units property. If units is not specified, the default is assumed to be "m" indicating meters.
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-altitude
#[derive(Debug, PartialEq)]
pub struct Altitude(f32);

impl Altitude {
    pub fn new(value: f32) -> Result<Self, TypeError> {
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let altitude = Altitude::new(42.13);

        assert!(altitude.is_ok());
        assert_eq!(altitude.unwrap(), Altitude(42.13));
    }
}
