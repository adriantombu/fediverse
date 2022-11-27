use crate::types::errors::TypeError;

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    Cm,
    Feet,
    Inches,
    Km,
    M,
    Miles,
}

/// Specifies the measurement units for the radius and altitude properties on a Place object. If not specified, the default is assumed to be "m" for "meters".
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-totalitems
#[derive(Debug, PartialEq, Eq)]
pub struct Units(Unit);

impl Units {
    fn new(value: Option<Unit>) -> Result<Self, TypeError> {
        Ok(Self(value.unwrap_or(Unit::M)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let total_items = Units::new(Some(Unit::Km));

        assert!(total_items.is_ok());
        assert_eq!(total_items.unwrap(), Units(Unit::Km));
    }

    #[test]
    fn test_new_default() {
        let total_items = Units::new(None);

        assert!(total_items.is_ok());
        assert_eq!(total_items.unwrap(), Units(Unit::M));
    }
}
