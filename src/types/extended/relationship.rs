use crate::types::core::object::ObjectProperties;
use crate::types::properties::object::Object;
use crate::types::properties::relationship::Relationship as RelationshipProperty;
use crate::types::properties::subject::Subject;

/// Describes a relationship between two individuals.
///
/// The `subject` and `object properties are used to identify the connected individuals.
///
/// See [5.2 Representing Relationships Between Entities](https://www.w3.org/TR/activitystreams-vocabulary/#connections)
/// for additional information.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship>
#[derive(Default, Debug, PartialEq)]
pub struct Relationship {
    pub object_properties: ObjectProperties,
    pub relationship_properties: RelationshipProperties,
}

impl Relationship {
    pub fn new(
        object_properties: ObjectProperties,
        relationship_properties: RelationshipProperties,
    ) -> Self {
        Self {
            object_properties,
            relationship_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct RelationshipProperties {
    pub subject: Option<Subject>,
    pub object: Option<Object>,
    pub relationship: Option<RelationshipProperty>,
}
