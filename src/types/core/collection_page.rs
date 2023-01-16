use crate::types::core::collection::CollectionProperties;
use crate::types::core::object::ObjectProperties;
use crate::types::properties::next::Next;
use crate::types::properties::part_of::PartOf;
use crate::types::properties::prev::Prev;

/// Used to represent distinct subsets of items from a [Collection](crate::types::core::collection::Collection).
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-collectionpage)
///for a complete description of the `CollectionPage object`.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collectionpage>
#[derive(Default, Debug, PartialEq)]
pub struct CollectionPage {
    pub object_properties: ObjectProperties,
    pub collection_properties: CollectionProperties,
    pub collection_page_properties: CollectionPageProperties,
}

impl CollectionPage {
    pub fn new(
        object_properties: ObjectProperties,
        collection_properties: CollectionProperties,
        collection_page_properties: CollectionPageProperties,
    ) -> Self {
        Self {
            object_properties,
            collection_properties,
            collection_page_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct CollectionPageProperties {
    pub part_of: Option<PartOf>,
    pub next: Option<Next>,
    pub prev: Option<Prev>,
}
