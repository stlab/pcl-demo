# Target Switching Scripts

This directory contains utilities to easily switch between different target architectures for rust-analyzer.

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
```

### `show-target.sh`
Display the current rust-analyzer target configuration.

```bash
./scripts/show-target.sh
```

### `reload-rust-analyzer.sh`
Reload rust-analyzer after switching targets.

```bash
./scripts/reload-rust-analyzer.sh
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

## Target Details

| Target | Architecture | Features | Use Case |
|--------|-------------|----------|----------|
| **web** | `wasm32-unknown-unknown` | `["web"]` | Web development with Dioxus |
| **desktop** | native (e.g., `aarch64-apple-darwin`) | `[]` | Desktop application development |
| **ios** | `aarch64-apple-ios-sim` | `["mobile"]` | iOS mobile development |

## Workflow

1. **Switch target**: `./scripts/switch-target.sh web`
2. **Reload rust-analyzer**: `Cmd+Shift+P` → "rust-analyzer: Reload Workspace"
3. **Start development**: rust-analyzer will now analyze code for the selected target

The scripts automatically configure:
- Target architecture
- Conditional compilation features
- Check and build targets
- All necessary rust-analyzer settings