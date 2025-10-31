mod element;
mod event_listeners;
mod internal_buffer;
mod standard_atrributes;
mod tag;
mod template;

use internal_buffer::InternalBuffer;

pub use element::Element;
pub use event_listeners::EventListeners;
pub use standard_atrributes::GlobalAttributes;
pub use tag::*;
pub use template::Template;

pub struct HtmlBuffer {
    internal_buffer: InternalBuffer,
}

impl HtmlBuffer {
    pub fn new() -> Self {
        Self {
            internal_buffer: InternalBuffer::new(),
        }
    }

    pub fn template(&mut self) -> Template<'_> {
        Template::new(&mut self.internal_buffer)
    }

    pub fn as_str(&mut self) -> &str {
        self.internal_buffer.as_str()
    }

    pub fn into_string(self) -> String {
        self.internal_buffer.into_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_buffer() {
    //     let mut buffer = HtmlBuffer::new();
    //     let mut t = buffer.template();

    //     t.div().id("wrapper").children(|t| {
    //         t.button().text("Click me!");
    //         t.span().text("Click me!");
    //     });

    //     assert_eq!(
    //         buffer.as_str(),
    //         "<div id=\"wrapper\"><button>Click me!</button><span>Click me!</span></div>"
    //     );
    // }
}
