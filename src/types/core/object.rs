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
#[derive(Default, Debug, PartialEq)]
pub struct Object {
    pub r#type: String,
    pub object_properties: ObjectProperties,
}

impl Object {
    pub fn new() -> Self {
        Self {
            r#type: "Object".to_string(),
            ..Object::default()
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ObjectProperties {
    pub attachment: Option<Box<Attachment>>,
    pub attributed_to: Option<Box<AttributedTo>>,
    pub audience: Option<Box<Audience>>,
    pub content: Option<Box<Content>>,
    pub context: Option<Box<Context>>,
    pub name: Option<Box<Name>>,
    pub end_time: Option<Box<EndTime>>,
    pub generator: Option<Box<Generator>>,
    pub icon: Option<Box<Icon>>,
    pub image: Option<Box<Image>>,
    pub in_reply_to: Option<Box<InReplyTo>>,
    pub location: Option<Box<Location>>,
    pub preview: Option<Box<Preview>>,
    pub published: Option<Box<Published>>,
    pub replies: Option<Box<Replies>>,
    pub start_time: Option<Box<StartTime>>,
    pub summary: Option<Box<Summary>>,
    pub tag: Option<Box<Tag>>,
    pub updated: Option<Box<Updated>>,
    pub url: Option<Box<Url>>,
    pub to: Option<Box<To>>,
    pub bto: Option<Box<Bto>>,
    pub cc: Option<Box<Cc>>,
    pub bcc: Option<Box<Bcc>>,
    pub media_type: Option<Box<MediaType>>,
    pub duration: Option<Box<Duration>>,
}

// TODO: export again as standalone types
/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as [Activity](crate::types::core::activity::Activity),
/// [IntransitiveActivity](crate::types::core::intransitive_activity::IntransitiveActivity),
/// [Collection](crate::types::core::collection::Collection) and
/// [OrderedCollection](crate::types::core::ordered_collection::OrderedCollection).
#[derive(Default, Debug, PartialEq, Eq)]
pub enum ObjectType {
    /// Base type
    #[default]
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
