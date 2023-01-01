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
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}
