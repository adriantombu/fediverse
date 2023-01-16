use crate::types::core::collection::CollectionProperties;
use crate::types::core::object::ObjectProperties;

///A subtype of [Collection](crate::types::core::collection::Collection) in which members of the logical
/// collection are assumed to always be strictly ordered.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection>
#[derive(Debug, PartialEq)]
pub struct OrderedCollection {
    pub object_properties: ObjectProperties,
    pub collection_properties: CollectionProperties,
}
