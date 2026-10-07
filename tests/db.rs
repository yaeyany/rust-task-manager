use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rust_task_manager::db::AppDB;

mod tests {
    use super::*;

    #[tokio::test]
    async fn task_add() {
        let pool = test_db().await;
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        assert!(id.into_inner() > 0, "Failed");
    }

    #[tokio::test]
    async fn task_patch() {
        let pool = test_db().await;
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.patch_task(id, "title_edit".try_into().unwrap(), None, "low".try_into().unwrap(), "new".try_into().unwrap()).await.unwrap();
    }

    #[tokio::test]
    async fn task_delete() {
        let pool = test_db().await; 
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.delete_task(id).await.unwrap();
    }

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

    #[tokio::test]
    async fn user_patch() {
        let pool = test_db().await;  
        let id = pool.add_user(format!("User_test_{}", rand::random_range(1..1000)).try_into().unwrap(), "password".try_into().unwrap()).await.unwrap();
        pool.patch_user_name(id, format!("User_test_patched{}", rand::random_range(1..1000)).try_into().unwrap()).await.unwrap();
    }

    #[tokio::test]
    async fn user_delete() {
        let pool = test_db().await;  
        let id = pool.add_user(format!("User_test_delete_{}", rand::random_range(1..1000)).try_into().unwrap(), "password".try_into().unwrap()).await.unwrap();
        pool.delete_user(id).await.unwrap();
    }

    async fn test_db() -> AppDB {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        AppDB::new(&database_url).await.unwrap()
    }
}
