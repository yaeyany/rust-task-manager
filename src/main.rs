use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::db::AppDB;

// Module declarations ──────────────────────────────────────────────────
mod tasks;
mod db;
mod errors;
mod helpers;
mod handlers;
mod router;
mod users;

// Main entry point ──────────────────────────────────────────────────
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Load environment variables ──────────────────────────────────────────────────
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = AppDB::new(&database_url).await?;    
    
    // Call your router function and pass the database pool here
    let app = router::router(pool);
    
    // Bind to address and start server ──────────────────────────────────────────────────
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Server running on http://127.0.0.1:3000/task/create");

    let server = tokio::spawn(async move {
        axum::serve(
            listener, 
            app.into_make_service()
        )
        .await.unwrap();
    });

    server.await?;
    Ok(())
}
