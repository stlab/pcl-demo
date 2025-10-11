You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.7 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone

Provide concise code examples with detailed descriptions

# Dioxus Dependency

You can add Dioxus to your `Cargo.toml` like this:

```toml
[dependencies]
dioxus = { version = "0.7.0-alpha.3" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

# Launching your application

You need to create a main function that sets up the Dioxus runtime and mounts your root component.

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! { "Hello, Dioxus!" }
}
```

Then serve with `dx serve`:

```sh
curl -sSL http://dioxus.dev/install.sh | sh
dx serve
```

# UI with RSX

```rust
rsx! {
	div {
		class: "container", // Attribute
		color: "red", // Inline styles
		width: if condition { "100%" }, // Conditional attributes
		"Hello, Dioxus!"
	}
	// Prefer loops over iterators
	for i in 0..5 {
		div { "{i}" } // use elements or components directly in loops
	}
	if condition {
		div { "Condition is true!" } // use elements or components directly in conditionals
	}

	{children} // Expressions are wrapped in brace
	{(0..5).map(|i| rsx! { span { "Item {i}" } })} // Iterators must be wrapped in braces
}
```

# Assets

The asset macro can be used to link to local files to use in your project. All links start with `/` and are relative to the root of your project.

```rust
rsx! {
	img {
		src: asset!("/assets/image.png"),
		alt: "An image",
	}
}
```

## Styles

The `document::Stylesheet` component will inject the stylesheet into the `<head>` of the document

```rust
rsx! {
	document::Stylesheet {
		href: asset!("/assets/styles.css"),
	}
}
```

# Components

Components are the building blocks of apps

* Component are functions annotated with the `#[component]` macro.
* The function name must start with a capital letter or contain an underscore.
* A component re-renders only under two conditions:
	1.  Its props change (as determined by `PartialEq`).
	2.  An internal reactive state it depends on is updated.

```rust
#[component]
fn Input(mut value: Signal<String>) -> Element {
	rsx! {
		input {
            value,
			oninput: move |e| {
				*value.write() = e.value();
			},
			onkeydown: move |e| {
				if e.key() == Key::Enter {
					value.write().clear();
				}
			},
		}
	}
}
```

Each component accepts function arguments (props)

* Props must be owned values, not references. Use `String` and `Vec<T>` instead of `&str` or `&[T]`.
* Props must implement `PartialEq` and `Clone`.
* To make props reactive and copy, you can wrap the type in `ReadOnlySignal`. Any reactive state like memos and resources that read `ReadOnlySignal` props will automatically re-run when the prop changes.

# State

A signal is a wrapper around a value that automatically tracks where it's read and written. Changing a signal's value causes code that relies on the signal to rerun.

## Local State

The `use_signal` hook creates state that is local to a single component. You can call the signal like a function (e.g. `my_signal()`) to clone the value, or use `.read()` to get a reference. `.write()` gets a mutable reference to the value.

Use `use_memo` to create a memoized value that recalculates when its dependencies change. Memos are useful for expensive calculations that you don't want to repeat unnecessarily.

```rust
#[component]
fn Counter() -> Element {
	let mut count = use_signal(|| 0);
	let mut doubled = use_memo(move || count() * 2); // doubled will re-run when count changes because it reads the signal

	rsx! {
		h1 { "Count: {count}" } // Counter will re-render when count changes because it reads the signal
		h2 { "Doubled: {doubled}" }
		button {
			onclick: move |_| *count.write() += 1, // Writing to the signal rerenders Counter
			"Increment"
		}
		button {
			onclick: move |_| count.with_mut(|count| *count += 1), // use with_mut to mutate the signal
			"Increment with with_mut"
		}
	}
}
```

## Context API

The Context API allows you to share state down the component tree. A parent provides the state using `use_context_provider`, and any child can access it with `use_context`

```rust
#[component]
fn App() -> Element {
	let mut theme = use_signal(|| "light".to_string());
	use_context_provider(|| theme); // Provide a type to children
	rsx! { Child {} }
}

#[component]
fn Child() -> Element {
	let theme = use_context::<Signal<String>>(); // Consume the same type
	rsx! {
		div {
			"Current theme: {theme}"
		}
	}
}
```

# Async

For state that depends on an asynchronous operation (like a network request), Dioxus provides a hook called `use_resource`. This hook manages the lifecycle of the async task and provides the result to your component.

* The `use_resource` hook takes an `async` closure. It re-runs this closure whenever any signals it depends on (reads) are updated
* The `Resource` object returned can be in several states when read:
1. `None` if the resource is still loading
2. `Some(value)` if the resource has successfully loaded

```rust
let mut dog = use_resource(move || async move {
	// api request
});

match dog() {
	Some(dog_info) => rsx! { Dog { dog_info } },
	None => rsx! { "Loading..." },
}
```

# Routing

All possible routes are defined in a single Rust `enum` that derives `Routable`. Each variant represents a route and is annotated with `#[route("/path")]`. Dynamic Segments can capture parts of the URL path as parameters by using `:name` in the route string. These become fields in the enum variant.

The `Router<Route> {}` component is the entry point that manages rendering the correct component for the current URL.

You can use the `#[layout(NavBar)]` to create a layout shared between pages and place an `Outlet<Route> {}` inside your layout component. The child routes will be rendered in the outlet.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)] // This will use NavBar as the layout for all routes
		#[route("/")]
		Home {},
		#[route("/blog/:id")] // Dynamic segment
		BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
	rsx! {
		a { href: "/", "Home" }
		Outlet<Route> {} // Renders Home or BlogPost
	}
}

#[component]
fn App() -> Element {
	rsx! { Router::<Route> {} }
}
```

```toml
dioxus = { version = "0.7.0-alpha.3", features = ["router"] }
```

# Fullstack

Fullstack enables server rendering and ipc calls. It uses Cargo features (`server` and a client feature like `web`) to split the code into a server and client binaries.

```toml
dioxus = { version = "0.7.0-alpha.3", features = ["fullstack"] }
```

## Server Functions

Use the `#[server]` macro to define an `async` function that will only run on the server. On the server, this macro generates an API endpoint. On the client, it generates a function that makes an HTTP request to that endpoint.

```rust
#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	Ok(number * 2)
}
```

## Hydration

Hydration is the process of making a server-rendered HTML page interactive on the client. The server sends the initial HTML, and then the client-side runs, attaches event listeners, and takes control of future rendering.

### Errors
The initial UI rendered by the component on the client must be identical to the UI rendered on the server.

* Use the `use_server_future` hook instead of `use_resource`. It runs the future on the server, serializes the result, and sends it to the client, ensuring the client has the data immediately for its first render.
* Any code that relies on browser-specific APIs (like accessing `localStorage`) must be run *after* hydration. Place this code inside a `use_effect` hook.

## Methodology

Follow the guidance of the Better Code book, currently under development at https://github.com/stlab/better-code/tree/main/better-code/src.

### Naming

Avoid embedding type information (or anything else visible in the declaration) in names of non-types.  Never call an array `array`; find a name that describes a role.  If you can't find a descriptive name that is more than its type, a single-character name might be the best choice.

Adapt the naming philosphy outlined in the Swift API guidelines (https://www.swift.org/documentation/api-design-guidelines/) as necessary for Rust. Keep in mind that when Rust programmers read code, the names of function parameters are very often displayed at call sites as though they were swift argument labels.


### Documentation

Use the contract documentation methodology described in https://github.com/stlab/better-code/blob/main/better-code/src/chapter-2-contracts.md

Document each function or method using the style outlined in the Swift API guidelines (https://www.swift.org/documentation/api-design-guidelines/), preferring to capture everything in the summary if reasonable.  Describe results at the level of human semantics, without replicating the logic of the function, including any regular expressions it may use. Be as concise as possible but include all information necessary to determine if the implementation is correct.

Use the names of parameters in documentation comments to save words, e.g. instead of

```rust
    /// Saves the document to the specified path.
    pub fn save_to_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
```

write:

```rust
    /// Saves the document as `p`.
    pub fn save_to_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
```

**Key documentation conventions:**

* Functions that **return** values should be documented as "Returns xxx", not "Creates xxx":
  ```rust
  /// Returns an accelerator triggered by `key` with `base` modifier.
  pub fn menu_key(&self, key: Code) -> Option<Accelerator>
  ```

* Functions with **side effects** should describe the action, not just the return value:
  ```rust
  /// Presents an open file dialog and returns the user's selection (or `None` if canceled).
  pub fn file_from_open_dialog() -> Option<std::path::PathBuf>
  ```

* Use precise names that reflect semantics:
  - `file_from_open_dialog()` - returns an existing file
  - `path_from_save_dialog()` - returns a path (may not exist yet)

* Don't use function names starting with `get_`. Functions that return values should have simple, direct names:
  ```rust
  // BAD: Unnecessary get_ prefix
  fn get_saved_files() -> Vec<String> { ... }
  fn get_file_size(filename: &str) -> usize { ... }

  // GOOD: Direct, clear names
  fn saved_files() -> Vec<String> { ... }
  fn file_size(filename: &str) -> usize { ... }
  ```

* Avoid repeating information (including type information) that's visible in the declaration of the thing being documented.  For example, Documentation beginning "Extension trait…", "Blanket implementation…", or "Method…" is right out.

### Code Organization

* Avoid unnecessary constants for single-use values.

* Keep “interesting” code out of format strings, which are opaque to rust-analyzer.  Being unable, in the IDE, to get type information on the expressions in `format!("{file_size(&filename).unwrap_or(0)}")` is inconvenient.

* Avoid implementing `Default` trait when it just calls `new()` - it adds no value

* Avoid creating "namespace structs" - empty structs that only serve as containers for static methods. Use standalone functions or proper abstractions instead:
  ```rust
  // BAD: Fake abstraction - just a namespace
  pub struct PlatformDialogs;
  impl PlatformDialogs {
      pub fn file_from_open_dialog() -> Option<PathBuf> { ... }
  }

  // GOOD: Standalone function
  pub fn file_from_open_dialog() -> Option<PathBuf> { ... }
  ```

* Only create structs/types when they represent actual abstractions with:
  - State that needs to be maintained
  - Behavior that operates on that state
  - Multiple implementations (traits)
  - Clear lifetime or ownership semantics

* Use specific imports to eliminate explicit qualification:
  ```rust
  // BAD: Explicit qualification throughout code
  use std::path;
  fn open() -> Option<path::PathBuf> { ... }

  // GOOD: Import specific types and use them directly
  use std::path::PathBuf;
  fn open() -> Option<PathBuf> { ... }
  ```

  ```rust
  // BAD: Module qualification in calling code
  mod platform;
  fn main() {
      let menu = platform::create_menu_bar();
      let file = platform::file_from_open_dialog();
  }

  // GOOD: Import specific functions
  mod platform;
  use platform::{create_menu_bar, file_from_open_dialog};
  fn main() {
      let menu = create_menu_bar();
      let file = file_from_open_dialog();
  }
  ```

### Testing

You can test that the code builds with `dx build --package desktop`.  Because we have no conditional compilation there's no need to build other targets to check for compile errors.

### Running the Web app

When running `dx serve` for the web app, always specify `--port 8765`.

## Overall Goal

You are developing a demo application for Project Code Less, whose
mission is to show that applications can be built with two orders of
magnitude less code by using more declarative forms and
domain-specific languages.

Our demo is going to be a vector graphic editor akin to Adobe Illustrator.
