// Copy paste example for comments ──────────────────────────────────────────────────
//  ──────────────────────────────────────────────────

use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::db::AppDB;

mod tasks;
mod db;
mod errors;
mod helpers;
mod handlers;
mod router;
mod users;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = AppDB::new(&database_url).await?;    
    
    // Call your router function and pass the database pool here
    let app = router::router(pool);
    
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