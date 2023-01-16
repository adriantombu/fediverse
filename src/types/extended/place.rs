use crate::types::core::object::ObjectProperties;
use crate::types::properties::accuracy::Accuracy;
use crate::types::properties::altitude::Altitude;
use crate::types::properties::latitude::Latitude;
use crate::types::properties::longitude::Longitude;
use crate::types::properties::radius::Radius;
use crate::types::properties::units::Units;

/// Represents a logical or physical location. See [5.3 Representing Places for additional information](https://www.w3.org/TR/activitystreams-vocabulary/#places).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page>
#[derive(Default, Debug, PartialEq)]
pub struct Place {
    pub object_properties: ObjectProperties,
    pub place_properties: PlaceProperties,
}

impl Place {
    pub fn new(object_properties: ObjectProperties, place_properties: PlaceProperties) -> Self {
        Self {
            object_properties,
            place_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct PlaceProperties {
    pub accuracy: Option<Box<Accuracy>>,
    pub altitude: Option<Box<Altitude>>,
    pub latitude: Option<Box<Latitude>>,
    pub longitude: Option<Box<Longitude>>,
    pub radius: Option<Box<Radius>>,
    pub units: Option<Box<Units>>,
}
