use crate::types::errors::TypeError;

/// The latitude of a place
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-latitude
#[derive(Debug, PartialEq)]
pub struct Latitude(f32);

impl Latitude {
    pub const MIN_BOUND: f32 = -90.0;
    pub const MAX_BOUND: f32 = 90.0;

    pub fn new(value: f32) -> Result<Self, TypeError> {
        if value < Self::MIN_BOUND {
            return Err(TypeError::OutOfBoundsMin {
                min: Self::MIN_BOUND.to_string(),
                found: value.to_string(),
            });
        }

        if value > Self::MAX_BOUND {
            return Err(TypeError::OutOfBoundsMax {
                max: Self::MAX_BOUND.to_string(),
                found: value.to_string(),
            });
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let latitude = Latitude::new(42.13);

        assert!(latitude.is_ok());
        assert_eq!(latitude.unwrap(), Latitude(42.13));
    }

    #[test]
    fn test_new_out_of_bounds_min() {
        let latitude = Latitude::new(-102.9);

        assert!(latitude.is_err());
        assert_eq!(
            latitude.unwrap_err(),
            TypeError::OutOfBoundsMin {
                min: Latitude::MIN_BOUND.to_string(),
                found: "-102.9".to_string()
            }
        );
    }

    #[test]
    fn test_new_out_of_bounds_max() {
        let latitude = Latitude::new(183.12);

        assert!(latitude.is_err());
        assert_eq!(
            latitude.unwrap_err(),
            TypeError::OutOfBoundsMax {
                max: Latitude::MAX_BOUND.to_string(),
                found: "183.12".to_string()
            }
        );
    }
}
