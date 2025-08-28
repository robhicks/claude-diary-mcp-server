# Claude Diary MCP Server

A fast, memory-safe MCP (Model Context Protocol) server written in Rust that provides tools to query your Claude diary entries stored in SQLite.

## Features

- **get_today_diary**: Get diary entries for today
- **get_yesterday_diary**: Get diary entries for yesterday  
- **get_diary_by_date**: Get diary entries for a specific date (YYYY-MM-DD format)
- **get_recent_sessions**: Get recent diary sessions (configurable limit)

## Prerequisites

- [Rust](https://rustup.rs/) (1.75 or later)
- SQLite database with diary entries (created by the Claude diary hook)

## Setup

1. **Clone and build**:
   ```bash
   git clone https://github.com/robhicks/claude-diary-mcp-server
   cd claude-diary-mcp-server
   cargo build --release
   ```

2. **Configure Claude Code**:
   Add this MCP server to your Claude Code configuration:
   ```json
   {
     "mcpServers": {
       "diary": {
         "command": "/path/to/claude-diary-mcp-server/target/release/claude-diary-mcp-server"
       }
     }
   }
   ```

   Or use the MCP configuration directly:
   ```bash
   claude --mcp-config /path/to/claude-diary-mcp-server/mcp-config.json
   ```

## Usage

Once configured, you can use these tools in Claude Code:

### Get Today's Entries
```
Use the get_today_diary tool to see what I worked on today
```

### Get Yesterday's Entries  
```
Use the get_yesterday_diary tool to see what I worked on yesterday
```

### Get Specific Date
```
Use the get_diary_by_date tool to see what I worked on 2025-08-25
```

### Get Recent Sessions
```
Use the get_recent_sessions tool to show my last 10 sessions
```

## Database Schema

The server reads from the SQLite database created by the diary hook:
- **sessions**: Main session records with timing
- **accomplishments**: What was accomplished in each session
- **objectives**: Session goals inferred from user prompts
- **issues**: Problems encountered during sessions
- **tool_usage**: Which Claude Code tools were used

## Development

**Build for development**:
```bash
cargo build
```

**Run the server**:
```bash
cargo run
```

**Run tests**:
```bash
cargo test
```

## Performance

This Rust implementation provides:
- ⚡ Fast startup times
- 🔒 Memory safety without garbage collection
- 📦 Single statically-linked binary
- 🚀 Concurrent query processing with async/await

## Output Format

The tools return formatted markdown with:
- Session timestamps and durations
- Accomplishments grouped by category
- Session objectives
- Issues encountered  
- Tool usage statistics

Perfect for getting context on your recent work or reviewing progress!