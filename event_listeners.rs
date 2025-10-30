use crate::{Element, element::ElementAttributor};

pub trait EventListeners
where
    Self: ElementAttributor + Sized,
{
    fn onabort(self, value: &str) -> Self {
        self.attr("onabort", value)
    }

    fn onafterprint(self, value: &str) -> Self {
        self.attr("onafterprint", value)
    }

    fn onauxclick(self, value: &str) -> Self {
        self.attr("onauxclick", value)
    }

    fn onbeforematch(self, value: &str) -> Self {
        self.attr("onbeforematch", value)
    }

    fn onbeforeprint(self, value: &str) -> Self {
        self.attr("onbeforeprint", value)
    }

    fn onbeforetoggle(self, value: &str) -> Self {
        self.attr("onbeforetoggle", value)
    }

    fn onbeforeunload(self, value: &str) -> Self {
        self.attr("onbeforeunload", value)
    }

    fn onblur(self, value: &str) -> Self {
        self.attr("onblur", value)
    }

    fn oncancel(self, value: &str) -> Self {
        self.attr("oncancel", value)
    }

    fn oncanplay(self, value: &str) -> Self {
        self.attr("oncanplay", value)
    }

    fn oncanplaythrough(self, value: &str) -> Self {
        self.attr("oncanplaythrough", value)
    }

    fn onchange(self, value: &str) -> Self {
        self.attr("onchange", value)
    }

    fn onclick(self, value: &str) -> Self {
        self.attr("onclick", value)
    }

    fn onclose(self, value: &str) -> Self {
        self.attr("onclose", value)
    }

    fn oncontextlost(self, value: &str) -> Self {
        self.attr("oncontextlost", value)
    }

    fn oncontextmenu(self, value: &str) -> Self {
        self.attr("oncontextmenu", value)
    }

    fn oncontextrestored(self, value: &str) -> Self {
        self.attr("oncontextrestored", value)
    }

    fn oncopy(self, value: &str) -> Self {
        self.attr("oncopy", value)
    }

    fn oncuechange(self, value: &str) -> Self {
        self.attr("oncuechange", value)
    }

    fn oncut(self, value: &str) -> Self {
        self.attr("oncut", value)
    }

    fn ondblclick(self, value: &str) -> Self {
        self.attr("ondblclick", value)
    }

    fn ondrag(self, value: &str) -> Self {
        self.attr("ondrag", value)
    }

    fn ondragend(self, value: &str) -> Self {
        self.attr("ondragend", value)
    }

    fn ondragenter(self, value: &str) -> Self {
        self.attr("ondragenter", value)
    }

    fn ondragleave(self, value: &str) -> Self {
        self.attr("ondragleave", value)
    }

    fn ondragover(self, value: &str) -> Self {
        self.attr("ondragover", value)
    }

    fn ondragstart(self, value: &str) -> Self {
        self.attr("ondragstart", value)
    }

    fn ondrop(self, value: &str) -> Self {
        self.attr("ondrop", value)
    }

    fn ondurationchange(self, value: &str) -> Self {
        self.attr("ondurationchange", value)
    }

    fn onemptied(self, value: &str) -> Self {
        self.attr("onemptied", value)
    }

    fn onended(self, value: &str) -> Self {
        self.attr("onended", value)
    }

    fn onerror(self, value: &str) -> Self {
        self.attr("onerror", value)
    }

    fn onfocus(self, value: &str) -> Self {
        self.attr("onfocus", value)
    }

    fn onformdata(self, value: &str) -> Self {
        self.attr("onformdata", value)
    }

    fn onhashchange(self, value: &str) -> Self {
        self.attr("onhashchange", value)
    }

    fn oninput(self, value: &str) -> Self {
        self.attr("oninput", value)
    }

    fn oninvalid(self, value: &str) -> Self {
        self.attr("oninvalid", value)
    }

    fn onkeydown(self, value: &str) -> Self {
        self.attr("onkeydown", value)
    }

    fn onkeypress(self, value: &str) -> Self {
        self.attr("onkeypress", value)
    }

    fn onkeyup(self, value: &str) -> Self {
        self.attr("onkeyup", value)
    }

    fn onlanguagechange(self, value: &str) -> Self {
        self.attr("onlanguagechange", value)
    }

    fn onload(self, value: &str) -> Self {
        self.attr("onload", value)
    }

    fn onloadeddata(self, value: &str) -> Self {
        self.attr("onloadeddata", value)
    }

    fn onloadedmetadata(self, value: &str) -> Self {
        self.attr("onloadedmetadata", value)
    }

    fn onloadstart(self, value: &str) -> Self {
        self.attr("onloadstart", value)
    }

    fn onmessage(self, value: &str) -> Self {
        self.attr("onmessage", value)
    }

    fn onmessageerror(self, value: &str) -> Self {
        self.attr("onmessageerror", value)
    }

    fn onmousedown(self, value: &str) -> Self {
        self.attr("onmousedown", value)
    }

    fn onmouseenter(self, value: &str) -> Self {
        self.attr("onmouseenter", value)
    }

    fn onmouseleave(self, value: &str) -> Self {
        self.attr("onmouseleave", value)
    }

    fn onmousemove(self, value: &str) -> Self {
        self.attr("onmousemove", value)
    }

    fn onmouseout(self, value: &str) -> Self {
        self.attr("onmouseout", value)
    }

    fn onmouseover(self, value: &str) -> Self {
        self.attr("onmouseover", value)
    }

    fn onmouseup(self, value: &str) -> Self {
        self.attr("onmouseup", value)
    }

    fn onoffline(self, value: &str) -> Self {
        self.attr("onoffline", value)
    }

    fn ononline(self, value: &str) -> Self {
        self.attr("ononline", value)
    }

    fn onpagehide(self, value: &str) -> Self {
        self.attr("onpagehide", value)
    }

    fn onpageshow(self, value: &str) -> Self {
        self.attr("onpageshow", value)
    }

    fn onpaste(self, value: &str) -> Self {
        self.attr("onpaste", value)
    }

    fn onpause(self, value: &str) -> Self {
        self.attr("onpause", value)
    }

    fn onplay(self, value: &str) -> Self {
        self.attr("onplay", value)
    }

    fn onplaying(self, value: &str) -> Self {
        self.attr("onplaying", value)
    }

    fn onpopstate(self, value: &str) -> Self {
        self.attr("onpopstate", value)
    }

    fn onprogress(self, value: &str) -> Self {
        self.attr("onprogress", value)
    }

    fn onratechange(self, value: &str) -> Self {
        self.attr("onratechange", value)
    }

    fn onrejectionhandled(self, value: &str) -> Self {
        self.attr("onrejectionhandled", value)
    }

    fn onreset(self, value: &str) -> Self {
        self.attr("onreset", value)
    }

    fn onresize(self, value: &str) -> Self {
        self.attr("onresize", value)
    }

    fn onscroll(self, value: &str) -> Self {
        self.attr("onscroll", value)
    }

    fn onscrollend(self, value: &str) -> Self {
        self.attr("onscrollend", value)
    }

    fn onsecuritypolicyviolation(self, value: &str) -> Self {
        self.attr("onsecuritypolicyviolation", value)
    }

    fn onseeked(self, value: &str) -> Self {
        self.attr("onseeked", value)
    }

    fn onseeking(self, value: &str) -> Self {
        self.attr("onseeking", value)
    }

    fn onselect(self, value: &str) -> Self {
        self.attr("onselect", value)
    }

    fn onslotchange(self, value: &str) -> Self {
        self.attr("onslotchange", value)
    }

    fn onstalled(self, value: &str) -> Self {
        self.attr("onstalled", value)
    }

    fn onstorage(self, value: &str) -> Self {
        self.attr("onstorage", value)
    }

    fn onsubmit(self, value: &str) -> Self {
        self.attr("onsubmit", value)
    }

    fn onsuspend(self, value: &str) -> Self {
        self.attr("onsuspend", value)
    }

    fn ontimeupdate(self, value: &str) -> Self {
        self.attr("ontimeupdate", value)
    }

    fn ontoggle(self, value: &str) -> Self {
        self.attr("ontoggle", value)
    }

    fn onunhandledrejection(self, value: &str) -> Self {
        self.attr("onunhandledrejection", value)
    }

    fn onunload(self, value: &str) -> Self {
        self.attr("onunload", value)
    }

    fn onvolumechange(self, value: &str) -> Self {
        self.attr("onvolumechange", value)
    }

    fn onwaiting(self, value: &str) -> Self {
        self.attr("onwaiting", value)
    }

    fn onwheel(self, value: &str) -> Self {
        self.attr("onwheel", value)
    }
}

impl<Tag> EventListeners for Element<'_, Tag> {}
