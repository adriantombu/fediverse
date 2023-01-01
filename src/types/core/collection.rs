use crate::types::properties::attachment::Attachment;
use crate::types::properties::attributed_to::AttributedTo;
use crate::types::properties::audience::Audience;
use crate::types::properties::bcc::Bcc;
use crate::types::properties::bto::Bto;
use crate::types::properties::cc::Cc;
use crate::types::properties::content::Content;
use crate::types::properties::context::Context;
use crate::types::properties::current::Current;
use crate::types::properties::duration::Duration;
use crate::types::properties::end_time::EndTime;
use crate::types::properties::first::First;
use crate::types::properties::generator::Generator;
use crate::types::properties::icon::Icon;
use crate::types::properties::image::Image;
use crate::types::properties::in_reply_to::InReplyTo;
use crate::types::properties::items::Items;
use crate::types::properties::last::Last;
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
use crate::types::properties::total_items::TotalItems;
use crate::types::properties::updated::Updated;
use crate::types::properties::url::Url;

/// A `Collection` is a subtype of [Object](crate::types::core::object::Object) that represents ordered or
/// unordered sets of [Object](crate::types::core::object::Object) or [Link](crate::types::core::link::Link) instances.
/// Refer to the [Activity Streams 2.0 Core specification](https://www.w3.org/TR/activitystreams-core/#collection)
/// for a complete description of the `Collection` type.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection>
pub struct Collection {
    // Properties from Object
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

    pub total_items: Option<TotalItems>,
    pub current: Option<Current>,
    pub first: Option<First>,
    pub last: Option<Last>,
    pub items: Option<Items>,
}
