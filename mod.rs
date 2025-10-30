mod html_buffer;

pub use html_buffer::HtmlBuffer;

pub struct Template<'a> {
    html: &'a mut HtmlBuffer,
}

impl<'a> Template<'a> {
    /// Create a new Template from an Html buffer
    pub fn new(html: &'a mut HtmlBuffer) -> Self {
        Self { html }
    }

    fn el(&mut self) -> Element<'_> {
        Element { html: self.html }
    }

    /// Create a div element
    pub fn div(&mut self) -> Element<'_> {
        self.html.open_normal("div");
        self.el()
    }

    /// Create a section element
    pub fn section(&mut self) -> Element<'_> {
        self.html.open_normal("section");
        self.el()
    }

    /// Create a button element
    pub fn button(&mut self) -> Element<'_> {
        self.html.open_normal("button");
        self.el()
    }

    /// Create an h1 element
    pub fn h1(&mut self) -> Element<'_> {
        self.html.open_normal("h1");
        self.el()
    }

    /// Create an h2 element
    pub fn h2(&mut self) -> Element<'_> {
        self.html.open_normal("h2");
        self.el()
    }

    /// Create a p element
    pub fn p(&mut self) -> Element<'_> {
        self.html.open_normal("p");
        self.el()
    }

    /// Create a span element
    pub fn span(&mut self) -> Element<'_> {
        self.html.open_normal("span");
        self.el()
    }

    /// Create an input element (void element)
    pub fn input(&mut self) -> Element<'_> {
        self.html.open_void("input");
        self.el()
    }
}

/// Represents an HTML element that is being built
/// This holds a mutable reference to the Html buffer and writes to it directly
pub struct Element<'a> {
    html: &'a mut HtmlBuffer,
}

impl<'a> Element<'a> {
    pub fn new(html: &'a mut HtmlBuffer) -> Self {
        Self { html }
    }

    /// Add a value attribute (for inputs)
    pub fn value(self, value: &str) -> Self {
        self.html.attr("value", value);
        self
    }

    /// Add an id attribute
    pub fn id(self, id: &str) -> Self {
        self.html.attr("id", id);
        self
    }

    /// Add text content and close the element
    pub fn text(self, content: &str) {
        self.html.start_children();
        self.html.push_text(content);
        self.html.end_children();
    }

    /// Add children using a closure that receives a mutable reference to the Template
    pub fn children<F>(self, f: F)
    where
        F: FnOnce(&mut Template),
    {
        self.html.start_children();
        let mut template = Template { html: self.html };
        f(&mut template);
        template.html.end_children();
    }
}
