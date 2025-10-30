use log::{error, warn};
use std::mem::replace;

enum CurrentTag {
    /// The is no current tag
    None,
    /// There is a tag, but it is void tag (can be self-closed)
    Void,
    /// There is a tag and it needs to be closed the normal way
    Normal(String),
}

/// HTML builder for constructing HTML strings with a fluent API
/// This uses a streaming approach where each operation directly mutates the buffer
pub struct InternalBuffer {
    /// This be where all the html is written to
    buffer: String,
    /// This is where all the classes for the current element are stored
    current_classes: String,
    /// The tag does not have children
    current_tag: CurrentTag,
    /// Stack of tags for all the elements with children
    parent_tag_stack: Vec<String>,
}

impl InternalBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            current_classes: String::new(),
            current_tag: CurrentTag::None,
            parent_tag_stack: Vec::new(),
        }
    }

    fn close_current_tag(&mut self) {
        let tag = replace(&mut self.current_tag, CurrentTag::None);

        match tag {
            CurrentTag::Normal(tag) => {
                self.finish_classes();
                self.buffer.push('>');
                self.buffer.push_str("</");
                self.buffer.push_str(&tag);
                self.buffer.push('>');
            }
            CurrentTag::Void => {
                self.finish_classes();
                self.buffer.push('>');
            }
            CurrentTag::None => {}
        }
    }

    fn finish_classes(&mut self) {
        if !self.current_classes.is_empty() {
            self.buffer.push(' ');
            self.buffer.push_str("class=\"");
            escape_attr_into(&mut self.buffer, &self.current_classes);
            self.buffer.push('"');
            self.current_classes.clear();
        }
    }

    /// Open a normal element; an element that opens and closes the normal way. This is to be distinguished from void elements, which do not close.
    pub fn open_normal(&mut self, tag: &str) {
        self.close_current_tag();
        self.buffer.push_str("<");
        self.buffer.push_str(tag);
        self.current_tag = CurrentTag::Normal(tag.to_string());
    }

    pub fn open_void(&mut self, tag: &str) {
        self.close_current_tag();
        self.buffer.push_str("<");
        self.buffer.push_str(tag);
        self.current_tag = CurrentTag::Void;
    }

    pub fn attr(&mut self, name: &str, value: &str) {
        if name.is_empty() {
            warn!("tried to add html attribute with an empty name; skipping");
            return;
        }

        self.buffer.push_str(" ");
        self.buffer.push_str(name);
        self.buffer.push_str("=\"");
        escape_attr_into(&mut self.buffer, value);
        self.buffer.push('"');
    }

    pub fn class(&mut self, class: &str) {
        if !self.current_classes.is_empty() {
            self.current_classes.push(' ');
        }

        if class.is_empty() {
            warn!("tried to add html class with an empty name; skipping");
            return;
        }

        self.current_classes.push_str(class)
    }

    pub fn start_children(&mut self) {
        let current_tag = replace(&mut self.current_tag, CurrentTag::None);

        match current_tag {
            CurrentTag::Normal(tag) => {
                self.finish_classes();
                self.buffer.push('>');
                self.parent_tag_stack.push(tag);
            }
            CurrentTag::Void => {
                error!(
                    "called start children, but there was currently a void element; siblings will be created instead"
                )
            }
            CurrentTag::None => {
                error!("called start children, but there was currently no element; ignoring")
            }
        }
    }

    pub fn push_text(&mut self, text: &str) {
        escape_html_into(&mut self.buffer, text);
    }

    pub fn end_children(&mut self) {
        self.close_current_tag();

        let last_tag = match self.parent_tag_stack.pop() {
            Some(tag) => tag,
            None => {
                error!("called end children, but there was currently no element; ignoring");
                return;
            }
        };

        self.buffer.push_str("</");
        self.buffer.push_str(&last_tag);
        self.buffer.push('>');
    }

    pub fn as_str(&mut self) -> &str {
        // Close any remaining unclosed tag
        self.close_current_tag();

        &self.buffer
    }

    pub fn into_string(mut self) -> String {
        // Close any remaining unclosed tag
        self.close_current_tag();

        self.buffer
    }
}

fn escape_html_into(buffer: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '&' => buffer.push_str("&amp;"),
            '<' => buffer.push_str("&lt;"),
            '>' => buffer.push_str("&gt;"),
            _ => buffer.push(ch),
        }
    }
}

fn escape_attr_into(buffer: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '&' => buffer.push_str("&amp;"),
            '"' => buffer.push_str("&quot;"),
            '<' => buffer.push_str("&lt;"),
            '>' => buffer.push_str("&gt;"),
            _ => buffer.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_normal_element() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("class", "container");
        assert_eq!(html.into_string(), r#"<div class="container"></div>"#);
    }

    #[test]
    fn test_void_element_with_attr() {
        let mut html = InternalBuffer::new();
        html.open_void("input");
        html.attr("value", "hello");
        assert_eq!(html.into_string(), r#"<input value="hello">"#);
    }

    #[test]
    fn test_nested_children() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_void("input");
        html.attr("value", "hello");
        html.end_children();
        assert_eq!(html.into_string(), r#"<div><input value="hello"></div>"#);
    }

    #[test]
    fn test_text_content() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.push_text("Hello, world!");
        html.end_children();
        assert_eq!(html.into_string(), r#"<div>Hello, world!</div>"#);
    }

    #[test]
    fn test_multiple_children() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_normal("h1");
        html.start_children();
        html.push_text("Title");
        html.end_children();
        html.open_normal("p");
        html.start_children();
        html.push_text("Paragraph");
        html.end_children();
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div><h1>Title</h1><p>Paragraph</p></div>"#
        );
    }

    #[test]
    fn test_multiple_attributes() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("style", "color: red; font-size: 16px;");
        assert_eq!(
            html.into_string(),
            r#"<div style="color: red; font-size: 16px;"></div>"#
        );
    }

    #[test]
    fn test_auto_close_siblings() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        // These siblings should auto-close each other
        html.open_normal("p");
        html.attr("class", "first");
        html.open_normal("p");
        html.attr("class", "second");
        html.open_normal("p");
        html.attr("class", "third");
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div><p class="first"></p><p class="second"></p><p class="third"></p></div>"#
        );
    }

    #[test]
    fn test_void_element_auto_close() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_void("input");
        html.attr("value", "test");
        html.open_normal("span");
        html.start_children();
        html.push_text("Label");
        html.end_children();
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div><input value="test"><span>Label</span></div>"#
        );
    }

    #[test]
    fn test_escape_html_in_text() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.push_text("<script>alert('xss')</script>");
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div>&lt;script&gt;alert('xss')&lt;/script&gt;</div>"#
        );
    }

    #[test]
    fn test_escape_attr_value() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("data-value", r#"foo"bar&baz"#);
        assert_eq!(
            html.into_string(),
            r#"<div data-value="foo&quot;bar&amp;baz"></div>"#
        );
    }

    #[test]
    fn test_single_class_on_normal_element() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("container");
        assert_eq!(html.into_string(), r#"<div class="container"></div>"#);
    }

    #[test]
    fn test_multiple_classes_on_normal_element() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("container");
        html.class("active");
        html.class("primary");
        assert_eq!(
            html.into_string(),
            r#"<div class="container active primary"></div>"#
        );
    }

    #[test]
    fn test_single_class_on_void_element() {
        let mut html = InternalBuffer::new();
        html.open_void("input");
        html.class("form-control");
        assert_eq!(html.into_string(), r#"<input class="form-control">"#);
    }

    #[test]
    fn test_multiple_classes_on_void_element() {
        let mut html = InternalBuffer::new();
        html.open_void("input");
        html.class("form-control");
        html.class("input-lg");
        html.class("required");
        assert_eq!(
            html.into_string(),
            r#"<input class="form-control input-lg required">"#
        );
    }

    #[test]
    fn test_classes_with_attributes_before() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("id", "main");
        html.attr("data-value", "test");
        html.class("container");
        html.class("active");
        assert_eq!(
            html.into_string(),
            r#"<div id="main" data-value="test" class="container active"></div>"#
        );
    }

    #[test]
    fn test_classes_with_attributes_after() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("container");
        html.class("active");
        html.attr("id", "main");
        html.attr("data-value", "test");
        assert_eq!(
            html.into_string(),
            r#"<div id="main" data-value="test" class="container active"></div>"#
        );
    }

    #[test]
    fn test_classes_mixed_with_attributes() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("id", "main");
        html.class("container");
        html.attr("data-value", "test");
        html.class("active");
        html.attr("role", "main");

        assert_eq!(
            html.into_string(),
            r#"<div id="main" data-value="test" role="main" class="container active"></div>"#
        );
    }

    #[test]
    fn test_classes_on_nested_elements() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("outer");
        html.start_children();
        html.open_normal("span");
        html.class("inner");
        html.class("highlight");
        html.end_children();

        assert_eq!(
            html.into_string(),
            r#"<div class="outer"><span class="inner highlight"></span></div>"#
        );
    }

    #[test]
    fn test_classes_on_sibling_elements() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_normal("p");
        html.class("first");
        html.class("item");
        html.open_normal("p");
        html.class("second");
        html.class("item");
        html.open_normal("p");
        html.class("third");
        html.class("item");
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div><p class="first item"></p><p class="second item"></p><p class="third item"></p></div>"#
        );
    }

    #[test]
    fn test_no_classes_no_class_attribute() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.attr("id", "test");
        assert_eq!(html.into_string(), r#"<div id="test"></div>"#);
    }

    #[test]
    fn test_classes_with_special_characters() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("foo<bar");
        html.class("baz>qux");
        html.class(r#"test"quote"#);
        html.class("amp&ersand");

        assert_eq!(
            html.into_string(),
            r#"<div class="foo&lt;bar baz&gt;qux test&quot;quote amp&amp;ersand"></div>"#
        );
    }

    #[test]
    fn test_classes_on_element_with_text_children() {
        let mut html = InternalBuffer::new();
        html.open_normal("p");
        html.class("paragraph");
        html.class("bold");
        html.start_children();
        html.push_text("Hello, world!");
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<p class="paragraph bold">Hello, world!</p>"#
        );
    }

    #[test]
    fn test_classes_on_deeply_nested_elements() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("level-1");
        html.start_children();
        html.open_normal("div");
        html.class("level-2");
        html.start_children();
        html.open_normal("div");
        html.class("level-3");
        html.start_children();
        html.open_normal("span");
        html.class("level-4");
        html.end_children();
        html.end_children();
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div class="level-1"><div class="level-2"><div class="level-3"><span class="level-4"></span></div></div></div>"#
        );
    }

    #[test]
    fn test_void_element_classes_with_siblings() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_void("input");
        html.class("input-1");
        html.attr("type", "text");
        html.open_void("input");
        html.class("input-2");
        html.class("required");
        html.attr("type", "password");
        html.open_normal("button");
        html.class("btn");
        html.class("btn-primary");
        html.end_children();

        assert_eq!(
            html.into_string(),
            r#"<div><input type="text" class="input-1"><input type="password" class="input-2 required"><button class="btn btn-primary"></button></div>"#
        );
    }

    #[test]
    fn test_classes_finish_on_start_children() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("parent");
        html.start_children();
        html.push_text("content");
        html.end_children();
        assert_eq!(html.into_string(), r#"<div class="parent">content</div>"#);
    }

    #[test]
    fn test_classes_finish_on_close_tag() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.start_children();
        html.open_void("input");
        html.class("form-input");
        // The void element should close and flush classes
        html.end_children();
        assert_eq!(
            html.into_string(),
            r#"<div><input class="form-input"></div>"#
        );
    }

    #[test]
    fn test_empty_class_strings() {
        let mut html = InternalBuffer::new();
        html.open_normal("div");
        html.class("");
        html.class("valid");
        html.class("");

        assert_eq!(html.into_string(), r#"<div class="valid "></div>"#);
    }
}
