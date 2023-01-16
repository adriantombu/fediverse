use crate::types::properties::accuracy::Accuracy;
use crate::types::properties::altitude::Altitude;
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
use crate::types::properties::latitude::Latitude;
use crate::types::properties::location::Location;
use crate::types::properties::longitude::Longitude;
use crate::types::properties::media_type::MediaType;
use crate::types::properties::name::Name;
use crate::types::properties::preview::Preview;
use crate::types::properties::published::Published;
use crate::types::properties::radius::Radius;
use crate::types::properties::replies::Replies;
use crate::types::properties::start_time::StartTime;
use crate::types::properties::summary::Summary;
use crate::types::properties::tag::Tag;
use crate::types::properties::to::To;
use crate::types::properties::units::Units;
use crate::types::properties::updated::Updated;
use crate::types::properties::url::Url;

/// Represents a logical or physical location. See [5.3 Representing Places for additional information](https://www.w3.org/TR/activitystreams-vocabulary/#places).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page>
#[derive(Debug, PartialEq)]
pub struct Place {
    // Properties from Object
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

    pub accuracy: Option<Box<Accuracy>>,
    pub altitude: Option<Box<Altitude>>,
    pub latitude: Option<Box<Latitude>>,
    pub longitude: Option<Box<Longitude>>,
    pub radius: Option<Box<Radius>>,
    pub units: Option<Box<Units>>,
}
