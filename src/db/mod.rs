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

#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
    use crate::db::*;

    async fn test_db() -> AppDB {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        AppDB::new(&database_url).await.unwrap()
    }

    // Test add_task with valid data ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_add() {
        let pool = test_db().await;
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        assert!(id.into_inner() > 0, "Failed");
    }

    // Test patch_task ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_patch() {
        let pool = test_db().await;
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.patch_task(id, "title_edit".try_into().unwrap(), None, "low".try_into().unwrap(), "new".try_into().unwrap()).await.unwrap();
    }

    // Test delete_task ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_delete() {
        let pool = test_db().await; 
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.delete_task(id).await.unwrap();
    }

    // Test add_user with valid data ──────────────────────────────────────────────────
    #[tokio::test]
    async fn user_add() {
        let pool = test_db().await; 
        let argon2 = Argon2::default();
        let password = "password";
        let password_hash = argon2.hash_password(password.as_bytes()).unwrap().to_string();
        let id = pool.add_user(format!("User_test_{}", rand::random_range(1..1000)).try_into().unwrap(), password_hash.clone().try_into().unwrap()).await.unwrap();
        assert!(id.into_inner() > 0, "Failed");
        let parsed_hash = PasswordHash::new(&password_hash).unwrap();
        assert!(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok());
    }

    // Test patch_user_name ──────────────────────────────────────────────────
    #[tokio::test]
    async fn user_patch() {
        let pool = test_db().await;  
        let id = pool.add_user(format!("User_test_{}", rand::random_range(1..1000)).try_into().unwrap(), "password".try_into().unwrap()).await.unwrap();
        pool.patch_user_name(id, format!("User_test_patched{}", rand::random_range(1..1000)).try_into().unwrap()).await.unwrap();
    }

    // Test delete_user ──────────────────────────────────────────────────
    #[tokio::test]
    async fn user_delete() {
        let pool = test_db().await;  
        let id = pool.add_user(format!("User_test_delete_{}", rand::random_range(1..1000)).try_into().unwrap(), "password".try_into().unwrap()).await.unwrap();
        pool.delete_user(id).await.unwrap();
    }
}
