use crate::types::core::object::ObjectProperties;
use crate::types::properties::deleted::Deleted;
use crate::types::properties::former_type::FormerType;

/// A Tombstone represents a content object that has been deleted. It can be used in [Collections](crate::types::core::collection::Collection)
/// to signify that there used to be an object at this position, but it has been deleted.
#[derive(Default, Debug, PartialEq)]
pub struct Tombstone {
    pub object_properties: ObjectProperties,
    pub tombstone_properties: TombstoneProperties,
}

impl Tombstone {
    pub fn new(
        object_properties: ObjectProperties,
        tombstone_properties: TombstoneProperties,
    ) -> Self {
        Self {
            object_properties,
            tombstone_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct TombstoneProperties {
    pub former_type: Option<FormerType>,
    pub deleted: Option<Deleted>,
}
