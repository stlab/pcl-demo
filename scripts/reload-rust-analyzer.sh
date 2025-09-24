#!/bin/bash

# Reload rust-analyzer via VS Code CLI
# Usage: ./reload-rust-analyzer.sh

echo "🔄 Reloading rust-analyzer..."

# Try to reload rust-analyzer using VS Code CLI if available
if command -v code &> /dev/null; then
    # Use VS Code CLI to execute the reload command
    code --command "rust-analyzer.reload"
    echo "✅ Sent reload command to rust-analyzer"
else
    echo "💡 VS Code CLI not found. Please manually reload rust-analyzer:"
    echo "   Open Command Palette (Cmd+Shift+P)"
    echo "   Run: 'rust-analyzer: Reload Workspace'"
fi

echo ""
echo "🎯 Current target configuration:"
./scripts/show-target.sh