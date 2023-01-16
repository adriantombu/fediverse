use crate::types::core::object::ObjectProperties;
use crate::types::properties::current::Current;
use crate::types::properties::first::First;
use crate::types::properties::items::Items;
use crate::types::properties::last::Last;
use crate::types::properties::total_items::TotalItems;

/// A `Collection` is a subtype of [Object](crate::types::core::object::Object) that represents ordered or
/// unordered sets of [Object](crate::types::core::object::Object) or [Link](crate::types::core::link::Link) instances.
/// Refer to the [Activity Streams 2.0 Core specification](https://www.w3.org/TR/activitystreams-core/#collection)
/// for a complete description of the `Collection` type.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection>
#[derive(Default, Debug, PartialEq)]
pub struct Collection {
    pub object_properties: ObjectProperties,
    pub collection_properties: CollectionProperties,
}

impl Collection {
    pub fn new(
        object_properties: ObjectProperties,
        collection_properties: CollectionProperties,
    ) -> Self {
        Self {
            object_properties,
            collection_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct CollectionProperties {
    pub total_items: Option<TotalItems>,
    pub current: Option<Current>,
    pub first: Option<First>,
    pub last: Option<Last>,
    pub items: Option<Items>,
}
