use log::error;
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
pub struct Html {
	buffer: String,
	/// The tag does not have children
	current_tag: CurrentTag,
	/// Stack of tags for all the elements with children
	parent_tag_stack: Vec<String>,
}

impl Html {
	pub fn new() -> Self {
		Self {
			buffer: String::new(),
			current_tag: CurrentTag::None,
			parent_tag_stack: Vec::new(),
		}
	}

	fn close_current_tag(&mut self) {
		let tag = replace(&mut self.current_tag, CurrentTag::None);

		match tag {
			CurrentTag::Normal(tag) => {
				self.buffer.push('>');
				self.buffer.push_str("</");
				self.buffer.push_str(&tag);
				self.buffer.push('>');
			}
			CurrentTag::Void => {
				self.buffer.push('>');
			}
			CurrentTag::None => {}
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
		self.buffer.push_str(" ");
		self.buffer.push_str(name);
		self.buffer.push_str("=\"");
		escape_attr_into(&mut self.buffer, value);
		self.buffer.push('"');
	}

	pub fn start_children(&mut self) {
		let current_tag = replace(&mut self.current_tag, CurrentTag::None);

		match current_tag {
			CurrentTag::Normal(tag) => {
				self.buffer.push('>');
				self.parent_tag_stack.push(tag);
			}
			CurrentTag::Void => {
				error!("called start children, but there was currently a void element; siblings will be created instead")
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
	use crate::template::Template;

	#[test]
	fn test_simple_div() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().class("container");
		assert_eq!(html.into_string(), r#"<div class="container"></div>"#);
	}

	#[test]
	fn test_input_with_value() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.input().value("hello");
		assert_eq!(html.into_string(), r#"<input value="hello">"#);
	}

	#[test]
	fn test_nested_children() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().children(|template| {
			template.input().value("hello");
		});
		assert_eq!(html.into_string(), r#"<div><input value="hello"></div>"#);
	}

	#[test]
	fn test_text_content() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().text("Hello, world!");
		assert_eq!(html.into_string(), r#"<div>Hello, world!</div>"#);
	}

	#[test]
	fn test_multiple_children() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().children(|template| {
			template.h1().text("Title");
			template.p().text("Paragraph");
		});
		assert_eq!(html.into_string(), r#"<div><h1>Title</h1><p>Paragraph</p></div>"#);
	}

	#[test]
	fn test_style_attribute() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().style("color: red; font-size: 16px;");
		assert_eq!(html.into_string(), r#"<div style="color: red; font-size: 16px;"></div>"#);
	}

	#[test]
	fn test_auto_close_siblings() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().children(|template| {
			// These siblings should auto-close each other
			template.p().class("first");
			template.p().class("second");
			template.p().class("third");
		});
		assert_eq!(
			html.into_string(),
			r#"<div><p class="first"></p><p class="second"></p><p class="third"></p></div>"#
		);
	}

	#[test]
	fn test_void_element_auto_close() {
		let mut html = Html::new();
		let mut template = Template::new(&mut html);
		template.div().children(|template| {
			template.input().value("test");
			template.span().text("Label");
		});
		assert_eq!(html.into_string(), r#"<div><input value="test"><span>Label</span></div>"#);
	}
}
