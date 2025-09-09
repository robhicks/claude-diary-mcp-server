# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **Rust MCP (Model Context Protocol) server** that provides tools for querying Claude diary entries stored in a SQLite database. The server exposes tools that allow Claude to retrieve diary session data, accomplishments, and work history.

## Development Commands

**Build for development:**
```bash
cargo build
```

**Build for release:**
```bash
cargo build --release
```

**Run the server:**
```bash
cargo run
```

**Run tests:**
```bash
cargo test
```

**Quick setup and configuration:**
```bash
./configure-claude.sh
```

## Architecture

### Core Components

- **DiaryMCPServer** (`src/main.rs:37-176`): Main server struct implementing MCP tools
- **Database Layer**: SQLite integration using `rusqlite` for querying diary data
- **MCP Tools**: Four main tools exposed via the `rmcp` framework:
  - `get_today_diary`: Retrieves today's entries
  - `get_yesterday_diary`: Retrieves yesterday's entries  
  - `get_diary_by_date`: Retrieves entries for specific date (YYYY-MM-DD)
  - `get_recent_sessions`: Retrieves recent sessions (configurable limit)

### Database Schema

The server reads from a SQLite database at `~/.claude/diary.db` (with automatic migration from `~/.claude/diaries/diary.db` if needed) with these tables:
- **sessions**: Main session records (id, start_time, end_time, total_duration_ms)
- **accomplishments**: What was accomplished (category, description, duration_ms)
- **objectives**: Session goals inferred from user prompts
- **issues**: Problems encountered during sessions
- **tool_usage**: Which Claude Code tools were used

### Key Dependencies

- `rmcp 0.6`: MCP framework with server, transport-io, and macros features
- `rusqlite 0.32`: SQLite database access with bundled feature
- `tokio 1.0`: Async runtime with full features
- `chrono 0.4`: Date/time handling with serde support

## MCP Configuration

The server is designed to be configured as an MCP server in Claude Code:

**Manual configuration file location:**
```
~/.claude/mcp/diary-server.json
```

**Launch with MCP config:**
```bash
claude --mcp-config ~/.claude/mcp/diary-server.json
```

## File Structure

- `src/main.rs`: Complete server implementation with all MCP tools
- `configure-claude.sh`: Setup script that builds and configures the server
- `mcp-config.json`: Example MCP configuration
- `examples/explore_api.rs`: API exploration example
- `Cargo.toml`: Rust dependencies and project metadata

## Development Notes

- The database path is `~/.claude/diary.db` with automatic migration from the legacy path `~/.claude/diaries/diary.db`
- All tools return formatted markdown output with session details
- Error handling uses MCP error codes (-32602 for invalid params, -32603 for internal errors)
- The server uses async/await throughout for concurrent processing
- Date validation ensures YYYY-MM-DD format for date-based queries

## Build Artifacts

- Development binary: `target/debug/claude-diary-mcp-server`
- Release binary: `target/release/claude-diary-mcp-server` (used by configuration)