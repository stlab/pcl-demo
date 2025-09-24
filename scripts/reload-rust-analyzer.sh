#!/bin/bash

# Reload rust-analyzer for VS Code and/or Emacs
# Usage: ./reload-rust-analyzer.sh [--ide=vscode|emacs|all]

IDE="all"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --ide=*)
            IDE="${1#*=}"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

echo "🔄 Reloading rust-analyzer..."

# Reload VS Code
if [ "$IDE" = "vscode" ] || [ "$IDE" = "all" ]; then
    echo ""
    echo "🔧 VS Code reload options:"
    if command -v code &> /dev/null; then
        code --command "rust-analyzer.reload"
        echo "✅ Sent reload command to VS Code rust-analyzer"
    else
        echo "💡 VS Code CLI not found. Please manually reload rust-analyzer:"
        echo "   Open Command Palette (Cmd+Shift+P)"
        echo "   Run: 'rust-analyzer: Reload Workspace'"
    fi
fi

# Reload Emacs
if [ "$IDE" = "emacs" ] || [ "$IDE" = "all" ]; then
    echo ""
    echo "📝 Emacs reload options:"
    echo "💡 Please reload rust-analyzer in Emacs:"
    echo "   M-x lsp-restart-workspace"
    echo "   or revert buffer to reload dir-locals: C-x x g"
    echo "   or restart Emacs to ensure dir-locals are applied"
    
    # Try to send command to running Emacs if emacsclient is available
    if command -v emacsclient &> /dev/null; then
        echo ""
        echo "🔄 Attempting to restart LSP workspace via emacsclient..."
        if emacsclient -e "(when (fboundp 'lsp-restart-workspace) (lsp-restart-workspace))" 2>/dev/null; then
            echo "✅ Sent restart command to Emacs"
        else
            echo "⚠️  Could not send command to Emacs (no running server or lsp-mode not loaded)"
        fi
    fi
fi

echo ""
echo "🎯 Current target configuration:"
./scripts/show-target.sh