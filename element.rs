use crate::{InternalBuffer, Template};

/// Represents an HTML element that is being built
/// This holds a mutable reference to the Html buffer and writes to it directly
pub struct Element<'a> {
    buffer: &'a mut InternalBuffer,
}

impl<'a> Element<'a> {
    pub fn new(html: &'a mut InternalBuffer) -> Self {
        Self { buffer: html }
    }

    /// Add a value attribute (for inputs)
    pub fn value(self, value: &str) -> Self {
        self.buffer.attr("value", value);
        self
    }

    /// Add an id attribute
    pub fn id(self, id: &str) -> Self {
        self.buffer.attr("id", id);
        self
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
}
