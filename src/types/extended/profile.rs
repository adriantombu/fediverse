use crate::types::core::object::ObjectProperties;
use crate::types::properties::describes::Describes;

/// A Profile is a content object that describes another Object, typically used to describe
/// [Actor Type](https://www.w3.org/TR/activitystreams-vocabulary/#actor-types) objects.
///
/// The `describes` property is used to reference the object being described by the profile.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-profile>
#[derive(Default, Debug, PartialEq)]
pub struct Profile {
    pub object_properties: ObjectProperties,
    pub describes: Option<Describes>,
}

impl Profile {
    pub fn new(object_properties: ObjectProperties, describes: Option<Describes>) -> Self {
        Self {
            object_properties,
            describes,
        }
    }
}
