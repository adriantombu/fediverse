use crate::types::properties::attachment::Attachment;
use crate::types::properties::attributed_to::AttributedTo;
use crate::types::properties::audience::Audience;
use crate::types::properties::bcc::Bcc;
use crate::types::properties::bto::Bto;
use crate::types::properties::cc::Cc;
use crate::types::properties::content::Content;
use crate::types::properties::context::Context;
use crate::types::properties::duration::Duration;
use crate::types::properties::end_time::EndTime;
use crate::types::properties::generator::Generator;
use crate::types::properties::icon::Icon;
use crate::types::properties::image::Image;
use crate::types::properties::in_reply_to::InReplyTo;
use crate::types::properties::location::Location;
use crate::types::properties::media_type::MediaType;
use crate::types::properties::name::Name;
use crate::types::properties::preview::Preview;
use crate::types::properties::published::Published;
use crate::types::properties::replies::Replies;
use crate::types::properties::start_time::StartTime;
use crate::types::properties::summary::Summary;
use crate::types::properties::tag::Tag;
use crate::types::properties::to::To;
use crate::types::properties::updated::Updated;
use crate::types::properties::url::Url;

/// Describes an object of any kind. The Object type serves as the base type for most of the other
/// kinds of objects defined in the Activity Vocabulary, including other Core types such as
/// [Activity](crate::types::core::activity::Activity), [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity),
/// [Collection](crate::types::core::collection::Collection) and [OrderedCollection](crate::types::core::ordered_collection::OrderedCollection).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object>
pub struct Object {
    pub r#type: ObjectType,

    pub attachment: Option<Attachment>,
    pub attributed_to: Option<AttributedTo>,
    pub audience: Option<Audience>,
    pub content: Option<Content>,
    pub context: Option<Context>,
    pub name: Option<Name>,
    pub end_time: Option<EndTime>,
    pub generator: Option<Generator>,
    pub icon: Option<Icon>,
    pub image: Option<Image>,
    pub in_reply_to: Option<InReplyTo>,
    pub location: Option<Location>,
    pub preview: Option<Preview>,
    pub published: Option<Published>,
    pub replies: Option<Replies>,
    pub start_time: Option<StartTime>,
    pub summary: Option<Summary>,
    pub tag: Option<Tag>,
    pub updated: Option<Updated>,
    pub url: Option<Url>,
    pub to: Option<To>,
    pub bto: Option<Bto>,
    pub cc: Option<Cc>,
    pub bcc: Option<Bcc>,
    pub media_type: Option<MediaType>,
    pub duration: Option<Duration>,
}

/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as [Activity](crate::types::core::activity::Activity),
/// [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity),
/// [Collection](crate::types::core::collection::Collection) and
/// [OrderedCollection](crate::types::core::ordered_collection::OrderedCollection).
pub enum ObjectType {
    /// Base type
    Object,

    /// Describes a software application.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application>
    Application,

    /// Represents any kind of multi-paragraph written work.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-article>
    Article,

    /// Represents any kind of event.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-event>
    Event,

    /// Represents a formal or informal collective of Actors.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group>
    Group,

    /// Represents a short written work typically less than a single paragraph in length.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image>
    Note,

    /// Represents an organization.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization>
    Organization,

    /// Represents an individual person.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person>
    Person,

    /// Represents a service of any kind.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service>
    Service,
}
