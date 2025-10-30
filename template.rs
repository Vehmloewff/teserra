use crate::{Element, InternalBuffer};

pub struct Template<'a> {
    buffer: &'a mut InternalBuffer,
}

impl<'a> Template<'a> {
    /// Create a new Template from an Html buffer
    pub fn new(html: &'a mut InternalBuffer) -> Self {
        Self { buffer: html }
    }

    fn el(&mut self, tag: &str) -> Element<'_> {
        self.buffer.open_normal(tag);
        Element::new(self.buffer)
    }

    fn el_void(&mut self, tag: &str) -> Element<'_> {
        self.buffer.open_void(tag);
        Element::new(self.buffer)
    }

    /// Create a div element
    pub fn div(&mut self) -> Element<'_> {
        self.el("div")
    }

    /// Create a section element
    pub fn section(&mut self) -> Element<'_> {
        self.el("section")
    }

    /// Create a button element
    pub fn button(&mut self) -> Element<'_> {
        self.el("button")
    }

    /// Create an h1 element
    pub fn h1(&mut self) -> Element<'_> {
        self.el("h1")
    }

    /// Create an h2 element
    pub fn h2(&mut self) -> Element<'_> {
        self.el("h2")
    }

    /// Create a p element
    pub fn p(&mut self) -> Element<'_> {
        self.el("p")
    }

    /// Create a span element
    pub fn span(&mut self) -> Element<'_> {
        self.el("span")
    }

    /// Create an input element
    pub fn input(&mut self) -> Element<'_> {
        self.el_void("input")
    }
}
