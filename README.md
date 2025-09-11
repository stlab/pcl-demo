# Set up

This project requires dioxus version = "0.7.0-rc.0" and additional Rust targets for cross-platform development.

## Install Dioxus CLI

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall dioxus-cli@0.7.0-rc.0 --force
```

## Install Required Rust Targets

For iOS development (required for `dx serve --package mobile --platform ios`):

```bash
rustup target add aarch64-apple-ios-sim    # iOS Simulator (Apple Silicon)
rustup target add aarch64-apple-ios        # iOS Device (Apple Silicon)
```

For Android development (if needed in the future):

```bash
rustup target add aarch64-linux-android    # Android ARM64
rustup target add armv7-linux-androideabi  # Android ARM32
```

## Platform Requirements

- **macOS**: Required for iOS development and simulator
- **Web**: Works on all platforms, no additional requirements
- **Desktop**: Works on all platforms (Windows, macOS, Linux)

# Running the app

From the top-level directory:

```bash
dx serve --package <package-name>
```

The `mobile` package also lets you specify a platform:
```bash
# Run on desktop for testing (default)
dx serve --package mobile --platform desktop

# Run in iOS Simulator (macOS only)
dx serve --package mobile --platform ios
```

You can find sample documents to open in `mobile_documents/` at the
root of this repository.

# Development

This workspace contains a member crate for each of the web, desktop and mobile platforms, and a `ui` crate for components that are shared between multiple platforms:

```
your_project/
├─ README.md
├─ Cargo.toml
├─ mobile_documents/        # Mobile app persistent storage directory
│  ├─ sample_circle.json    # Sample documents
│  ├─ sample_square.json
│  └─ *.json               # User-saved documents
└─ packages/
   ├─ web/
   │  └─ ... # Web specific UI/logic
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic
   └─  ui/
      ├─ src/
      │  ├─ file_menu.rs           # Web file menu component
      │  ├─ mobile_file_menu.rs    # Mobile file menu component
      │  └─ ... # Other shared components
      └─ assets/styling/
         ├─ file_menu.css          # Web file menu styles
         └─ mobile_file_menu.css   # Mobile file menu styles
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app.
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
```
