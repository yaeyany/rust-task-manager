use crate::{db::AppDB, tasks::*};

// Tasks methods for the database ──────────────────────────────────────────────────
impl AppDB {

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
