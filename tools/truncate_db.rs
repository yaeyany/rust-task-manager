use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await?;
    
    sqlx::query!("TRUNCATE TABLE tasks RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;
    
    println!("Tasks table truncated successfully");
    
    Ok(())
}