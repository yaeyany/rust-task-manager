use argon2::{Argon2, PasswordHasher};

use crate::{db::AppDB, users::*};

// Users methods for the database ──────────────────────────────────────────────────
impl AppDB {

    // Add a user to the database ──────────────────────────────────────────────────
    pub async fn add_user(
        &self,
        name: UserName,
        password: UserPassword,
    ) -> Result<UserId, anyhow::Error> {
        let argon2 = Argon2::default();
        let name = name.into_inner();
        let password_hash = argon2.hash_password(password.into_inner().as_bytes())?.to_string();

        let query = sqlx::query!(
            "INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id",
            name,
            password_hash
        )
        .fetch_one(&self.database)
        .await?;

        Ok(UserId::try_from(query.id)?)
    }

    // Retrieve password hash for authentication ──────────────────────────────────────────────────
    pub async fn get_user_password_hash(
    &self,
    username: &str,
    ) -> Result<Option<String>, anyhow::Error> {
        let row = sqlx::query!(
            "SELECT password_hash FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.database)
        .await?;

        Ok(row.map(|row| row.password_hash))
    }

    // Retrieve users ──────────────────────────────────────────────────
    pub async fn get_users(
        &self,
        limit: i64,
    ) -> Result<Vec<User>, anyhow::Error> {
        let rows = sqlx::query!(
            "SELECT * FROM users ORDER BY id LIMIT $1",
            limit
        )
        .fetch_all(&self.database)
        .await?;

        let users = rows
            .into_iter()
            .map(|row| {
                Ok(User::from_parts(
                    row.id.try_into()?,
                    row.username.try_into()?,
                    row.created_at
                ))
            })
            .collect::<Result<Vec<User>, anyhow::Error>>()?;

        Ok(users)
    }

    // Patch a user ──────────────────────────────────────────────────
    pub async fn patch_user_name(
        &self,
        id: UserId,
        name: UserName,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET
                username = $1
            WHERE id = $2
            "#,
            name.into_inner(),
            id.into_inner(),
        )
        .execute(&self.database)
        .await?;

        Ok(())
    }

    // Delete a user ──────────────────────────────────────────────────
    pub async fn delete_user(
    &self,
    id: UserId,
    ) -> Result<bool, anyhow::Error> {
        let deleted = sqlx::query_scalar!(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING id
            "#,
            id.into_inner(),
        )
        .fetch_optional(&self.database)
        .await?;

        Ok(deleted.is_some())
    }
}
