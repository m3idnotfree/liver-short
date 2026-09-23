//! A lightweight, `no_std` JSON value position extractor.
//!
//! Returns a [`Span`] (start/end byte offsets) rather than a parsed value.
//!
//! ## Limitations
//!
//! - Object navigation only - arrays at the top level or as a path step return [`Error::is_unsupported_array`].
//! - Values are not validated — only their position is extracted.
//!
//! ## Examples
//!
//! ### Extract an object
//!
//! ```
//! let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;
//!
//! let span = liver_shot::find("a", json)?;
//! assert_eq!(r#"{"b": "value", "c": [1, 2, 3]}"#, span.get(json));
//! # Ok::<(), liver_shot::Error>(())
//! ```
//!
//! ### Extract a nested object
//!
//! ```
//! let json = r#"{"a": {"b": {"c": "value"}}}"#;
//!
//! let span = liver_shot::find("a.b", json)?;
//! assert_eq!(r#"{"c": "value"}"#, span.get(json));
//! # Ok::<(), liver_shot::Error>(())
//! ```
//!
//! ### Extract an array
//!
//! ```
//! let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;
//!
//! let span = liver_shot::find("a.c", json)?;
//! assert_eq!("[1, 2, 3]", span.get(json));
//! # Ok::<(), liver_shot::Error>(())
//! ```
//!
//! ### Reuse [`Span`]
//!
//! ```
//! let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;
//!
//! let a = liver_shot::find("a", json)?;
//! let b = a.find("b", json)?;
//! let c = a.find("c", json)?;
//!
//! assert_eq!(r#"{"b": "value", "c": [1, 2, 3]}"#, a.get(json));
//! assert_eq!(r#""value""#, b.get(json));
//! assert_eq!("[1, 2, 3]", c.get(json));
//! # Ok::<(), liver_shot::Error>(())
//! ```

#![no_std]

mod cursor;
mod error;
mod span;

pub use error::Error;
pub use span::{Span, Value};

pub fn find(pattern: &str, data: &str) -> Result<Span, Error> {
    let mut cursor = cursor::Cursor::new(data.as_bytes(), 0);
    cursor.skip_whitespace();
    cursor.find(pattern)
}
