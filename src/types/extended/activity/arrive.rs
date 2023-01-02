use crate::types::properties::actor::Actor;
use crate::types::properties::instrument::Instrument;
use crate::types::properties::object::Object;
use crate::types::properties::origin::Origin;
use crate::types::properties::result::Result;
use crate::types::properties::target::Target;

/// An [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity) that
/// indicates that the `actor` has arrived at the `location`
///
/// The `origin` can be used to identify the context from which the `actor` originated. The `target`
/// typically has no defined meaning.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive>
pub struct Arrive {
    // Properties from Activity
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}
