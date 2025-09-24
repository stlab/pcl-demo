#!/bin/bash

# Show current rust-analyzer target configuration
# Usage: ./show-target.sh

VSCODE_SETTINGS_FILE=".vscode/settings.json"

if [ ! -f "$VSCODE_SETTINGS_FILE" ]; then
    echo "❌ No VS Code settings file found"
    exit 1
fi

echo "📋 Current rust-analyzer configuration:"
echo ""

# Extract current target
TARGET=$(grep -o '"rust-analyzer.cargo.target": "[^"]*"' "$VSCODE_SETTINGS_FILE" 2>/dev/null | cut -d'"' -f4)
FEATURES=$(grep -o '"rust-analyzer.cargo.features": \[[^\]]*\]' "$VSCODE_SETTINGS_FILE" 2>/dev/null | cut -d'[' -f2 | cut -d']' -f1)

if [ "$TARGET" = "null" ] || [ -z "$TARGET" ]; then
    echo "🎯 Target: native (desktop)"
    echo "📝 Features: default"
    echo "💻 Platform: Desktop development"
elif [ "$TARGET" = "wasm32-unknown-unknown" ]; then
    echo "🎯 Target: wasm32-unknown-unknown"
    echo "📝 Features: $FEATURES"
    echo "🌐 Platform: Web development"
elif [ "$TARGET" = "aarch64-apple-ios-sim" ]; then
    echo "🎯 Target: aarch64-apple-ios-sim"
    echo "📝 Features: $FEATURES"
    echo "📱 Platform: iOS development"
else
    echo "🎯 Target: $TARGET"
    echo "📝 Features: $FEATURES"
    echo "❓ Platform: Custom target"
fi

echo ""
echo "🔧 To switch targets:"
echo "   ./scripts/switch-target.sh [web|desktop|ios]"