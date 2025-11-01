use log::error;
use std::marker::PhantomData;

use super::{InternalBuffer, Template, tag::HtmlTag};

pub trait ElementAttributor {
    fn attr(self, name: &str, value: &str) -> Self;
}

/// Represents an HTML element that is being built
/// This holds a mutable reference to the Html buffer and writes to it directly
pub struct Element<'a, Tag> {
    buffer: &'a mut InternalBuffer,
    _marker: PhantomData<Tag>,
}

impl<Tag> Element<'_, Tag>
where
    Tag: HtmlTag,
{
    pub fn new<'a>(html: &'a mut InternalBuffer) -> Element<'a, Tag> {
        Element {
            buffer: html,
            _marker: PhantomData,
        }
    }

    /// Add a class to the element. Can be called multiple times to add multiple classes.
    pub fn class(self, class: &str) -> Self {
        self.buffer.class(class);
        self
    }

    /// Add a class to the element if the condition is true. Can be called multiple times to add multiple classes.
    pub fn class_if(self, condition: bool, class: &str) -> Self {
        if condition { self.class(class) } else { self }
    }

    /// Add text content and close the element
    pub fn text(self, content: &str) {
        self.buffer.start_children();
        self.buffer.push_text(content);
        self.buffer.end_children();
    }

    /// Add children using a closure that receives a mutable reference to the Template
    pub fn children<F>(self, f: F)
    where
        F: FnOnce(&mut Template),
    {
        self.buffer.start_children();
        let mut template = Template::new(self.buffer);
        f(&mut template);
        self.buffer.end_children();
    }

    pub fn custom_attribute(self, name: &str, value: &str) -> Self {
        if name == "class" {
            error!(
                "tried to set class attribute using `custom_attribute`; this is not permitted because classes are internally buffered; ignoring"
            );
        } else {
            self.buffer.attr(name, value);
        }

        self
    }
}

impl<Tag> ElementAttributor for Element<'_, Tag> {
    #[inline]
    fn attr(self, name: &str, value: &str) -> Self {
        self.buffer.attr(name, value);
        self
    }
}
