use crate::types::properties::actor::Actor;
use crate::types::properties::instrument::Instrument;
use crate::types::properties::object::Object;
use crate::types::properties::origin::Origin;
use crate::types::properties::result::Result;
use crate::types::properties::target::Target;

/// Represents a question being asked.
///
/// Question objects are an extension of [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity).
/// That is, the Question object is an Activity, but the direct object is the question itself and
/// therefore it would not contain an [object](crate::types::properties::object::Object) property.
///
/// Either of the [anyOf](crate::types::properties::any_of::AnyOf) and [oneOf](crate::types::properties::one_of::OneOf)
/// properties MAY be used to express possible answers, but a Question object MUST NOT have both properties.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question>
pub struct Question {
    // Properties from Activity
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}
