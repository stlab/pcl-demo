#!/bin/bash

# Target switching script for rust-analyzer
# Usage: ./switch-target.sh [web|desktop|ios] [--ide=vscode|emacs|all]

set -e

VSCODE_SETTINGS_FILE=".vscode/settings.json"
EMACS_DIR_LOCALS_FILE=".dir-locals.el"
TARGET="$1"
IDE="all"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --ide=*)
            IDE="${1#*=}"
            shift
            ;;
        web|desktop|ios)
            TARGET="$1"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

if [ -z "$TARGET" ]; then
    echo "Usage: $0 [web|desktop|ios] [--ide=vscode|emacs|all]"
    echo "Available targets:"
    echo "  web     - wasm32-unknown-unknown (for web development)"
    echo "  desktop - native target (for desktop development)"
    echo "  ios     - aarch64-apple-ios-sim (for iOS development)"
    echo ""
    echo "IDE options:"
    echo "  --ide=vscode  - Update VS Code settings only"
    echo "  --ide=emacs   - Update Emacs dir-locals only"
    echo "  --ide=all     - Update both (default)"
    exit 1
fi

# Backup current settings
if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
    [ -f "$VSCODE_SETTINGS_FILE" ] && cp "$VSCODE_SETTINGS_FILE" "$VSCODE_SETTINGS_FILE.bak"
fi

if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
    [ -f "$EMACS_DIR_LOCALS_FILE" ] && cp "$EMACS_DIR_LOCALS_FILE" "$EMACS_DIR_LOCALS_FILE.bak"
fi

case "$TARGET" in
    "web")
        echo "🌐 Switching to Web/WASM32 target..."
        
        # Update VS Code settings
        if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
            cat > "$VSCODE_SETTINGS_FILE" << 'EOF'
{
    "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
    "rust-analyzer.check.targets": ["wasm32-unknown-unknown"],
    "rust-analyzer.cargo.features": ["web"],
    "rust-analyzer.cargo.allTargets": false,
    "rust-analyzer.checkOnSave.targets": ["wasm32-unknown-unknown"],
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.workspace.symbol.search.scope": "workspace_and_dependencies"
}
EOF
        fi
        
        # Update Emacs dir-locals
        if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
            cat > "$EMACS_DIR_LOCALS_FILE" << 'EOF'
;;; Directory Local Variables for rust-analyzer target switching
;;; For more information see (info "(emacs) Directory Variables")

;; Current configuration: Web/WASM32 target
;; To switch targets, use ./scripts/switch-target.sh [web|desktop|ios]

((rust-mode . ((lsp-rust-analyzer-cargo-target . "wasm32-unknown-unknown")
               (lsp-rust-analyzer-check-command . "clippy")
               (lsp-rust-analyzer-cargo-watch-args . ["--target" "wasm32-unknown-unknown"])
               (lsp-rust-analyzer-cargo-all-targets . nil)
               (lsp-rust-analyzer-cargo-features . ["web"])
               (lsp-rust-analyzer-proc-macro-enable . t)
               (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))
EOF
        fi
        
        echo "✅ Switched to Web/WASM32 target"
        echo "📝 Features: web"
        echo "🎯 Target: wasm32-unknown-unknown"
        ;;
    
    "desktop")
        echo "🖥️  Switching to Desktop target..."
        
        # Update VS Code settings
        if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
            cat > "$VSCODE_SETTINGS_FILE" << 'EOF'
{
    "rust-analyzer.cargo.target": null,
    "rust-analyzer.check.targets": null,
    "rust-analyzer.cargo.features": [],
    "rust-analyzer.cargo.allTargets": true,
    "rust-analyzer.checkOnSave.targets": null,
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.workspace.symbol.search.scope": "workspace_and_dependencies"
}
EOF
        fi
        
        # Update Emacs dir-locals
        if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
            cat > "$EMACS_DIR_LOCALS_FILE" << 'EOF'
;;; Directory Local Variables for rust-analyzer target switching
;;; For more information see (info "(emacs) Directory Variables")

;; Current configuration: Desktop/Native target
;; To switch targets, use ./scripts/switch-target.sh [web|desktop|ios]

((rust-mode . ((lsp-rust-analyzer-cargo-target . nil)
               (lsp-rust-analyzer-check-command . "clippy")
               (lsp-rust-analyzer-cargo-watch-args . [])
               (lsp-rust-analyzer-cargo-all-targets . t)
               (lsp-rust-analyzer-cargo-features . [])
               (lsp-rust-analyzer-proc-macro-enable . t)
               (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))
EOF
        fi
        
        echo "✅ Switched to Desktop target"
        echo "📝 Features: default"
        echo "🎯 Target: native (aarch64-apple-darwin)"
        ;;
    
    "ios")
        echo "📱 Switching to iOS target..."
        
        # Update VS Code settings
        if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
            cat > "$VSCODE_SETTINGS_FILE" << 'EOF'
{
    "rust-analyzer.cargo.target": "aarch64-apple-ios-sim",
    "rust-analyzer.check.targets": ["aarch64-apple-ios-sim"],
    "rust-analyzer.cargo.features": ["mobile"],
    "rust-analyzer.cargo.allTargets": false,
    "rust-analyzer.checkOnSave.targets": ["aarch64-apple-ios-sim"],
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.workspace.symbol.search.scope": "workspace_and_dependencies"
}
EOF
        fi
        
        # Update Emacs dir-locals
        if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
            cat > "$EMACS_DIR_LOCALS_FILE" << 'EOF'
;;; Directory Local Variables for rust-analyzer target switching
;;; For more information see (info "(emacs) Directory Variables")

;; Current configuration: iOS target
;; To switch targets, use ./scripts/switch-target.sh [web|desktop|ios]

((rust-mode . ((lsp-rust-analyzer-cargo-target . "aarch64-apple-ios-sim")
               (lsp-rust-analyzer-check-command . "clippy")
               (lsp-rust-analyzer-cargo-watch-args . ["--target" "aarch64-apple-ios-sim"])
               (lsp-rust-analyzer-cargo-all-targets . nil)
               (lsp-rust-analyzer-cargo-features . ["mobile"])
               (lsp-rust-analyzer-proc-macro-enable . t)
               (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))
EOF
        fi
        
        echo "✅ Switched to iOS target"
        echo "📝 Features: mobile"
        echo "🎯 Target: aarch64-apple-ios-sim"
        ;;
    
    *)
        echo "❌ Unknown target: $TARGET"
        echo "Available targets: web, desktop, ios"
        exit 1
        ;;
esac

echo ""
if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
    echo "🔄 Please reload rust-analyzer in VS Code:"
    echo "   Cmd+Shift+P → 'rust-analyzer: Reload Workspace'"
fi

if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
    echo "🔄 Please reload rust-analyzer in Emacs:"
    echo "   M-x lsp-restart-workspace"
    echo "   or revert buffer to reload dir-locals: C-x x g"
fi

echo ""
echo "💡 Or run: ./scripts/reload-rust-analyzer.sh --ide=$IDE"