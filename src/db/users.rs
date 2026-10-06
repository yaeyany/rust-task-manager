use crate::{db::AppDB, users::{User, UserId, UserName}};

// Users methods for the database ──────────────────────────────────────────────────
impl AppDB {

    // Add a user to the database ──────────────────────────────────────────────────
    pub async fn add_user(
        &self,
        name: UserName,
    ) -> Result<UserId, anyhow::Error> {
        let name = name.into_inner();

        let query = sqlx::query!(
            "INSERT INTO users (username)
            VALUES ($1)
            RETURNING id",
            name
        )
        .fetch_one(&self.database)
        .await?;

        Ok(UserId::try_from(query.id)?)
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
    pub async fn patch_user(
        &self,
        id: UserId,
        name: UserName
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