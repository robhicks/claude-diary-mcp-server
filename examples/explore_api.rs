use rmcp::{handler, model};

#[tokio::main]
async fn main() {
    println!("Exploring rmcp 0.6 API...");
    
    // Try to call some basic function in handler::server  
    handler::server::serve().await;
    
    println!("Found API!");
}