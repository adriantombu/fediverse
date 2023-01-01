use crate::types::errors::TypeError;

/// The longitude of a [Place](crate::types::extended::object::place::Place).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-longitude>
#[derive(Debug, PartialEq)]
pub struct Longitude(f32);

impl Longitude {
    pub const MIN_BOUND: f32 = -180.0;
    pub const MAX_BOUND: f32 = 180.0;

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
        let longitude = Longitude::new(42.13);

        assert!(longitude.is_ok());
        assert_eq!(longitude.unwrap(), Longitude(42.13));
    }

    #[test]
    fn test_new_out_of_bounds_min() {
        let longitude = Longitude::new(-192.9);

        assert!(longitude.is_err());
        assert_eq!(
            longitude.unwrap_err(),
            TypeError::OutOfBoundsMin {
                min: Longitude::MIN_BOUND.to_string(),
                found: "-192.9".to_string()
            }
        );
    }

    #[test]
    fn test_new_out_of_bounds_max() {
        let longitude = Longitude::new(183.12);

        assert!(longitude.is_err());
        assert_eq!(
            longitude.unwrap_err(),
            TypeError::OutOfBoundsMax {
                max: Longitude::MAX_BOUND.to_string(),
                found: "183.12".to_string()
            }
        );
    }
}
