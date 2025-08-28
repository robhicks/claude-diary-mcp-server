use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDate, Local};
use rmcp::{
    ErrorData as McpError, 
    ServiceExt, 
    model::*, 
    tool, 
    tool_router, 
    tool_handler,
    transport::stdio,
    ServerHandler,
};
use serde_json::{Value as JsonValue};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DiarySession {
    id: i64,
    start_time: String,
    end_time: Option<String>,
    total_duration_ms: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Accomplishment {
    id: i64,
    session_id: i64,
    category: String,
    description: String,
    duration_ms: Option<i64>,
}

#[derive(Clone)]
pub struct DiaryMCPServer {
    db: Arc<tokio::sync::Mutex<Connection>>,
    tool_router: rmcp::handler::server::router::tool::ToolRouter<Self>,
}

#[tool_router]
impl DiaryMCPServer {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not find home directory"))?;
        
        let diary_path = home_dir
            .join(".claude")
            .join("diaries")
            .join("diary.db");

        let db = Connection::open(&diary_path)?;
        
        Ok(DiaryMCPServer { 
            db: Arc::new(tokio::sync::Mutex::new(db)),
            tool_router: Self::tool_router(),
        })
    }

    #[tool(description = "Get diary entries for today")]
    async fn get_today_diary(&self) -> Result<CallToolResult, McpError> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.get_diary_entries(&today).await
    }

    #[tool(description = "Get diary entries for yesterday")]
    async fn get_yesterday_diary(&self) -> Result<CallToolResult, McpError> {
        let yesterday = Local::now() - chrono::Duration::days(1);
        let yesterday_str = yesterday.format("%Y-%m-%d").to_string();
        self.get_diary_entries(&yesterday_str).await
    }

    async fn get_diary_entries(&self, date: &str) -> Result<CallToolResult, McpError> {
        // Validate date format
        if NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
            return Err(McpError::new(
                ErrorCode(-32602), 
                "Invalid date format".to_string(), 
                Some(JsonValue::String("Date must be in YYYY-MM-DD format".to_string()))
            ));
        }

        let db = self.db.lock().await;
        
        // Get sessions for the specified date
        let mut stmt = db.prepare(
            "SELECT id, start_time, end_time, total_duration_ms
             FROM sessions
             WHERE DATE(start_time) = ?
             ORDER BY start_time DESC"
        ).map_err(|e| McpError::new(ErrorCode(-32603), "Database error".to_string(), Some(JsonValue::String(e.to_string()))))?;

        let sessions: Vec<DiarySession> = stmt.query_map(params![date], |row| {
            Ok(DiarySession {
                id: row.get(0)?,
                start_time: row.get(1)?,
                end_time: row.get(2)?,
                total_duration_ms: row.get(3)?,
            })
        }).map_err(|e| McpError::new(ErrorCode(-32603), "Database query error".to_string(), Some(JsonValue::String(e.to_string()))))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| McpError::new(ErrorCode(-32603), "Database result error".to_string(), Some(JsonValue::String(e.to_string()))))?;

        if sessions.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                format!("No diary entries found for {}", date)
            )]));
        }

        let mut output = format!("# Diary Entries for {}\n\n", date);

        for session in sessions {
            let start_time = DateTime::parse_from_rfc3339(&session.start_time)
                .or_else(|_| DateTime::parse_from_str(&session.start_time, "%Y-%m-%d %H:%M:%S"))
                .map_err(|e| McpError::new(ErrorCode(-32603), "DateTime parse error".to_string(), Some(JsonValue::String(e.to_string()))))?;
            let duration_mins = session.total_duration_ms / 60000;
            let duration_display = if duration_mins > 0 {
                format!("~{} minutes", duration_mins)
            } else {
                "< 1 minute".to_string()
            };

            output.push_str(&format!(
                "## Session {} - {}\n\n",
                start_time.format("%H:%M:%S"),
                duration_display
            ));

            // Get accomplishments for this session
            let mut stmt = db.prepare(
                "SELECT category, description, duration_ms
                 FROM accomplishments
                 WHERE session_id = ?
                 ORDER BY id"
            ).map_err(|e| McpError::new(ErrorCode(-32603), "Database error".to_string(), Some(JsonValue::String(e.to_string()))))?;

            let accomplishments: Vec<Accomplishment> = stmt.query_map(params![session.id], |row| {
                Ok(Accomplishment {
                    id: 0,
                    session_id: session.id,
                    category: row.get(0)?,
                    description: row.get(1)?,
                    duration_ms: row.get(2)?,
                })
            }).map_err(|e| McpError::new(ErrorCode(-32603), "Database query error".to_string(), Some(JsonValue::String(e.to_string()))))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| McpError::new(ErrorCode(-32603), "Database result error".to_string(), Some(JsonValue::String(e.to_string()))))?;

            if !accomplishments.is_empty() {
                output.push_str("### ✅ **Accomplishments**\n\n");
                
                // Group by category
                let mut categories: HashMap<String, Vec<&Accomplishment>> = HashMap::new();
                for acc in &accomplishments {
                    categories.entry(acc.category.clone()).or_default().push(acc);
                }

                for (category, accs) in categories {
                    output.push_str(&format!("#### **{}**\n", category));
                    for acc in accs {
                        let duration_str = acc.duration_ms
                            .map(|d| format!(" _({}ms)_", d))
                            .unwrap_or_default();
                        output.push_str(&format!("- **{}**{}\n", acc.description, duration_str));
                    }
                    output.push('\n');
                }
            }

            output.push_str("---\n\n");
        }

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }
}

// Implement the server handler
#[tool_handler]
impl ServerHandler for DiaryMCPServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            server_info: Implementation {
                name: "Claude Diary MCP Server".into(),
                version: "0.1.0".into(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { 
                    list_changed: None 
                }),
                ..Default::default()
            },
            instructions: Some("A diary server that provides access to Claude's diary entries stored in a SQLite database".into()),
        }
    }
}

// Run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the server with STDIO transport
    let diary_server = DiaryMCPServer::new()?;
    
    println!("Starting Claude Diary MCP Server...");
    println!("Available tools:");
    println!("  ✓ get_today_diary: Get diary entries for today");
    println!("  ✓ get_yesterday_diary: Get diary entries for yesterday");  
    
    let service = diary_server.serve(stdio()).await.inspect_err(|e| {
        eprintln!("Error starting server: {}", e);
    })?;
    
    println!("Server started successfully. Waiting for connections...");
    service.waiting().await?;

    Ok(())
}