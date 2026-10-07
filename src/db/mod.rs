use sqlx::PgPool;

// DB Struct ──────────────────────────────────────────────────
#[derive(Clone)]
pub struct AppDB {
    database: PgPool
}

// DB methods ──────────────────────────────────────────────────
impl AppDB {

    // Initialize new AppDB with a given database URL ──────────────────────────────────────────────────
    pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        let db = AppDB { 
            database: sqlx::PgPool::connect(database_url).await?,
        };
        Ok(db)
    }

}

// Database modules ──────────────────────────────────────────────────
pub mod tasks;
pub mod users;