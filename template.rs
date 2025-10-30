use crate::{
    Element, InternalBuffer,
    tag::{HtmlDivTag, HtmlTag},
};

pub struct Template<'a> {
    buffer: &'a mut InternalBuffer,
}

impl<'a> Template<'a> {
    /// Create a new Template from an Html buffer
    pub fn new(html: &'a mut InternalBuffer) -> Self {
        Self { buffer: html }
    }

    fn el<Tag>(&mut self) -> Element<'_, Tag>
    where
        Tag: HtmlTag,
    {
        self.buffer.open(Tag::name(), Tag::is_void());
        Element::new(self.buffer)
    }

    /// Create a div element
    pub fn div(&mut self) -> Element<'_, HtmlDivTag> {
        self.el()
    }
}
