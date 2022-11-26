use crate::types::errors::TypeError;

/// Indicates the accuracy of position coordinates on a Place objects.
/// Expressed in properties of percentage. e.g. "94.0" means "94.0% accurate".
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accuracy
#[derive(Debug, PartialEq)]
pub struct Accuracy(f32);

impl Accuracy {
    fn new(value: f32) -> Result<Self, TypeError> {
        if value < 0.0 {
            return Err(TypeError::OutOfBoundsMin {
                min: "0.0".to_string(),
                found: value.to_string(),
            });
        }

        if value > 100.0 {
            return Err(TypeError::OutOfBoundsMax {
                max: "100.0".to_string(),
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
        let acc = Accuracy::new(42.13);

        assert!(acc.is_ok());
        assert_eq!(acc.unwrap(), Accuracy(42.13));
    }

    #[test]
    fn test_new_out_of_bounds_min() {
        let acc = Accuracy::new(-31.19);

        assert!(acc.is_err());
        assert_eq!(
            acc.unwrap_err(),
            TypeError::OutOfBoundsMin {
                min: "0.0".to_string(),
                found: "-31.19".to_string()
            }
        );
    }

    #[test]
    fn test_new_out_of_bounds_max() {
        let acc = Accuracy::new(183.12);

        assert!(acc.is_err());
        assert_eq!(
            acc.unwrap_err(),
            TypeError::OutOfBoundsMax {
                max: "100.0".to_string(),
                found: "183.12".to_string()
            }
        );
    }
}
