# Tessera

A fast html builder library.

- Contains element and attribute functions for every html element and attribute.
- Also has opt-in support for all of the htmx attributes too.

## Usage

```rust
use tessera::{HtmlBuffer, GlobalAttributes};

let mut buffer = HtmlBuffer::new();
let mut t = buffer.template();

// Every html element has a function: t.div(), t.h1(), t.p(), etc.
t.div()
	// All global html attributes are supplied via the `GlobalAttributes` trait.
	.id("wrapper")
	// Create children elements by writing to the `t` provided when calling `.children()`.
	.children(|t| {
	    t.h1().text("Hello, World!");
	    t.p().text("Welcome to Tessera!");

	    t.a()
			// Normally, calling an attribute method sets the attribute value.
			.href("https://example.com")
			// However the class attribute is special because each call adds to the classes that already exist.
			.class("btn")
			.class_if(link_is_active, "active")9
			// Calling text is shorthand for .children(|t| t.text_node(...))
			.text("Visit Example");
	});

let html = buffer.into_string();
// Output: <div id="wrapper"><h1>Hello, World!</h1><p>Welcome to Tessera!</p></div>
```
