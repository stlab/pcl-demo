use crate::application_state::ApplicationState;
use crate::wasm_utilities::NormalizedJsResult;
use crate::Document;
use dioxus::prelude::*;

// Web API imports (available on all platforms for development ease)
use js_sys::Array;
use serde_json::{from_str, to_string_pretty};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console::log_1, window, Blob, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

/// Menu button for creating a new document.
#[component]
fn NewButton(mut state: Signal<ApplicationState>) -> Element {
    let handle_click = move |_| {
        state.write().new_document();
    };

    rsx! {
        button {
            class: "menu-button",
            title: "New document (Ctrl+N)",
            onclick: handle_click,
            "New"
        }
    }
}

/// Hidden file input and open button for loading documents.
#[component]
fn OpenButton(mut state: Signal<ApplicationState>) -> Element {
    let mut file_input_ref = use_signal(|| None::<HtmlInputElement>);
    let mut onload_closure = use_signal(|| None::<Closure<dyn FnMut(web_sys::Event)>>);

    let handle_open = move |_| {
        if let Some(input) = file_input_ref.read().as_ref() {
            input.click();
        }
    };

    let handle_file_input_mounted = move |element: MountedEvent| {
        if let Some(web_element) = element.downcast::<web_sys::Element>() {
            match web_element.clone().dyn_into::<HtmlInputElement>() {
                Ok(input) => {
                    *file_input_ref.write() = Some(input);
                }
                Err(e) => {
                    eprintln!("Failed to cast element to HtmlInputElement: {e:?}");
                }
            }
        }
    };

    let handle_file_change = move |_event| {
        if let Some(file) = file_input_ref
            .read()
            .as_ref()
            .and_then(|input| input.files())
            .and_then(|files| files.get(0))
        {
            log_1(&format!("Selected file: {}", file.name()).into());

            let file_reader = match FileReader::new() {
                Ok(reader) => reader,
                Err(_) => {
                    eprintln!("Failed to create FileReader - browser API unavailable");
                    return;
                }
            };
            let mut state_clone = state;
            let file_reader_clone = file_reader.clone();

            let onload = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                if let Ok(result) = file_reader_clone.result() {
                    if let Some(text) = result.as_string() {
                        log_1(&format!("File content read: {} chars", text.len()).into());

                        match from_str::<Document>(&text) {
                            Ok(document) => {
                                log_1(&"Successfully parsed document".into());
                                state_clone.write().the_only_document = document;
                                state_clone.write().current_file_path = None;
                            }
                            Err(e) => {
                                eprintln!("Parse error: {e}");
                            }
                        }
                    }
                }
            });

            file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            if let Err(_) = file_reader.read_as_text(&file) {
                eprintln!("Failed to read file as text");
                return;
            }

            *onload_closure.write() = Some(onload);
        }
    };

    rsx! {
        input {
            r#type: "file",
            accept: ".json",
            style: "display: none",
            id: "file-input-hidden",
            onmounted: handle_file_input_mounted,
            onchange: handle_file_change
        }

        button {
            class: "menu-button",
            title: "Open document (Ctrl+O)",
            onclick: handle_open,
            "Open"
        }
    }
}

/// Menu button for saving the current document.
#[component]
fn SaveButton(state: Signal<ApplicationState>) -> Element {
    let handle_click = move |_| {
        let current_state = state.read();
        match to_string_pretty(&current_state.the_only_document) {
            Ok(json_content) => {
                let filename = current_state
                    .current_file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("document.json");
                if let Err(e) = download_file(&json_content, filename) {
                    eprintln!("Failed to download file for save: {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize document for save: {e}");
            }
        }
    };

    rsx! {
        button {
            class: "menu-button",
            title: "Save document (Ctrl+S)",
            onclick: handle_click,
            "Save"
        }
    }
}

/// Menu button for saving the document with a new name.
#[component]
fn SaveAsButton(state: Signal<ApplicationState>) -> Element {
    let handle_click = move |_| match to_string_pretty(&state.read().the_only_document) {
        Ok(json_content) => {
            if let Err(e) = download_file(&json_content, "document.json") {
                eprintln!("Failed to download file for save as: {e:?}");
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize document for save as: {e}");
        }
    };

    rsx! {
        button {
            class: "menu-button",
            title: "Save document as... (Ctrl+Shift+S)",
            onclick: handle_click,
            "Save As"
        }
    }
}

/// The web app's file menu.
#[component]
pub fn WebFileMenu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/file_menu.css") }
        div {
            class: "file-menu",
            div {
                class: "menu-bar",
                span { class: "menu-title", "File" }
                div {
                    class: "menu-buttons",
                    NewButton { state: application_state }
                    OpenButton { state: application_state }
                    SaveButton { state: application_state }
                    SaveAsButton { state: application_state }
                }
            }
        }
    }
}

// Browser API functions for file operations

/// Saves a file called `filename` containing `content`.
fn download_file(content: &str, filename: &str) -> anyhow::Result<()> {
    let document = window()
        .ok_or_else(|| anyhow::anyhow!("Failed to get window object - browser API unavailable"))?
        .document()
        .ok_or_else(|| anyhow::anyhow!("Failed to get document object - browser API unavailable"))?;

    let array = Array::new();
    array.push(&JsValue::from_str(content));

    let blob = Blob::new_with_str_sequence(&array).normalized()?;
    let url = Url::create_object_url_with_blob(&blob).normalized()?;
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .normalized()?
        .dyn_into()
        .map_err(|_| anyhow::anyhow!("Failed to cast element to HtmlAnchorElement"))?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    let anchor_element: &web_sys::Element = anchor.as_ref();
    anchor_element
        .set_attribute("style", "display: none")
        .normalized()?;

    let body = document
        .body()
        .ok_or_else(|| anyhow::anyhow!("Failed to get document body"))?;

    // Append, click, and remove
    body.append_child(&anchor).normalized()?;
    anchor.click();
    body.remove_child(&anchor).normalized()?;

    // Clean up the object URL
    Url::revoke_object_url(&url).normalized()?;

    Ok(())
}
