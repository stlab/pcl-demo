# Target Switching Scripts

This directory contains utilities to easily switch between different target architectures for rust-analyzer in **VS Code** and **Emacs**.

## Scripts

### `switch-target.sh`
Switch rust-analyzer to a specific target architecture.

```bash
# Switch to web/wasm32 target (for web development)
./scripts/switch-target.sh web

# Switch to desktop target (native development)
./scripts/switch-target.sh desktop

# Switch to iOS target (mobile development)  
./scripts/switch-target.sh ios

# IDE-specific switching
./scripts/switch-target.sh web --ide=vscode    # VS Code only
./scripts/switch-target.sh web --ide=emacs     # Emacs only
./scripts/switch-target.sh web --ide=all       # Both IDEs (default)
```

### `show-target.sh`
Display the current rust-analyzer target configuration for both IDEs.

```bash
./scripts/show-target.sh
```

### `reload-rust-analyzer.sh`
Reload rust-analyzer after switching targets.

```bash
./scripts/reload-rust-analyzer.sh
./scripts/reload-rust-analyzer.sh --ide=emacs   # Emacs only
./scripts/reload-rust-analyzer.sh --ide=vscode  # VS Code only
```

## VS Code Integration

You can also switch targets using VS Code tasks:

1. Open Command Palette (`Cmd+Shift+P`)
2. Run "Tasks: Run Task"
3. Select one of:
   - "Switch to Web/WASM32 Target"
   - "Switch to Desktop Target" 
   - "Switch to iOS Target"
   - "Show Current Target"

## Emacs Integration

### Prerequisites
Ensure you have rust-analyzer working with Emacs using either:
- **lsp-mode** with `lsp-rust-analyzer` 
- **eglot** (built into Emacs 29+)

### Method 1: Shell Scripts (Recommended)
Use the same shell scripts as VS Code:

```bash
./scripts/switch-target.sh web --ide=emacs
```

### Method 2: Interactive Emacs Functions
Load the provided Emacs Lisp file for interactive target switching:

1. **Load the integration file:**
   ```elisp
   (load-file "scripts/emacs-target-switch.el")
   ```

2. **Use interactive functions:**
   - `M-x rust-target-switch-web` - Switch to web target
   - `M-x rust-target-switch-desktop` - Switch to desktop target  
   - `M-x rust-target-switch-ios` - Switch to iOS target
   - `M-x rust-target-show-current` - Show current target

3. **Optional keybindings** (add to your init.el):
   ```elisp
   (add-hook 'rust-mode-hook #'rust-target-switch-setup-keybindings)
   ```
   - `C-c t w` - Switch to web target
   - `C-c t d` - Switch to desktop target
   - `C-c t i` - Switch to iOS target
   - `C-c t s` - Show current target

### Method 3: Manual Configuration
Edit `.dir-locals.el` directly and reload with `C-x x g` or restart Emacs.

## Target Details

| Target | Architecture | Features | Use Case |
|--------|-------------|----------|----------|
| **web** | `wasm32-unknown-unknown` | `["web"]` | Web development with Dioxus |
| **desktop** | native (e.g., `aarch64-apple-darwin`) | `[]` | Desktop application development |
| **ios** | `aarch64-apple-ios-sim` | `["mobile"]` | iOS mobile development |

## Workflow

### VS Code Workflow
1. **Switch target**: `./scripts/switch-target.sh web`
2. **Reload rust-analyzer**: `Cmd+Shift+P` → "rust-analyzer: Reload Workspace"
3. **Start development**: rust-analyzer will now analyze code for the selected target

### Emacs Workflow
1. **Switch target**: `./scripts/switch-target.sh web --ide=emacs`
2. **Reload configuration**: `M-x lsp-restart-workspace` or `C-x x g` to reload dir-locals
3. **Start development**: rust-analyzer will now analyze code for the selected target

## Configuration Files

The scripts automatically manage these configuration files:

### VS Code (`.vscode/settings.json`)
- `rust-analyzer.cargo.target` - Target architecture
- `rust-analyzer.cargo.features` - Enabled features
- `rust-analyzer.check.targets` - Check targets
- All necessary rust-analyzer settings

### Emacs (`.dir-locals.el`)
- `lsp-rust-analyzer-cargo-target` - Target architecture  
- `lsp-rust-analyzer-cargo-features` - Enabled features
- `lsp-rust-analyzer-cargo-watch-args` - Watch arguments
- LSP-specific rust-analyzer settings

## Benefits

- **Cross-IDE Support**: Works with both VS Code and Emacs
- **Accurate IntelliSense**: Conditional compilation (`#[cfg(target_arch = "wasm32")]`) works correctly
- **Proper Dependencies**: Target-specific deps like `web-sys` are recognized
- **Feature Gates**: Dioxus features (`web`, `mobile`) are properly enabled
- **Error Checking**: No false errors from inactive target code
- **Fast Switching**: Takes seconds to switch between targets