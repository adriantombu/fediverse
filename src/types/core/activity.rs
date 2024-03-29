use crate::types::core::object::ObjectProperties;
use crate::types::properties::actor::Actor;
use crate::types::properties::instrument::Instrument;
use crate::types::properties::object::Object;
use crate::types::properties::origin::Origin;
use crate::types::properties::result::Result;
use crate::types::properties::target::Target;

/// An `Activity` is a subtype of [Object](crate::types::core::object::Object) that describes some form of
/// action that may happen, is currently happening, or has already happened. The `Activity` type
/// itself serves as an abstract base type for all types of activities.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-activity>
#[derive(Default, Debug, PartialEq)]
pub struct Activity {
    r#type: String,
    pub object_properties: ObjectProperties,
    pub activity_properties: ActivityProperties,
}

impl Activity {
    pub fn new(
        object_properties: ObjectProperties,
        activity_properties: ActivityProperties,
    ) -> Self {
        Self {
            r#type: "Activity".to_string(),
            object_properties,
            activity_properties,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ActivityProperties {
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub target: Option<Target>,
    pub result: Option<Result>,
    pub origin: Option<Origin>,
    pub instrument: Option<Instrument>,
}

// TODO: export again as standalone types
/// The Activity type itself serves as an abstract base type for all types of activities.
#[derive(Debug, PartialEq, Eq)]
pub enum ActivityType {
    /// It is important to note that the `Activity` type itself does not carry any specific
    /// semantics about the kind of action being taken.
    Activity,

    /// Indicates that the `actor` accepts the `object`.
    ///
    /// The `target` property can be used in certain circumstances to indicate the context into which
    /// the `object` has been accepted.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept>
    Accept,

    /// Indicates that the `actor` has added the `object` to the `target`.
    ///
    /// If the `target` property is not explicitly specified, the target would need to be determined
    /// implicitly by context. The `origin` can be used to identify the context from which the `object`
    /// originated.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add>
    Add,

    /// Indicates that the `actor` is calling the `target`'s attention the `object`.
    ///
    /// The `origin` typically has no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce>
    Announce,

    /// Indicates that the `actor` is blocking the `object`. Blocking is a stronger form of [Ignore](crate::types::core::activity::ActivityType::Ignore).
    ///
    /// The typical use is to support social systems that allow one user to block activities or content
    /// of other users. The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block>
    Block,

    /// Indicates that the `actor` has created the `object`.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create>
    Create,

    /// Indicates that the `actor` has deleted the `object`.
    ///
    /// If specified, the `origin` indicates the context from which the `object` was deleted.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete>
    Delete,

    /// Indicates that the `actor` dislikes the `object`.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike>
    Dislike,

    /// Indicates that the `actor` is "flagging" the `object`.
    ///
    /// Flagging is defined in the sense common to many social platforms as reporting content as being
    /// inappropriate for any number of reasons.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag>
    Flag,

    /// Indicates that the `actor` is "following" the `object`.
    ///
    /// Following is defined in the sense typically used within Social systems in which the actor is
    /// interested in any activity performed by or on the object. The `target` and `origin` typically
    /// have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow>
    Follow,

    /// Indicates that the `actor` is ignoring the `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore>
    Ignore,

    /// A specialization of [Offer](crate::types::core::activity::ActivityType::Offer) in which the `actor`
    /// is extending an invitation for the `object` to the `target`.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite>
    Invite,

    /// Indicates that the `actor` has joined the `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join>
    Join,

    /// Indicates that the `actor` has left the `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave>
    Leave,

    /// Indicates that the `actor` likes, recommends or endorses the `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like>
    Like,

    /// Indicates that the `actor` has listened to the `object`.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen>
    Listen,

    /// Indicates that the `actor` has moved `object` from `origin` to `target`.
    ///
    /// If the `origin` or `target` are not specified, either can be determined by context.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move>
    Move,

    /// Indicates that the `actor` is offering the `object`.
    ///
    /// If specified, the `target` indicates the entity to which the `object` is being offered.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer>
    Offer,

    /// Indicates that the `actor` has updated the `object`.
    ///
    /// Note, however, that this vocabulary does not define a mechanism for describing the actual set
    /// of modifications made to `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read>
    Read,

    /// Indicates that the `actor` is rejecting the `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject>
    Reject,

    /// Indicates that the `actor` is removing the `object`.
    ///
    /// If specified, the `origin` indicates the context from which the `object` is being removed.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove>
    Remove,

    /// A specialization of [Accept](crate::types::core::activity::ActivityType::Accept) in which the
    /// acceptance is tentative.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept>
    TentativeAccept,

    /// A specialization of [Reject](crate::types::core::activity::ActivityType::Reject) in which the
    /// rejection is considered tentative.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject>
    TentativeReject,

    /// Indicates that the `actor` is undoing the `object`.
    ///
    /// In most cases, the `object` will be an [Activity](crate::types::core::activity::Activity)
    /// describing some previously performed action (for instance, a person may have previously "liked"
    /// an article but, for whatever reason, might choose to undo that like at some later point in time).
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo>
    Undo,

    /// Indicates that the `actor` has updated the `object`.
    ///
    /// Note, however, that this vocabulary does not define a mechanism for describing the actual set
    /// of modifications made to `object`.
    ///
    /// The `target` and `origin` typically have no defined meaning.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update>
    Update,

    /// Indicates that the `actor` has viewed the `object`.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view>
    View,
}
