use crate::types::properties::actor::Actor;
use crate::types::properties::instrument::Instrument;
use crate::types::properties::object::Object;
use crate::types::properties::origin::Origin;
use crate::types::properties::result::Result;
use crate::types::properties::target::Target;

/// Indicates that the `actor` is traveling to `target` from `origin`.
///
/// Travel is an [IntransitiveObject](crate::types::core::intransitive_activity::IntransitiveActivity)
/// whose actor specifies the direct object.
///
/// If the `target` or `origin` are not specified, either can be determined by context.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel>
pub struct Travel {
    // Properties from Activity
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}
