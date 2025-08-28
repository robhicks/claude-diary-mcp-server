#!/bin/bash

# Script to configure the Claude Diary MCP server with Claude Code

echo "🔧 Configuring Claude Diary MCP server..."

# Get the absolute path to the server
SERVER_PATH="/Users/robhicks/.claude/mcp-servers/diary-server/dist/index.js"

# Check if the built server exists
if [ ! -f "$SERVER_PATH" ]; then
    echo "❌ Server not found at $SERVER_PATH"
    echo "Please run 'npm run build' first"
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
      "command": "node",
      "args": ["$SERVER_PATH"],
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
echo "🛠 Available tools:"
echo "   - get_today_diary: Get today's diary entries"  
echo "   - get_yesterday_diary: Get yesterday's diary entries"
echo "   - get_diary_by_date: Get entries for a specific date"
echo "   - get_recent_sessions: Get recent diary sessions"
echo ""
echo "💡 Example usage in Claude Code:"
echo '   "Use the get_today_diary tool to show what I worked on today"'