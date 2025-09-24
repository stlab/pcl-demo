#!/bin/bash

# Show current rust-analyzer target configuration
# Usage: ./show-target.sh

VSCODE_SETTINGS_FILE=".vscode/settings.json"
EMACS_DIR_LOCALS_FILE=".dir-locals.el"

echo "📋 Current rust-analyzer configuration:"
echo ""

# Function to determine target from configuration
determine_target() {
    local target="$1"
    local features="$2"
    
    if [ "$target" = "null" ] || [ -z "$target" ] || [ "$target" = "nil" ]; then
        echo "🎯 Target: native (desktop)"
        echo "📝 Features: default"
        echo "💻 Platform: Desktop development"
    elif [ "$target" = "wasm32-unknown-unknown" ]; then
        echo "🎯 Target: wasm32-unknown-unknown"
        echo "📝 Features: $features"
        echo "🌐 Platform: Web development"
    elif [ "$target" = "aarch64-apple-ios-sim" ]; then
        echo "🎯 Target: aarch64-apple-ios-sim"
        echo "📝 Features: $features"
        echo "📱 Platform: iOS development"
    else
        echo "🎯 Target: $target"
        echo "📝 Features: $features"
        echo "❓ Platform: Custom target"
    fi
}

# Check VS Code configuration
if [ -f "$VSCODE_SETTINGS_FILE" ]; then
    echo "🔧 VS Code Configuration:"
    TARGET=$(grep -o '"rust-analyzer.cargo.target": "[^"]*"' "$VSCODE_SETTINGS_FILE" 2>/dev/null | cut -d'"' -f4)
    FEATURES=$(grep -o '"rust-analyzer.cargo.features": \[[^\]]*\]' "$VSCODE_SETTINGS_FILE" 2>/dev/null | cut -d'[' -f2 | cut -d']' -f1)
    determine_target "$TARGET" "$FEATURES"
    echo ""
fi

# Check Emacs configuration
if [ -f "$EMACS_DIR_LOCALS_FILE" ]; then
    echo "📝 Emacs Configuration:"
    TARGET=$(grep -o 'lsp-rust-analyzer-cargo-target[[:space:]]*\.[[:space:]]*"[^"]*"' "$EMACS_DIR_LOCALS_FILE" 2>/dev/null | cut -d'"' -f2)
    if [ -z "$TARGET" ]; then
        TARGET=$(grep -o 'lsp-rust-analyzer-cargo-target[[:space:]]*\.[[:space:]]*nil' "$EMACS_DIR_LOCALS_FILE" 2>/dev/null | grep -o 'nil')
    fi
    FEATURES=$(grep -o 'lsp-rust-analyzer-cargo-features[[:space:]]*\.[[:space:]]*\[[^\]]*\]' "$EMACS_DIR_LOCALS_FILE" 2>/dev/null | sed 's/.*\[\(.*\)\].*/\1/')
    determine_target "$TARGET" "$FEATURES"
    echo ""
fi

if [ ! -f "$VSCODE_SETTINGS_FILE" ] && [ ! -f "$EMACS_DIR_LOCALS_FILE" ]; then
    echo "❌ No IDE configuration files found"
    echo "   Expected: $VSCODE_SETTINGS_FILE or $EMACS_DIR_LOCALS_FILE"
    exit 1
fi

echo "🔧 To switch targets:"
echo "   ./scripts/switch-target.sh [web|desktop|ios] [--ide=vscode|emacs|all]"