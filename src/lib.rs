//! yate is a simple, idiomatic, combined html template engine and static site
//! generator.
//!
//! Templating is done through composing html macro generated string chunks that
//! can be returned by Rust functions. The macro, powered by [syn-rsx], supports
//! arbitrary Rust syntax in braced blocks in various html positions which
//! unlocks using regular language features without being limited by specialized
//! template syntax. The fact that the composed content is just a string makes
//! it very flexible and allows e.g. serving it with a webserver or write it as
//! static content.
//!
//! # Example
//! ```
//! # async {
//! use yate::html;
//!
//! fn content() -> String {
//!     html! { <div>"content"</div> }
//! }
//!
//! fn index() -> String {
//!     html! {
//!         <!DOCTYPE html>
//!         <html>
//!             <head></head>
//!             <body>
//!                 {content()}
//!             </body>
//!         </html>
//!     }
//! }
//!
//! std::fs::write("index.html", index())?;
//! #
//! # Ok::<(), Box<dyn std::error::Error>>(()) };
//! ```
//!
//! [syn-rsx]: https://crates.io/crates/syn-rsx

pub use html_to_string_macro::html;
