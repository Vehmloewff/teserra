use crate::{Element, HtmlBuffer};

pub struct Template<'a> {
    buffer: &'a mut HtmlBuffer,
}

impl<'a> Template<'a> {
    /// Create a new Template from an Html buffer
    pub fn new(html: &'a mut HtmlBuffer) -> Self {
        Self { buffer: html }
    }

    fn el(&mut self) -> Element<'_> {
        Element::new(self.buffer)
    }

    /// Create a div element
    pub fn div(&mut self) -> Element<'_> {
        self.buffer.open_normal("div");
        self.el()
    }

    /// Create a section element
    pub fn section(&mut self) -> Element<'_> {
        self.buffer.open_normal("section");
        self.el()
    }

    /// Create a button element
    pub fn button(&mut self) -> Element<'_> {
        self.buffer.open_normal("button");
        self.el()
    }

    /// Create an h1 element
    pub fn h1(&mut self) -> Element<'_> {
        self.buffer.open_normal("h1");
        self.el()
    }

    /// Create an h2 element
    pub fn h2(&mut self) -> Element<'_> {
        self.buffer.open_normal("h2");
        self.el()
    }

    /// Create a p element
    pub fn p(&mut self) -> Element<'_> {
        self.buffer.open_normal("p");
        self.el()
    }

    /// Create a span element
    pub fn span(&mut self) -> Element<'_> {
        self.buffer.open_normal("span");
        self.el()
    }

    /// Create an input element (void element)
    pub fn input(&mut self) -> Element<'_> {
        self.buffer.open_void("input");
        self.el()
    }
}
