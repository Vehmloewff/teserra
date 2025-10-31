use crate::{Element, element::ElementAttributor, tag::*};

// ============================================================================
// Global Attributes (can be used on ANY HTML element)
// ============================================================================

pub trait GlobalAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accesskey(self, value: &str) -> Self {
        self.attr("accesskey", value)
    }

    fn autocapitalize(self, value: &str) -> Self {
        self.attr("autocapitalize", value)
    }

    fn autofocus(self, value: &str) -> Self {
        self.attr("autofocus", value)
    }

    fn contenteditable(self, value: &str) -> Self {
        self.attr("contenteditable", value)
    }

    fn dir(self, value: &str) -> Self {
        self.attr("dir", value)
    }

    fn draggable(self, value: &str) -> Self {
        self.attr("draggable", value)
    }

    fn enterkeyhint(self, value: &str) -> Self {
        self.attr("enterkeyhint", value)
    }

    fn exportparts(self, value: &str) -> Self {
        self.attr("exportparts", value)
    }

    fn hidden(self, value: &str) -> Self {
        self.attr("hidden", value)
    }

    fn id(self, value: &str) -> Self {
        self.attr("id", value)
    }

    fn inert(self, value: &str) -> Self {
        self.attr("inert", value)
    }

    fn inputmode(self, value: &str) -> Self {
        self.attr("inputmode", value)
    }

    fn is(self, value: &str) -> Self {
        self.attr("is", value)
    }

    // Microdata attributes
    fn itemid(self, value: &str) -> Self {
        self.attr("itemid", value)
    }

    fn itemprop(self, value: &str) -> Self {
        self.attr("itemprop", value)
    }

    fn itemref(self, value: &str) -> Self {
        self.attr("itemref", value)
    }

    fn itemscope(self, value: &str) -> Self {
        self.attr("itemscope", value)
    }

    fn itemtype(self, value: &str) -> Self {
        self.attr("itemtype", value)
    }

    fn lang(self, value: &str) -> Self {
        self.attr("lang", value)
    }

    fn nonce(self, value: &str) -> Self {
        self.attr("nonce", value)
    }

    fn part(self, value: &str) -> Self {
        self.attr("part", value)
    }

    fn popover(self, value: &str) -> Self {
        self.attr("popover", value)
    }

    fn role(self, value: &str) -> Self {
        self.attr("role", value)
    }

    fn slot(self, value: &str) -> Self {
        self.attr("slot", value)
    }

    fn spellcheck(self, value: &str) -> Self {
        self.attr("spellcheck", value)
    }

    fn style(self, value: &str) -> Self {
        self.attr("style", value)
    }

    fn tabindex(self, value: &str) -> Self {
        self.attr("tabindex", value)
    }

    fn title(self, value: &str) -> Self {
        self.attr("title", value)
    }

    fn translate(self, value: &str) -> Self {
        self.attr("translate", value)
    }

    fn virtualkeyboardpolicy(self, value: &str) -> Self {
        self.attr("virtualkeyboardpolicy", value)
    }

    fn writingsuggestions(self, value: &str) -> Self {
        self.attr("writingsuggestions", value)
    }
}

impl<Tag> GlobalAttributes for Element<'_, Tag> {}

// ============================================================================
// Form Element Attributes
// ============================================================================

pub trait FormElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accept_charset(self, value: &str) -> Self {
        self.attr("accept-charset", value)
    }

    fn action(self, value: &str) -> Self {
        self.attr("action", value)
    }

    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn enctype(self, value: &str) -> Self {
        self.attr("enctype", value)
    }

    fn method(self, value: &str) -> Self {
        self.attr("method", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn novalidate(self, value: &str) -> Self {
        self.attr("novalidate", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl FormElementAttributes for Element<'_, HtmlFormTag> {}

// ============================================================================
// Input Element Attributes
// ============================================================================

pub trait InputElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accept(self, value: &str) -> Self {
        self.attr("accept", value)
    }

    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn checked(self, value: &str) -> Self {
        self.attr("checked", value)
    }

    fn dirname(self, value: &str) -> Self {
        self.attr("dirname", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn formaction(self, value: &str) -> Self {
        self.attr("formaction", value)
    }

    fn formenctype(self, value: &str) -> Self {
        self.attr("formenctype", value)
    }

    fn formmethod(self, value: &str) -> Self {
        self.attr("formmethod", value)
    }

    fn formnovalidate(self, value: &str) -> Self {
        self.attr("formnovalidate", value)
    }

    fn formtarget(self, value: &str) -> Self {
        self.attr("formtarget", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn list(self, value: &str) -> Self {
        self.attr("list", value)
    }

    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn maxlength(self, value: &str) -> Self {
        self.attr("maxlength", value)
    }

    fn min(self, value: &str) -> Self {
        self.attr("min", value)
    }

    fn minlength(self, value: &str) -> Self {
        self.attr("minlength", value)
    }

    fn multiple(self, value: &str) -> Self {
        self.attr("multiple", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn pattern(self, value: &str) -> Self {
        self.attr("pattern", value)
    }

    fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }

    fn readonly(self, value: &str) -> Self {
        self.attr("readonly", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn size(self, value: &str) -> Self {
        self.attr("size", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn step(self, value: &str) -> Self {
        self.attr("step", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl InputElementAttributes for Element<'_, HtmlInputTag> {}

// ============================================================================
// Button Element Attributes
// ============================================================================

pub trait ButtonElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn command(self, value: &str) -> Self {
        self.attr("command", value)
    }

    fn commandfor(self, value: &str) -> Self {
        self.attr("commandfor", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn formaction(self, value: &str) -> Self {
        self.attr("formaction", value)
    }

    fn formenctype(self, value: &str) -> Self {
        self.attr("formenctype", value)
    }

    fn formmethod(self, value: &str) -> Self {
        self.attr("formmethod", value)
    }

    fn formnovalidate(self, value: &str) -> Self {
        self.attr("formnovalidate", value)
    }

    fn formtarget(self, value: &str) -> Self {
        self.attr("formtarget", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl ButtonElementAttributes for Element<'_, HtmlButtonTag> {}

// ============================================================================
// Select Element Attributes
// ============================================================================

pub trait SelectElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn multiple(self, value: &str) -> Self {
        self.attr("multiple", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn size(self, value: &str) -> Self {
        self.attr("size", value)
    }
}

impl SelectElementAttributes for Element<'_, HtmlSelectTag> {}

// ============================================================================
// Textarea Element Attributes
// ============================================================================

pub trait TextareaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn cols(self, value: &str) -> Self {
        self.attr("cols", value)
    }

    fn dirname(self, value: &str) -> Self {
        self.attr("dirname", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn maxlength(self, value: &str) -> Self {
        self.attr("maxlength", value)
    }

    fn minlength(self, value: &str) -> Self {
        self.attr("minlength", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }

    fn readonly(self, value: &str) -> Self {
        self.attr("readonly", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn rows(self, value: &str) -> Self {
        self.attr("rows", value)
    }

    fn wrap(self, value: &str) -> Self {
        self.attr("wrap", value)
    }
}

impl TextareaElementAttributes for Element<'_, HtmlTextareaTag> {}

// ============================================================================
// Label Element Attributes
// ============================================================================

pub trait LabelElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn for_attr(self, value: &str) -> Self {
        self.attr("for", value)
    }
}

impl LabelElementAttributes for Element<'_, HtmlLabelTag> {}

// ============================================================================
// Option Element Attributes
// ============================================================================

pub trait OptionElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }

    fn selected(self, value: &str) -> Self {
        self.attr("selected", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl OptionElementAttributes for Element<'_, HtmlOptionTag> {}

// ============================================================================
// Optgroup Element Attributes
// ============================================================================

pub trait OptgroupElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }
}

impl OptgroupElementAttributes for Element<'_, HtmlOptgroupTag> {}

// ============================================================================
// Fieldset Element Attributes
// ============================================================================

pub trait FieldsetElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl FieldsetElementAttributes for Element<'_, HtmlFieldsetTag> {}

// ============================================================================
// Output Element Attributes
// ============================================================================

pub trait OutputElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn for_attr(self, value: &str) -> Self {
        self.attr("for", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl OutputElementAttributes for Element<'_, HtmlOutputTag> {}

// ============================================================================
// Media Element Attributes (audio, video)
// ============================================================================

pub trait MediaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autoplay(self, value: &str) -> Self {
        self.attr("autoplay", value)
    }

    fn controls(self, value: &str) -> Self {
        self.attr("controls", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn loop_attr(self, value: &str) -> Self {
        self.attr("loop", value)
    }

    fn muted(self, value: &str) -> Self {
        self.attr("muted", value)
    }

    fn preload(self, value: &str) -> Self {
        self.attr("preload", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }
}

impl MediaElementAttributes for Element<'_, HtmlAudioTag> {}
impl MediaElementAttributes for Element<'_, HtmlVideoTag> {}

// ============================================================================
// Video Element Attributes (video-specific, in addition to media)
// ============================================================================

pub trait VideoElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn playsinline(self, value: &str) -> Self {
        self.attr("playsinline", value)
    }

    fn poster(self, value: &str) -> Self {
        self.attr("poster", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl VideoElementAttributes for Element<'_, HtmlVideoTag> {}

// ============================================================================
// Image Element Attributes
// ============================================================================

pub trait ImageElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn decoding(self, value: &str) -> Self {
        self.attr("decoding", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn ismap(self, value: &str) -> Self {
        self.attr("ismap", value)
    }

    fn loading(self, value: &str) -> Self {
        self.attr("loading", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn sizes(self, value: &str) -> Self {
        self.attr("sizes", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcset(self, value: &str) -> Self {
        self.attr("srcset", value)
    }

    fn usemap(self, value: &str) -> Self {
        self.attr("usemap", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl ImageElementAttributes for Element<'_, HtmlImgTag> {}

// ============================================================================
// Anchor/Link Element Attributes
// ============================================================================

pub trait AnchorElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn download(self, value: &str) -> Self {
        self.attr("download", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn hreflang(self, value: &str) -> Self {
        self.attr("hreflang", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl AnchorElementAttributes for Element<'_, HtmlATag> {}

// ============================================================================
// Link Element Attributes (for <link> tag)
// ============================================================================

pub trait LinkElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn as_attr(self, value: &str) -> Self {
        self.attr("as", value)
    }

    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn hreflang(self, value: &str) -> Self {
        self.attr("hreflang", value)
    }

    fn imagesizes(self, value: &str) -> Self {
        self.attr("imagesizes", value)
    }

    fn imagesrcset(self, value: &str) -> Self {
        self.attr("imagesrcset", value)
    }

    fn integrity(self, value: &str) -> Self {
        self.attr("integrity", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl LinkElementAttributes for Element<'_, HtmlLinkTag> {}

// ============================================================================
// Area Element Attributes
// ============================================================================

pub trait AreaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn coords(self, value: &str) -> Self {
        self.attr("coords", value)
    }

    fn download(self, value: &str) -> Self {
        self.attr("download", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn shape(self, value: &str) -> Self {
        self.attr("shape", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl AreaElementAttributes for Element<'_, HtmlAreaTag> {}

// ============================================================================
// Iframe Element Attributes
// ============================================================================

pub trait IframeElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn allow(self, value: &str) -> Self {
        self.attr("allow", value)
    }

    fn allowfullscreen(self, value: &str) -> Self {
        self.attr("allowfullscreen", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn loading(self, value: &str) -> Self {
        self.attr("loading", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn sandbox(self, value: &str) -> Self {
        self.attr("sandbox", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcdoc(self, value: &str) -> Self {
        self.attr("srcdoc", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl IframeElementAttributes for Element<'_, HtmlIframeTag> {}

// ============================================================================
// Embed Element Attributes
// ============================================================================

pub trait EmbedElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl EmbedElementAttributes for Element<'_, HtmlEmbedTag> {}

// ============================================================================
// Object Element Attributes
// ============================================================================

pub trait ObjectElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn data(self, value: &str) -> Self {
        self.attr("data", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn usemap(self, value: &str) -> Self {
        self.attr("usemap", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl ObjectElementAttributes for Element<'_, HtmlObjectTag> {}

// ============================================================================
// Source Element Attributes
// ============================================================================

pub trait SourceElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    fn sizes(self, value: &str) -> Self {
        self.attr("sizes", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcset(self, value: &str) -> Self {
        self.attr("srcset", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl SourceElementAttributes for Element<'_, HtmlSourceTag> {}

// ============================================================================
// Track Element Attributes
// ============================================================================

pub trait TrackElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn default(self, value: &str) -> Self {
        self.attr("default", value)
    }

    fn kind(self, value: &str) -> Self {
        self.attr("kind", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srclang(self, value: &str) -> Self {
        self.attr("srclang", value)
    }
}

impl TrackElementAttributes for Element<'_, HtmlTrackTag> {}

// ============================================================================
// Map Element Attributes
// ============================================================================

pub trait MapElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl MapElementAttributes for Element<'_, HtmlMapTag> {}

// ============================================================================
// Table Cell Attributes (td, th)
// ============================================================================

pub trait TableCellAttributes
where
    Self: ElementAttributor + Sized,
{
    fn colspan(self, value: &str) -> Self {
        self.attr("colspan", value)
    }

    fn headers(self, value: &str) -> Self {
        self.attr("headers", value)
    }

    fn rowspan(self, value: &str) -> Self {
        self.attr("rowspan", value)
    }
}

impl TableCellAttributes for Element<'_, HtmlTdTag> {}
impl TableCellAttributes for Element<'_, HtmlThTag> {}

// ============================================================================
// Th Element Attributes (in addition to TableCellAttributes)
// ============================================================================

pub trait ThElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn abbr(self, value: &str) -> Self {
        self.attr("abbr", value)
    }

    fn scope(self, value: &str) -> Self {
        self.attr("scope", value)
    }
}

impl ThElementAttributes for Element<'_, HtmlThTag> {}

// ============================================================================
// Col/Colgroup Element Attributes
// ============================================================================

pub trait ColElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn span(self, value: &str) -> Self {
        self.attr("span", value)
    }
}

impl ColElementAttributes for Element<'_, HtmlColTag> {}
impl ColElementAttributes for Element<'_, HtmlColgroupTag> {}

// ============================================================================
// Script Element Attributes
// ============================================================================

pub trait ScriptElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn async_attr(self, value: &str) -> Self {
        self.attr("async", value)
    }

    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn defer(self, value: &str) -> Self {
        self.attr("defer", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn integrity(self, value: &str) -> Self {
        self.attr("integrity", value)
    }

    fn nomodule(self, value: &str) -> Self {
        self.attr("nomodule", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl ScriptElementAttributes for Element<'_, HtmlScriptTag> {}

// ============================================================================
// Style Element Attributes
// ============================================================================

pub trait StyleElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }
}

impl StyleElementAttributes for Element<'_, HtmlStyleTag> {}

// ============================================================================
// Meta Element Attributes
// ============================================================================

pub trait MetaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn charset(self, value: &str) -> Self {
        self.attr("charset", value)
    }

    fn content(self, value: &str) -> Self {
        self.attr("content", value)
    }

    fn http_equiv(self, value: &str) -> Self {
        self.attr("http-equiv", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }
}

impl MetaElementAttributes for Element<'_, HtmlMetaTag> {}

// ============================================================================
// Base Element Attributes
// ============================================================================

pub trait BaseElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl BaseElementAttributes for Element<'_, HtmlBaseTag> {}

// ============================================================================
// Blockquote/Q Element Attributes
// ============================================================================

pub trait QuoteElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cite(self, value: &str) -> Self {
        self.attr("cite", value)
    }
}

impl QuoteElementAttributes for Element<'_, HtmlBlockquoteTag> {}
impl QuoteElementAttributes for Element<'_, HtmlQTag> {}

// ============================================================================
// Del/Ins Element Attributes
// ============================================================================

pub trait ModElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cite(self, value: &str) -> Self {
        self.attr("cite", value)
    }

    fn datetime(self, value: &str) -> Self {
        self.attr("datetime", value)
    }
}

impl ModElementAttributes for Element<'_, HtmlDelTag> {}
impl ModElementAttributes for Element<'_, HtmlInsTag> {}

// ============================================================================
// Time Element Attributes
// ============================================================================

pub trait TimeElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn datetime(self, value: &str) -> Self {
        self.attr("datetime", value)
    }
}

impl TimeElementAttributes for Element<'_, HtmlTimeTag> {}

// ============================================================================
// Data Element Attributes
// ============================================================================

pub trait DataElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl DataElementAttributes for Element<'_, HtmlDataTag> {}

// ============================================================================
// Meter Element Attributes
// ============================================================================

pub trait MeterElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn high(self, value: &str) -> Self {
        self.attr("high", value)
    }

    fn low(self, value: &str) -> Self {
        self.attr("low", value)
    }

    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn min(self, value: &str) -> Self {
        self.attr("min", value)
    }

    fn optimum(self, value: &str) -> Self {
        self.attr("optimum", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl MeterElementAttributes for Element<'_, HtmlMeterTag> {}

// ============================================================================
// Progress Element Attributes
// ============================================================================

pub trait ProgressElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl ProgressElementAttributes for Element<'_, HtmlProgressTag> {}

// ============================================================================
// Details Element Attributes
// ============================================================================

pub trait DetailsElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn open(self, value: &str) -> Self {
        self.attr("open", value)
    }
}

impl DetailsElementAttributes for Element<'_, HtmlDetailsTag> {}

// ============================================================================
// Dialog Element Attributes
// ============================================================================

pub trait DialogElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn open(self, value: &str) -> Self {
        self.attr("open", value)
    }
}

impl DialogElementAttributes for Element<'_, HtmlDialogTag> {}

// ============================================================================
// Template Element Attributes
// ============================================================================

pub trait TemplateElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn shadowrootclonable(self, value: &str) -> Self {
        self.attr("shadowrootclonable", value)
    }

    fn shadowrootcustomelementregistry(self, value: &str) -> Self {
        self.attr("shadowrootcustomelementregistry", value)
    }

    fn shadowrootdelegatesfocus(self, value: &str) -> Self {
        self.attr("shadowrootdelegatesfocus", value)
    }

    fn shadowrootmode(self, value: &str) -> Self {
        self.attr("shadowrootmode", value)
    }

    fn shadowrootserializable(self, value: &str) -> Self {
        self.attr("shadowrootserializable", value)
    }
}

impl TemplateElementAttributes for Element<'_, HtmlTemplateTag> {}

// ============================================================================
// Ol Element Attributes
// ============================================================================

pub trait OlElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn reversed(self, value: &str) -> Self {
        self.attr("reversed", value)
    }

    fn start(self, value: &str) -> Self {
        self.attr("start", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl OlElementAttributes for Element<'_, HtmlOlTag> {}

// ============================================================================
// Li Element Attributes (when used in <ol>)
// ============================================================================

pub trait LiElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl LiElementAttributes for Element<'_, HtmlLiTag> {}

// ============================================================================
// Canvas Element Attributes
// ============================================================================

pub trait CanvasElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl CanvasElementAttributes for Element<'_, HtmlCanvasTag> {}
