use dioxus::prelude::*;
use crate::application_state::*;
use std::rc::Rc;

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Url, HtmlAnchorElement, HtmlInputElement, FileReader, Event};



const FILE_MENU_CSS: Asset = asset!("/assets/styling/file_menu.css");

/// File menu component that provides file operations for web app
#[component]
pub fn FileMenu(application_state: Signal<ApplicationState>) -> Element {
    
    let mut state = application_state;
    let mut file_input_ref = use_signal(|| None::<web_sys::HtmlInputElement>);
    
    // Handler for creating a new document
    let handle_new = move |_| {
        state.write().new_document();
    };
    
    // Handler for opening a file
    let handle_open = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(input) = file_input_ref.read().as_ref() {
                input.click();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // On native platforms, this would show a native file dialog
            state.write().new_document();
        }
    };
    
    // Handler for saving the current document
    let handle_save = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let current_state = state.read();
            if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
                let filename = current_state.current_file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("document.json");
                download_file(&json_content, filename);
            }
        }
    };
    
    // Handler for save as
    let handle_save_as = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let current_state = state.read();
            if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
                download_file(&json_content, "document.json");
            }
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
                    
                    // Hidden file input for opening files
                    input {
                        r#type: "file",
                        accept: ".json",
                        style: "display: none",
                        id: "file-input-hidden",
                        onmounted: move |element| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(web_element) = element.downcast::<web_sys::Element>() {
                                    if let Ok(input) = web_element.clone().dyn_into::<web_sys::HtmlInputElement>() {
                                        *file_input_ref.write() = Some(input);
                                    }
                                }
                            }
                        },
                        onchange: move |_event| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(input) = file_input_ref.read().as_ref() {
                                    if let Some(files) = input.files() {
                                        if files.length() > 0 {
                                            if let Some(file) = files.get(0) {
                                                web_sys::console::log_1(&format!("Selected file: {}", file.name()).into());
                                                
                                                let file_reader = web_sys::FileReader::new().unwrap();
                                                let mut state_clone = state.clone();
                                                
                                                // Use Rc to share the file_reader between the closure and the setup
                                                let file_reader_rc = Rc::new(file_reader);
                                                let file_reader_for_closure = file_reader_rc.clone();
                                                
                                                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
                                                    if let Ok(result) = file_reader_for_closure.result() {
                                                        if let Some(text) = result.as_string() {
                                                            web_sys::console::log_1(&format!("File content read: {} chars", text.len()).into());
                                                            
                                                            // Try to parse the content as JSON
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
                                                
                                                file_reader_rc.set_onload(Some(onload.as_ref().unchecked_ref()));
                                                file_reader_rc.read_as_text(&file).unwrap();
                                                onload.forget();
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
#[cfg(target_arch = "wasm32")]
fn download_file(content: &str, filename: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // Create a blob with the content
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(content));
    
    let blob = Blob::new_with_str_sequence(&array).unwrap();
    
    // Create a download link
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
