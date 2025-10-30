use crate::{Element, element::ElementAttributor};

pub trait GlobalAttributes
where
    Self: ElementAttributor + Sized,
{
    fn abbr(self, value: &str) -> Self {
        self.attr("abbr", value)
    }

    fn accept(self, value: &str) -> Self {
        self.attr("accept", value)
    }

    fn accept_charset(self, value: &str) -> Self {
        self.attr("accept-charset", value)
    }

    fn accesskey(self, value: &str) -> Self {
        self.attr("accesskey", value)
    }

    fn action(self, value: &str) -> Self {
        self.attr("action", value)
    }

    fn allow(self, value: &str) -> Self {
        self.attr("allow", value)
    }

    fn allowfullscreen(self, value: &str) -> Self {
        self.attr("allowfullscreen", value)
    }

    fn alpha(self, value: &str) -> Self {
        self.attr("alpha", value)
    }

    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn anchor(self, value: &str) -> Self {
        self.attr("anchor", value)
    }

    fn as_attr(self, value: &str) -> Self {
        self.attr("as", value)
    }

    fn async_attr(self, value: &str) -> Self {
        self.attr("async", value)
    }

    fn autocapitalize(self, value: &str) -> Self {
        self.attr("autocapitalize", value)
    }

    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn autocorrect(self, value: &str) -> Self {
        self.attr("autocorrect", value)
    }

    fn autofocus(self, value: &str) -> Self {
        self.attr("autofocus", value)
    }

    fn autoplay(self, value: &str) -> Self {
        self.attr("autoplay", value)
    }

    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn charset(self, value: &str) -> Self {
        self.attr("charset", value)
    }

    fn checked(self, value: &str) -> Self {
        self.attr("checked", value)
    }

    fn cite(self, value: &str) -> Self {
        self.attr("cite", value)
    }

    fn closedby(self, value: &str) -> Self {
        self.attr("closedby", value)
    }

    fn color(self, value: &str) -> Self {
        self.attr("color", value)
    }

    fn colorspace(self, value: &str) -> Self {
        self.attr("colorspace", value)
    }

    fn cols(self, value: &str) -> Self {
        self.attr("cols", value)
    }

    fn colspan(self, value: &str) -> Self {
        self.attr("colspan", value)
    }

    fn command(self, value: &str) -> Self {
        self.attr("command", value)
    }

    fn commandfor(self, value: &str) -> Self {
        self.attr("commandfor", value)
    }

    fn content(self, value: &str) -> Self {
        self.attr("content", value)
    }

    fn contenteditable(self, value: &str) -> Self {
        self.attr("contenteditable", value)
    }

    fn controls(self, value: &str) -> Self {
        self.attr("controls", value)
    }

    fn coords(self, value: &str) -> Self {
        self.attr("coords", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn data(self, value: &str) -> Self {
        self.attr("data", value)
    }

    fn datetime(self, value: &str) -> Self {
        self.attr("datetime", value)
    }

    fn decoding(self, value: &str) -> Self {
        self.attr("decoding", value)
    }

    fn default(self, value: &str) -> Self {
        self.attr("default", value)
    }

    fn defer(self, value: &str) -> Self {
        self.attr("defer", value)
    }

    fn dir(self, value: &str) -> Self {
        self.attr("dir", value)
    }

    fn dirname(self, value: &str) -> Self {
        self.attr("dirname", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn download(self, value: &str) -> Self {
        self.attr("download", value)
    }

    fn draggable(self, value: &str) -> Self {
        self.attr("draggable", value)
    }

    fn enctype(self, value: &str) -> Self {
        self.attr("enctype", value)
    }

    fn enterkeyhint(self, value: &str) -> Self {
        self.attr("enterkeyhint", value)
    }

    fn exportparts(self, value: &str) -> Self {
        self.attr("exportparts", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn for_attr(self, value: &str) -> Self {
        self.attr("for", value)
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

    fn headers(self, value: &str) -> Self {
        self.attr("headers", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn hidden(self, value: &str) -> Self {
        self.attr("hidden", value)
    }

    fn high(self, value: &str) -> Self {
        self.attr("high", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn hreflang(self, value: &str) -> Self {
        self.attr("hreflang", value)
    }

    fn http_equiv(self, value: &str) -> Self {
        self.attr("http-equiv", value)
    }

    fn id(self, value: &str) -> Self {
        self.attr("id", value)
    }

    fn imagesizes(self, value: &str) -> Self {
        self.attr("imagesizes", value)
    }

    fn imagesrcset(self, value: &str) -> Self {
        self.attr("imagesrcset", value)
    }

    fn inert(self, value: &str) -> Self {
        self.attr("inert", value)
    }

    fn inputmode(self, value: &str) -> Self {
        self.attr("inputmode", value)
    }

    fn integrity(self, value: &str) -> Self {
        self.attr("integrity", value)
    }

    fn is(self, value: &str) -> Self {
        self.attr("is", value)
    }

    fn ismap(self, value: &str) -> Self {
        self.attr("ismap", value)
    }

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

    fn kind(self, value: &str) -> Self {
        self.attr("kind", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }

    fn lang(self, value: &str) -> Self {
        self.attr("lang", value)
    }

    fn list(self, value: &str) -> Self {
        self.attr("list", value)
    }

    fn loading(self, value: &str) -> Self {
        self.attr("loading", value)
    }

    fn loop_attr(self, value: &str) -> Self {
        self.attr("loop", value)
    }

    fn low(self, value: &str) -> Self {
        self.attr("low", value)
    }

    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn maxlength(self, value: &str) -> Self {
        self.attr("maxlength", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    fn method(self, value: &str) -> Self {
        self.attr("method", value)
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

    fn muted(self, value: &str) -> Self {
        self.attr("muted", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn nonce(self, value: &str) -> Self {
        self.attr("nonce", value)
    }

    fn nomodule(self, value: &str) -> Self {
        self.attr("nomodule", value)
    }

    fn novalidate(self, value: &str) -> Self {
        self.attr("novalidate", value)
    }

    fn open(self, value: &str) -> Self {
        self.attr("open", value)
    }

    fn optimum(self, value: &str) -> Self {
        self.attr("optimum", value)
    }

    fn part(self, value: &str) -> Self {
        self.attr("part", value)
    }

    fn pattern(self, value: &str) -> Self {
        self.attr("pattern", value)
    }

    fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }

    fn playsinline(self, value: &str) -> Self {
        self.attr("playsinline", value)
    }

    fn popover(self, value: &str) -> Self {
        self.attr("popover", value)
    }

    fn poster(self, value: &str) -> Self {
        self.attr("poster", value)
    }

    fn preload(self, value: &str) -> Self {
        self.attr("preload", value)
    }

    fn readonly(self, value: &str) -> Self {
        self.attr("readonly", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn reversed(self, value: &str) -> Self {
        self.attr("reversed", value)
    }

    fn role(self, value: &str) -> Self {
        self.attr("role", value)
    }

    fn rows(self, value: &str) -> Self {
        self.attr("rows", value)
    }

    fn rowspan(self, value: &str) -> Self {
        self.attr("rowspan", value)
    }

    fn sandbox(self, value: &str) -> Self {
        self.attr("sandbox", value)
    }

    fn scope(self, value: &str) -> Self {
        self.attr("scope", value)
    }

    fn selected(self, value: &str) -> Self {
        self.attr("selected", value)
    }

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

    fn shape(self, value: &str) -> Self {
        self.attr("shape", value)
    }

    fn size(self, value: &str) -> Self {
        self.attr("size", value)
    }

    fn sizes(self, value: &str) -> Self {
        self.attr("sizes", value)
    }

    fn slot(self, value: &str) -> Self {
        self.attr("slot", value)
    }

    fn span(self, value: &str) -> Self {
        self.attr("span", value)
    }

    fn spellcheck(self, value: &str) -> Self {
        self.attr("spellcheck", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcdoc(self, value: &str) -> Self {
        self.attr("srcdoc", value)
    }

    fn srcset(self, value: &str) -> Self {
        self.attr("srcset", value)
    }

    fn srclang(self, value: &str) -> Self {
        self.attr("srclang", value)
    }

    fn start(self, value: &str) -> Self {
        self.attr("start", value)
    }

    fn step(self, value: &str) -> Self {
        self.attr("step", value)
    }

    fn style(self, value: &str) -> Self {
        self.attr("style", value)
    }

    fn tabindex(self, value: &str) -> Self {
        self.attr("tabindex", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }

    fn title(self, value: &str) -> Self {
        self.attr("title", value)
    }

    fn translate(self, value: &str) -> Self {
        self.attr("translate", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn usemap(self, value: &str) -> Self {
        self.attr("usemap", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }

    fn virtualkeyboardpolicy(self, value: &str) -> Self {
        self.attr("virtualkeyboardpolicy", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn wrap(self, value: &str) -> Self {
        self.attr("wrap", value)
    }

    fn writingsuggestions(self, value: &str) -> Self {
        self.attr("writingsuggestions", value)
    }
}

impl<Tag> GlobalAttributes for Element<'_, Tag> {}
