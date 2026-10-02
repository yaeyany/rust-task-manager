use anyhow::Ok;
use sqlx::PgPool;

use crate::tasks::*;

// DB Struct ──────────────────────────────────────────────────
#[derive(Clone)]
pub struct TasksDB {
    database: PgPool
}

// DB methods ──────────────────────────────────────────────────
impl TasksDB {

    // Initialize new TaskDB with a given database URL ──────────────────────────────────────────────────
    pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        let db = TasksDB { 
            database: sqlx::PgPool::connect(database_url).await?,
        };
        Ok(db)
    }

    // Add a task to the database ──────────────────────────────────────────────────
    pub async fn add_task(
        &self,
        title: TaskTitle,
        description: Option<TaskDescription>,
    ) -> Result<TaskId, anyhow::Error> {
        let title = title.into_inner();
        let description = description.map(TaskDescription::into_inner);

        let query = sqlx::query!(
            "INSERT INTO tasks (title, description, priority, status)
            VALUES ($1, $2, 'medium', 'new')
            RETURNING id",
            title,
            description
        )
        .fetch_one(&self.database)
        .await?;

        Ok(TaskId::try_from(query.id)?)
    }

    // Retrieve tasks ──────────────────────────────────────────────────
    pub async fn get_tasks(
        &self,
        limit: i64,
    ) -> Result<Vec<Task>, anyhow::Error> {
        let rows = sqlx::query!(
            "SELECT * FROM tasks ORDER BY id LIMIT $1",
            limit
        )
        .fetch_all(&self.database)
        .await?;

        let tasks = rows
            .into_iter()
            .map(|row| {
                Ok(Task::from_parts(
                    row.id.try_into()?,
                    row.title.try_into()?,
                    row.description
                        .map(|d| d.try_into())
                        .transpose()?,
                    row.priority.try_into()?,
                    row.status.try_into()?,
                ))
            })
            .collect::<Result<Vec<Task>, anyhow::Error>>()?;

        Ok(tasks)
    }

    // Patch a task ──────────────────────────────────────────────────
    pub async fn patch_task(
        &self,
        id: TaskId,
        title: TaskTitle,
        description: Option<TaskDescription>,
        priority: TaskPriority,
        status: TaskStatus,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            UPDATE tasks
            SET
                title = $1,
                description = $2,
                priority = $3,
                status = $4
            WHERE id = $5
            "#,
            title.into_inner(),
            description.map(TaskDescription::into_inner),
            priority.into_inner(),
            status.into_inner(),
            id.into_inner(),
        )
        .execute(&self.database)
        .await?;

        Ok(())
    }

    // Delete a task ──────────────────────────────────────────────────
    pub async fn delete_task(
    &self,
    id: TaskId,
    ) -> Result<bool, anyhow::Error> {
        let deleted = sqlx::query_scalar!(
            r#"
            DELETE FROM tasks
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

#[cfg(test)]
mod tests {
    use crate::database::*;

    // Test add_task with valid data ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_add() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = TasksDB::new(&database_url).await.unwrap();   
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        assert!(id.into_inner() > 0, "Failed");
    }

    // Test patch_task ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_patch() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = TasksDB::new(&database_url).await.unwrap();   
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.patch_task(id, "title_edit".try_into().unwrap(), None, "low".try_into().unwrap(), "new".try_into().unwrap()).await.unwrap();
    }

    // Test delete_task ──────────────────────────────────────────────────
    #[tokio::test]
    async fn task_delete() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = TasksDB::new(&database_url).await.unwrap();   
        let id = pool.add_task("title".try_into().unwrap(), None).await.unwrap();
        pool.delete_task(id).await.unwrap();
    }
}