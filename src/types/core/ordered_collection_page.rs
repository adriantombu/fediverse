use crate::types::core::collection::CollectionProperties;
use crate::types::core::collection_page::CollectionPageProperties;
use crate::types::core::object::ObjectProperties;
use crate::types::properties::start_index::StartIndex;

/// Used to represent ordered subsets of items from an [OrderedCollection](crate::types::core::ordered_collection::OrderedCollection).
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-orderedcollectionpage)
/// for a complete description of the `OrderedCollectionPage` object.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollectionpage>
#[derive(Debug, PartialEq)]
pub struct OrderedCollectionPage {
    pub object_properties: ObjectProperties,
    pub collection_properties: CollectionProperties,
    pub collection_page_properties: CollectionPageProperties,

    pub start_index: Option<StartIndex>,
}
