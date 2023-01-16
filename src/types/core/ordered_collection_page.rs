use crate::types::core::collection::CollectionProperties;
use crate::types::core::collection_page::CollectionPageProperties;
use crate::types::core::object::ObjectProperties;
use crate::types::properties::start_index::StartIndex;

/// Used to represent ordered subsets of items from an [OrderedCollection](crate::types::core::ordered_collection::OrderedCollection).
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-orderedcollectionpage)
/// for a complete description of the `OrderedCollectionPage` object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollectionpage>
#[derive(Default, Debug, PartialEq)]
pub struct OrderedCollectionPage {
    pub object_properties: ObjectProperties,
    pub collection_properties: CollectionProperties,
    pub collection_page_properties: CollectionPageProperties,

    pub start_index: Option<StartIndex>,
}

impl OrderedCollectionPage {
    pub fn new(
        object_properties: ObjectProperties,
        collection_properties: CollectionProperties,
        collection_page_properties: CollectionPageProperties,
        start_index: Option<StartIndex>,
    ) -> Self {
        Self {
            object_properties,
            collection_properties,
            collection_page_properties,
            start_index,
        }
    }
}
