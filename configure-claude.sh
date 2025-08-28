#!/bin/bash

# Script to configure the Claude Diary MCP server with Claude Code

echo "🔧 Configuring Claude Diary MCP server (Rust implementation)..."

# Get the absolute path to the server directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_PATH="$SCRIPT_DIR/target/release/claude-diary-mcp-server"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the server if it doesn't exist
if [ ! -f "$SERVER_PATH" ]; then
    echo "🔨 Building Rust MCP server..."
    cd "$SCRIPT_DIR"
    cargo build --release
    
    if [ $? -ne 0 ]; then
        echo "❌ Failed to build the server"
        exit 1
    fi
fi

# Verify the built server exists
if [ ! -f "$SERVER_PATH" ]; then
    echo "❌ Server binary not found at $SERVER_PATH"
    echo "Please ensure 'cargo build --release' completed successfully"
    exit 1
fi

# Configure Claude Code to use this MCP server
echo "📝 Adding MCP server configuration to Claude Code..."

# Create MCP config directory if it doesn't exist
mkdir -p ~/.claude/mcp

# Write the MCP configuration
cat > ~/.claude/mcp/diary-server.json << EOF
{
  "mcpServers": {
    "diary": {
      "command": "$SERVER_PATH",
      "env": {}
    }
  }
}
EOF

echo "✅ Configuration complete!"
echo ""
echo "📋 To use the MCP server, run Claude Code with:"
echo "   claude --mcp-config ~/.claude/mcp/diary-server.json"
echo ""
echo "🚀 Performance benefits of Rust implementation:"
echo "   ⚡ Fast startup times"
echo "   🔒 Memory safety without garbage collection"
echo "   📦 Single statically-linked binary"
echo "   🚀 Concurrent query processing"
echo ""
echo "🛠 Available tools:"
echo "   - get_today_diary: Get today's diary entries"  
echo "   - get_yesterday_diary: Get yesterday's diary entries"
echo "   - get_diary_by_date: Get entries for a specific date (YYYY-MM-DD)"
echo "   - get_recent_sessions: Get recent diary sessions (configurable limit)"
echo ""
echo "💡 Example usage in Claude Code:"
echo '   "Use the get_today_diary tool to show what I worked on today"'
echo '   "Use get_diary_by_date with date 2025-08-25 to see what I did that day"'
echo ""
echo "🔧 To rebuild the server manually:"
echo "   cd $SCRIPT_DIR && cargo build --release"