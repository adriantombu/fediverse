use crate::types::errors::TypeError;

pub const ACCURACY_MIN_BOUND: f32 = 0.0;
pub const ACCURACY_MAX_BOUND: f32 = 100.0;

/// Indicates the accuracy of position coordinates on a Place objects.
/// Expressed in properties of percentage. e.g. "94.0" means "94.0% accurate".
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accuracy
#[derive(Debug, PartialEq)]
pub struct Accuracy(f32);

impl Accuracy {
    fn new(value: f32) -> Result<Self, TypeError> {
        if value < ACCURACY_MIN_BOUND {
            return Err(TypeError::OutOfBoundsMin {
                min: ACCURACY_MIN_BOUND.to_string(),
                found: value.to_string(),
            });
        }

        if value > ACCURACY_MAX_BOUND {
            return Err(TypeError::OutOfBoundsMax {
                max: ACCURACY_MAX_BOUND.to_string(),
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
        let accuracy = Accuracy::new(42.13);

        assert!(accuracy.is_ok());
        assert_eq!(accuracy.unwrap(), Accuracy(42.13));
    }

    #[test]
    fn test_new_out_of_bounds_min() {
        let accuracy = Accuracy::new(-31.19);

        assert!(accuracy.is_err());
        assert_eq!(
            accuracy.unwrap_err(),
            TypeError::OutOfBoundsMin {
                min: ACCURACY_MIN_BOUND.to_string(),
                found: "-31.19".to_string()
            }
        );
    }

    #[test]
    fn test_new_out_of_bounds_max() {
        let accuracy = Accuracy::new(183.12);

        assert!(accuracy.is_err());
        assert_eq!(
            accuracy.unwrap_err(),
            TypeError::OutOfBoundsMax {
                max: ACCURACY_MAX_BOUND.to_string(),
                found: "183.12".to_string()
            }
        );
    }
}
