use crate::types::errors::TypeError;

pub const LONGITUDE_MIN_BOUND: f32 = -180.0;
pub const LONGITUDE_MAX_BOUND: f32 = 180.0;

/// The longitude of a place
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-longitude
#[derive(Debug, PartialEq)]
pub struct Longitude(f32);

impl Longitude {
    fn new(value: f32) -> Result<Self, TypeError> {
        if value < LONGITUDE_MIN_BOUND {
            return Err(TypeError::OutOfBoundsMin {
                min: LONGITUDE_MIN_BOUND.to_string(),
                found: value.to_string(),
            });
        }

        if value > LONGITUDE_MAX_BOUND {
            return Err(TypeError::OutOfBoundsMax {
                max: LONGITUDE_MAX_BOUND.to_string(),
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
                min: LONGITUDE_MIN_BOUND.to_string(),
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
                max: LONGITUDE_MAX_BOUND.to_string(),
                found: "183.12".to_string()
            }
        );
    }
}
