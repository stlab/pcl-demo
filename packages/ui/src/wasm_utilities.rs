//! Utilities for working with WebAssembly interop and JavaScript errors.
//!
//! This module provides:
//! - `JsonEncodedError`: an error type that encodes a JavaScript error as JSON and is compatible with `anyhow`.
//! - `JsResultExt::normalized()`: an extension trait for converting `Result<T, JsValue>` into
//!   `Result<T, JsonEncodedError>`, panicking when encountering JavaScript exceptions that represent
//!   precondition violations (e.g., `TypeError`).

use std::fmt;

use serde_json::Value as JsonValue;
use wasm_bindgen::prelude::JsValue;

/// Error type that JSON-encodes a JavaScript error value.
#[derive(Debug)]
pub struct JsonEncodedError(pub JsonValue);

impl fmt::Display for JsonEncodedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for JsonEncodedError {}

/// Extension trait for normalizing `Result<T, JsValue>` into a Rust error that preserves JS info.
pub trait JsResultExt<T> {
    /// Converts `Result<T, JsValue>` into `Result<T, JsonEncodedError>`.
    ///
    /// - Panics when the `JsValue` is a well-known precondition violation (e.g., `TypeError`).
    /// - Otherwise, returns a `JsonEncodedError` encoding the JavaScript error as JSON.
    fn normalized(self) -> Result<T, JsonEncodedError>;
}

impl<T> JsResultExt<T> for Result<T, JsValue> {
    fn normalized(self) -> Result<T, JsonEncodedError> {
        match self {
            Ok(value) => Ok(value),
            Err(js_error) => {
                // Panic on precondition violations (e.g., TypeError)
                if is_precondition_violation(&js_error) {
                    let details: JsonValue = serde_wasm_bindgen::from_value(js_error.clone())
                        .expect("JsValue to serde_json::Value conversion always succeeds");
                    panic!("JavaScript precondition violation: {}", details);
                }

                // Otherwise, convert into a JsonEncodedError to preserve error details
                Err(JsonEncodedError(
                    serde_wasm_bindgen::from_value(js_error)
                        .expect("JsValue to serde_json::Value conversion always succeeds"),
                ))
            }
        }
    }
}

fn is_precondition_violation(js_error: &JsValue) -> bool {
    // Treat JavaScript TypeError as a precondition violation.
    // Additional exception types can be added here if needed.
    js_error.is_instance_of::<js_sys::TypeError>()
}
