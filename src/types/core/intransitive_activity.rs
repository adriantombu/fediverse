use crate::types::properties::actor::Actor;
use crate::types::properties::instrument::Instrument;
use crate::types::properties::object::Object;
use crate::types::properties::origin::Origin;
use crate::types::properties::result::Result;
use crate::types::properties::target::Target;

/// Instances of `IntransitiveActivity are a subtype of [Activity](crate::types::core::activity::Activity)
/// representing intransitive actions. The object property is therefore inappropriate for these
/// activities.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-intransitiveactivity>
pub struct IntransitiveActivity {
    pub r#type: IntransitiveActivityType,
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntransitiveActivityType {
    /// An [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity) that
    /// indicates that the `actor` has arrived at the `location`
    ///
    /// The `origin` can be used to identify the context from which the `actor` originated. The `target`
    /// typically has no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive>
    Arrive,

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
    Question,

    /// Indicates that the `actor` is traveling to `target` from `origin`.
    ///
    /// Travel is an [IntransitiveObject](crate::types::core::intransitive_activity::IntransitiveActivity)
    /// whose actor specifies the direct object.
    ///
    /// If the `target` or `origin` are not specified, either can be determined by context.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel>
    Travel,
}
