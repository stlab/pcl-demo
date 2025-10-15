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

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Helper to produce a generic JS Error as JsValue.
    fn make_js_error(message: &str) -> JsValue { js_sys::Error::new(message).into() }

    // Helper to produce a TypeError as JsValue.
    fn make_type_error(message: &str) -> JsValue { js_sys::TypeError::new(message).into() }

    #[wasm_bindgen_test]
    fn normalized_ok_passthrough() {
        let ok: Result<i32, JsValue> = Ok(42);
        let result = JsResultExt::normalized(ok);
        assert_eq!(result.unwrap(), 42);
    }

    #[wasm_bindgen_test]
    fn normalized_converts_non_precondition_error() {
        let err: Result<i32, JsValue> = Err(make_js_error("something went wrong"));
        let result = JsResultExt::normalized(err);
        match result {
            Ok(_) => panic!("expected error"),
            Err(e) => {
                // The JSON should be an object with at least a message field.
                // Exact shape may vary by environment; check it stringifies.
                let s = e.to_string();
                assert!(s.contains("something went wrong"));
            }
        }
    }

    #[wasm_bindgen_test]
    #[should_panic]
    fn normalized_panics_on_typeerror() {
        let err: Result<i32, JsValue> = Err(make_type_error("bad arg"));
        // should panic due to precondition violation
        let _ = JsResultExt::normalized(err);
    }
}
