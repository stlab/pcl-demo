#!/bin/bash

# Target switching script for rust-analyzer
# Usage: ./switch-target.sh [web|desktop|ios]

set -e

VSCODE_SETTINGS_FILE=".vscode/settings.json"
TARGET="$1"

if [ -z "$TARGET" ]; then
    echo "Usage: $0 [web|desktop|ios]"
    echo "Available targets:"
    echo "  web     - wasm32-unknown-unknown (for web development)"
    echo "  desktop - native target (for desktop development)"
    echo "  ios     - aarch64-apple-ios-sim (for iOS development)"
    exit 1
fi

# Backup current settings
cp "$VSCODE_SETTINGS_FILE" "$VSCODE_SETTINGS_FILE.bak"

case "$TARGET" in
    "web")
        echo "🌐 Switching to Web/WASM32 target..."
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
        echo "✅ Switched to Web/WASM32 target"
        echo "📝 Features: web"
        echo "🎯 Target: wasm32-unknown-unknown"
        ;;
    
    "desktop")
        echo "🖥️  Switching to Desktop target..."
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
        echo "✅ Switched to Desktop target"
        echo "📝 Features: default"
        echo "🎯 Target: native (aarch64-apple-darwin)"
        ;;
    
    "ios")
        echo "📱 Switching to iOS target..."
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
echo "🔄 Please reload rust-analyzer in VS Code:"
echo "   Cmd+Shift+P → 'rust-analyzer: Reload Workspace'"
echo ""
echo "💡 Or run: ./scripts/reload-rust-analyzer.sh"