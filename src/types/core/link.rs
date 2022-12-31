use crate::types::properties::height::Height;
use crate::types::properties::href::Href;
use crate::types::properties::hreflang::Hreflang;
use crate::types::properties::media_type::MediaType;
use crate::types::properties::name::Name;
use crate::types::properties::preview::Preview;
use crate::types::properties::rel::Rel;
use crate::types::properties::width::Width;

/// A `Link` is an indirect, qualified reference to a resource identified by a URL. The fundamental
/// model for links is established by [RFC5988](https://www.w3.org/TR/activitystreams-vocabulary/#bib-RFC5988).
/// Many of the properties defined by the Activity Vocabulary allow values that are either instances
/// of [Object](crate::types::core::object) or `Link`. When a `Link` is used, it establishes a qualified
/// relation connecting the subject (the containing object) to the resource identified by the [href](crate::types::properties::href).
/// Properties of the `Link` are properties of the reference as opposed to properties of the resource.
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-link>
pub struct Link {
    pub href: Href,
    pub rel: Option<Rel>,
    pub media_type: Option<MediaType>,
    pub name: Option<Name>,
    pub hreflang: Option<Hreflang>,
    pub height: Option<Height>,
    pub width: Option<Width>,
    pub preview: Option<Preview>,
}
