use dioxus::prelude::*;
use crate::application_state::*;
use std::rc::Rc;

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Url, HtmlAnchorElement};



const FILE_MENU_CSS: Asset = asset!("/assets/styling/file_menu.css");

/// File menu component that provides file operations for web app
#[component]
pub fn FileMenu(application_state: Signal<ApplicationState>) -> Element {
    
    let mut state = application_state;
    let mut file_input_ref = use_signal(|| None::<web_sys::HtmlInputElement>);
    
    let handle_new = move |_| {
        state.write().new_document();
    };
    
    let handle_open = move |_| {
        if let Some(input) = file_input_ref.read().as_ref() {
            input.click();
        }
    };
    
    let handle_save = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
            let filename = current_state.current_file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("document.json");
            download_file(&json_content, filename);
        }
    };
    
    let handle_save_as = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
            download_file(&json_content, "document.json");
        }
    };
    
    let handle_file_input_mounted = move |element: MountedEvent| {
        if let Some(web_element) = element.downcast::<web_sys::Element>() {
            if let Ok(input) = web_element.clone().dyn_into::<web_sys::HtmlInputElement>() {
                *file_input_ref.write() = Some(input);
            }
        }
    };
    
    let handle_file_change = move |_event| {
        if let Some(file) = file_input_ref.read()
            .as_ref()
            .and_then(|input| input.files())
            .and_then(|files| files.get(0))
        {
            web_sys::console::log_1(&format!("Selected file: {}", file.name()).into());

            let file_reader = web_sys::FileReader::new().unwrap();
            let mut state_clone = state.clone();

            // We need to get a reference to the file_reader for the closure, but we can use a raw pointer
            // and 'move' the file_reader into the closure, then forget the closure to keep it alive.
            let file_reader_ptr = file_reader.clone();

            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(result) = file_reader_ptr.result() {
                    if let Some(text) = result.as_string() {
                        web_sys::console::log_1(&format!("File content read: {} chars", text.len()).into());

                        match serde_json::from_str::<crate::Document>(&text) {
                            Ok(document) => {
                                web_sys::console::log_1(&"Successfully parsed document".into());
                                state_clone.write().the_only_document = document;
                                state_clone.write().current_file_path = None;
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Parse error: {}", e).into());
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            file_reader.read_as_text(&file).unwrap();
            onload.forget();
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: FILE_MENU_CSS }
        div {
            class: "file-menu",
            div {
                class: "menu-bar",
                span { class: "menu-title", "File" }
                div {
                    class: "menu-buttons",
                    button {
                        class: "menu-button",
                        title: "New document (Ctrl+N)",
                        onclick: handle_new,
                        "New"
                    }
                    
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
                    
                    button {
                        class: "menu-button",
                        title: "Save document (Ctrl+S)", 
                        onclick: handle_save,
                        "Save"
                    }
                    button {
                        class: "menu-button",
                        title: "Save document as... (Ctrl+Shift+S)",
                        onclick: handle_save_as,
                        "Save As"
                    }
                }
            }
        }
    }
}

// Browser API functions for file operations



/// Downloads a file with the given content and filename
fn download_file(content: &str, filename: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(content));
    
    let blob = Blob::new_with_str_sequence(&array).unwrap();
    
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    
    anchor.set_href(&url);
    anchor.set_download(filename);
    let anchor_element: &web_sys::Element = anchor.as_ref();
    anchor_element.set_attribute("style", "display: none").unwrap();
    
    // Append, click, and remove
    document.body().unwrap().append_child(&anchor).unwrap();
    anchor.click();
    document.body().unwrap().remove_child(&anchor).unwrap();
    
    // Clean up the object URL
    Url::revoke_object_url(&url).unwrap();
}
