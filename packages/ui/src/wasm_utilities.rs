//! Utilities for handling WebAssembly/JavaScript errors.
//!
//! This module provides utilities for converting JavaScript errors into Rust Result types
//! that are compatible with anyhow, while properly handling precondition violations.

use std::fmt;
use wasm_bindgen::{JsCast, JsValue};

/// An error type that wraps a JSON-encoded JavaScript error.
///
/// This type is compatible with anyhow and preserves the information from JavaScript errors.
#[derive(Debug)]
pub struct JsonEncodedError(serde_json::Value);

impl fmt::Display for JsonEncodedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for JsonEncodedError {}

/// Extension trait for normalizing `Result<T, JsValue>` into `Result<T, JsonEncodedError>`.
///
/// This trait provides the `normalized()` method which:
/// - Panics if the error is a TypeError or other precondition violation
/// - Converts other JavaScript errors into `JsonEncodedError` for proper error propagation
pub trait NormalizedJsResult<T> {
    /// Normalizes a JavaScript Result into a Rust Result with proper error handling.
    ///
    /// # Panics
    ///
    /// Panics if the error indicates a precondition violation (e.g., TypeError).
    fn normalized(self) -> Result<T, JsonEncodedError>;
}

impl<T> NormalizedJsResult<T> for Result<T, JsValue> {
    fn normalized(self) -> Result<T, JsonEncodedError> {
        match self {
            Ok(value) => Ok(value),
            Err(js_value) => {
                // Check if this is a TypeError or other precondition violation
                if let Some(error) = js_value.dyn_ref::<web_sys::js_sys::TypeError>() {
                    panic!("JavaScript TypeError (precondition violation): {:?}", error);
                }
                
                // Check for other error types that indicate precondition violations
                if let Some(name) = js_sys::Reflect::get(&js_value, &JsValue::from_str("name"))
                    .ok()
                    .and_then(|v| v.as_string())
                {
                    if name == "TypeError" || name == "ReferenceError" {
                        panic!("JavaScript {} (precondition violation): {:?}", name, js_value);
                    }
                }
                
                // For other errors, convert to JsonEncodedError
                let error_as_jsonencodederror = JsonEncodedError(
                    serde_wasm_bindgen::from_value(js_value)
                        .expect("JsValue to serde_json::Value conversion always succeeds"),
                );
                Err(error_as_jsonencodederror)
            }
        }
    }
}
