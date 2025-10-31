// ============================================================================
// HTMX Attributes (can be used on ANY HTML element)
// ============================================================================

use super::element::{Element, ElementAttributor};

pub trait HtmxAttributes
where
    Self: ElementAttributor + Sized,
{
    // Core AJAX attributes
    fn hx_get(self, value: &str) -> Self {
        self.attr("hx-get", value)
    }

    fn hx_post(self, value: &str) -> Self {
        self.attr("hx-post", value)
    }

    fn hx_put(self, value: &str) -> Self {
        self.attr("hx-put", value)
    }

    fn hx_patch(self, value: &str) -> Self {
        self.attr("hx-patch", value)
    }

    fn hx_delete(self, value: &str) -> Self {
        self.attr("hx-delete", value)
    }

    // Event handling
    fn hx_on(self, event: &str, script: &str) -> Self {
        self.attr(&format!("hx-on:{}", event), script)
    }

    fn hx_trigger(self, value: &str) -> Self {
        self.attr("hx-trigger", value)
    }

    // Target and swap
    fn hx_target(self, value: &str) -> Self {
        self.attr("hx-target", value)
    }

    fn hx_swap(self, value: &str) -> Self {
        self.attr("hx-swap", value)
    }

    fn hx_swap_oob(self, value: &str) -> Self {
        self.attr("hx-swap-oob", value)
    }

    // Content selection
    fn hx_select(self, value: &str) -> Self {
        self.attr("hx-select", value)
    }

    fn hx_select_oob(self, value: &str) -> Self {
        self.attr("hx-select-oob", value)
    }

    // Data and parameters
    fn hx_vals(self, value: &str) -> Self {
        self.attr("hx-vals", value)
    }

    fn hx_params(self, value: &str) -> Self {
        self.attr("hx-params", value)
    }

    fn hx_include(self, value: &str) -> Self {
        self.attr("hx-include", value)
    }

    // Headers and encoding
    fn hx_headers(self, value: &str) -> Self {
        self.attr("hx-headers", value)
    }

    fn hx_encoding(self, value: &str) -> Self {
        self.attr("hx-encoding", value)
    }

    // History
    fn hx_push_url(self, value: &str) -> Self {
        self.attr("hx-push-url", value)
    }

    fn hx_replace_url(self, value: &str) -> Self {
        self.attr("hx-replace-url", value)
    }

    fn hx_history(self, value: &str) -> Self {
        self.attr("hx-history", value)
    }

    fn hx_history_elt(self, value: &str) -> Self {
        self.attr("hx-history-elt", value)
    }

    // UI feedback
    fn hx_indicator(self, value: &str) -> Self {
        self.attr("hx-indicator", value)
    }

    fn hx_disabled_elt(self, value: &str) -> Self {
        self.attr("hx-disabled-elt", value)
    }

    // User prompts
    fn hx_confirm(self, value: &str) -> Self {
        self.attr("hx-confirm", value)
    }

    fn hx_prompt(self, value: &str) -> Self {
        self.attr("hx-prompt", value)
    }

    // Behavior control
    fn hx_boost(self, value: &str) -> Self {
        self.attr("hx-boost", value)
    }

    fn hx_disable(self, value: &str) -> Self {
        self.attr("hx-disable", value)
    }

    fn hx_preserve(self, value: &str) -> Self {
        self.attr("hx-preserve", value)
    }

    fn hx_validate(self, value: &str) -> Self {
        self.attr("hx-validate", value)
    }

    // Synchronization
    fn hx_sync(self, value: &str) -> Self {
        self.attr("hx-sync", value)
    }

    // Extensions
    fn hx_ext(self, value: &str) -> Self {
        self.attr("hx-ext", value)
    }

    // Inheritance
    fn hx_inherit(self, value: &str) -> Self {
        self.attr("hx-inherit", value)
    }

    fn hx_disinherit(self, value: &str) -> Self {
        self.attr("hx-disinherit", value)
    }

    // Request configuration
    fn hx_request(self, value: &str) -> Self {
        self.attr("hx-request", value)
    }

    // ========================================================================
    // SSE Extension (Server Sent Events)
    // ========================================================================

    /// Establishes a Server Sent Events connection to the specified URL
    fn sse_connect(self, value: &str) -> Self {
        self.attr("sse-connect", value)
    }

    /// Listens for the specified SSE event name and swaps content when received
    fn sse_swap(self, value: &str) -> Self {
        self.attr("sse-swap", value)
    }

    // ========================================================================
    // WebSocket Extension
    // ========================================================================

    /// Establishes a WebSocket connection to the specified URL
    fn ws_connect(self, value: &str) -> Self {
        self.attr("ws-connect", value)
    }

    /// Sends a message to the nearest WebSocket connection based on the trigger
    fn ws_send(self, value: &str) -> Self {
        self.attr("ws-send", value)
    }

    // ========================================================================
    // Preload Extension
    // ========================================================================

    /// Enables preloading on elements (values: "mousedown", "mouseover", "always", or custom event)
    fn preload(self, value: &str) -> Self {
        self.attr("preload", value)
    }

    /// Determines whether linked image resources should also be preloaded
    fn preload_images(self, value: &str) -> Self {
        self.attr("preload-images", value)
    }

    // ========================================================================
    // Class-Tools Extension
    // ========================================================================

    /// Specifies CSS classes to be swapped onto or off of elements with operations like add, remove, toggle
    fn classes(self, value: &str) -> Self {
        self.attr("classes", value)
    }

    /// Same as classes but using data- prefix
    fn data_classes(self, value: &str) -> Self {
        self.attr("data-classes", value)
    }

    // ========================================================================
    // Loading-States Extension
    // ========================================================================

    /// Shows the element during a request (default: inline-block)
    fn data_loading(self, value: &str) -> Self {
        self.attr("data-loading", value)
    }

    /// Adds CSS classes to the element during a request
    fn data_loading_class(self, value: &str) -> Self {
        self.attr("data-loading-class", value)
    }

    /// Removes CSS classes from the element during a request
    fn data_loading_class_remove(self, value: &str) -> Self {
        self.attr("data-loading-class-remove", value)
    }

    /// Disables the element during a request
    fn data_loading_disable(self, value: &str) -> Self {
        self.attr("data-loading-disable", value)
    }

    /// Sets a delay (in ms) before loading states are applied (default: 200ms)
    fn data_loading_delay(self, value: &str) -> Self {
        self.attr("data-loading-delay", value)
    }

    /// Sets a different target to apply the loading states (CSS selector)
    fn data_loading_target(self, value: &str) -> Self {
        self.attr("data-loading-target", value)
    }

    /// Filters loading states to only apply for specific request paths
    fn data_loading_path(self, value: &str) -> Self {
        self.attr("data-loading-path", value)
    }

    /// Defines a scope for loading states
    fn data_loading_states(self, value: &str) -> Self {
        self.attr("data-loading-states", value)
    }

    // ========================================================================
    // Response-Targets Extension
    // ========================================================================

    /// Routes responses to different targets based on HTTP status code
    fn hx_target_status(self, status_code: &str, target: &str) -> Self {
        self.attr(&format!("hx-target-{}", status_code), target)
    }

    /// Routes 4xx and 5xx error responses to the specified target
    fn hx_target_error(self, value: &str) -> Self {
        self.attr("hx-target-error", value)
    }

    // ========================================================================
    // Multi-Swap Extension
    // ========================================================================

    // Note: multi-swap uses the standard hx-swap attribute with "multi:" prefix
    // No additional attributes needed - use hx_swap() with "multi:..." syntax

    // ========================================================================
    // Path-Params Extension
    // ========================================================================

    // Note: path-params uses URL template syntax (e.g., /users/:id)
    // No additional attributes needed - works with standard hx-get/post/etc.

    // ========================================================================
    // JSON-Enc Extension
    // ========================================================================

    // Note: json-enc uses standard hx-ext attribute to enable
    // No additional attributes needed - works automatically when enabled

    // ========================================================================
    // Additional Community Extensions
    // ========================================================================

    /// Ajax-header extension: adds X-Requested-With header
    /// Enable with hx_ext("ajax-header")

    /// Remove-me extension: removes element after swap
    fn remove_me(self, value: &str) -> Self {
        self.attr("remove-me", value)
    }

    /// Restored extension: triggers event on back button navigation
    fn restored(self, value: &str) -> Self {
        self.attr("restored", value)
    }

    /// Method-override extension: allows method override via _method parameter
    fn method_override(self, value: &str) -> Self {
        self.attr("method-override", value)
    }

    // ========================================================================
    // Client-Side-Templates Extension
    // ========================================================================

    /// Mustache template ID to render with the response
    fn mustache_template(self, value: &str) -> Self {
        self.attr("mustache-template", value)
    }

    /// Mustache template ID for rendering array responses
    fn mustache_array_template(self, value: &str) -> Self {
        self.attr("mustache-array-template", value)
    }

    /// Handlebars template ID to render with the response
    fn handlebars_template(self, value: &str) -> Self {
        self.attr("handlebars-template", value)
    }

    /// Handlebars template ID for rendering array responses
    fn handlebars_array_template(self, value: &str) -> Self {
        self.attr("handlebars-array-template", value)
    }

    /// Nunjucks template name to render with the response
    fn nunjucks_template(self, value: &str) -> Self {
        self.attr("nunjucks-template", value)
    }

    /// Nunjucks template name for rendering array responses
    fn nunjucks_array_template(self, value: &str) -> Self {
        self.attr("nunjucks-array-template", value)
    }

    // ========================================================================
    // Debug Extension
    // ========================================================================

    // Note: debug extension uses standard hx-ext attribute to enable
    // No additional attributes needed - logs all htmx events when enabled

    // ========================================================================
    // Head-Support Extension
    // ========================================================================

    /// Controls how head tag content is merged (values: "merge", "re-eval")
    fn hx_head(self, value: &str) -> Self {
        self.attr("hx-head", value)
    }

    // ========================================================================
    // Path-Deps Extension
    // ========================================================================

    /// Specifies path dependencies for triggering updates on related requests
    fn path_deps(self, value: &str) -> Self {
        self.attr("path-deps", value)
    }

    // ========================================================================
    // Idiomorph Extension
    // ========================================================================

    // Note: idiomorph uses hx-swap with "morph" values:
    // - hx-swap="morph" (outerHTML)
    // - hx-swap="morph:outerHTML"
    // - hx-swap="morph:innerHTML"
    // - hx-swap="morph:{ignoreActiveValue:true}"
    // Use the existing hx_swap() method with these values

    // ========================================================================
    // Alpine-Morph Extension
    // ========================================================================

    // Note: alpine-morph also uses hx-swap="morph" to enable Alpine.js morph plugin
    // Use the existing hx_swap() method with "morph" value

    // ========================================================================
    // Event-Header Extension
    // ========================================================================

    // Note: event-header uses standard hx-ext attribute to enable
    // Adds the Triggering-Event header with JSON serialized event details

    // ========================================================================
    // Morphdom-Swap Extension
    // ========================================================================

    // Note: morphdom-swap uses hx-swap="morph" similar to idiomorph
    // Use the existing hx_swap() method with "morph" value

    // ========================================================================
    // Ajax-Header Extension
    // ========================================================================

    // Note: ajax-header uses standard hx-ext attribute to enable
    // Adds the X-Requested-With: XMLHttpRequest header

    // ========================================================================
    // Additional Notes on Extensions
    // ========================================================================

    // Many extensions work purely through enabling them with hx-ext and don't
    // introduce new attributes. Examples include:
    // - json-enc: encodes parameters as JSON
    // - debug: logs htmx events to console
    // - event-header: adds triggering event header
    // - ajax-header: adds X-Requested-With header
    // - method-override: allows HTTP method override
    //
    // Morph-based extensions (idiomorph, alpine-morph, morphdom-swap) use
    // the standard hx-swap attribute with "morph" values.
}

impl<Tag> HtmxAttributes for Element<'_, Tag> {}
