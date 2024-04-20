use crate::model::model_manager::ModelManager;
use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{FromRow, QueryBuilder};

#[derive(Debug, Clone, Serialize, sqlx::prelude::Type, Deserialize, Type)]
#[sqlx(type_name = "status_enum", rename_all = "lowercase")]
pub enum TaskStatus {
    Undone,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Undone => write!(f, "undone"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TaskEntity {
    id: i32,
    title: String,
    status: TaskStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Type)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: TaskStatus,
}

impl From<TaskEntity> for Task {
    fn from(value: TaskEntity) -> Self {
        Self {
            id: value.id,
            title: value.title,
            status: value.status,
        }
    }
}
#[derive(Clone, Debug)]
pub struct TaskRepository {
    mm: ModelManager,
}

impl TaskRepository {
    pub fn new(mm: ModelManager) -> Self {
        Self { mm }
    }

    pub async fn create(&self, title: String, status: TaskStatus) -> Result<Task> {
        let query = "insert into tasks (title, status) values ($1, $2) returning *";

        let entity = sqlx::query_as::<_, TaskEntity>(query)
            .bind(title)
            .bind(status)
            .fetch_one(self.mm.db())
            .await?;

        Ok(Task::from(entity))
    }

    pub async fn get_task(&self, id: i32) -> Result<Task> {
        let query = "select * from tasks where id = $1";

        let entity = sqlx::query_as::<_, TaskEntity>(query)
            .bind(id)
            .fetch_one(self.mm.db())
            .await?;

        Ok(Task::from(entity))
    }

    pub async fn get_all_task(&self) -> Result<Vec<Task>> {
        let query = "select * from tasks";

        let entities = sqlx::query_as::<_, TaskEntity>(query)
            .fetch_all(self.mm.db())
            .await?;

        let result = entities.into_iter().map(Task::from).collect();

        Ok(result)
    }

    pub async fn get_tasks(&self, page_size: i32, page_number: i32) -> Result<Vec<Task>> {
        let offset = (page_number - 1) * page_size;

        let query = " SELECT id, title, status, created_at, updated_at
        FROM tasks
        ORDER BY id
        LIMIT $1 OFFSET $2";

        let entities = sqlx::query_as::<_, TaskEntity>(query)
            .bind(page_size)
            .bind(offset)
            .fetch_all(self.mm.db())
            .await?;

        let result = entities.into_iter().map(Task::from).collect();

        Ok(result)
    }

    pub async fn update_task(
        &self,
        id: i32,
        title: Option<&str>,
        status: Option<TaskStatus>,
    ) -> Result<Task> {
        println!("id={},title={:?},status={:?}", id, title, status);
        let mut query = QueryBuilder::new("UPDATE tasks SET ");
        // Conditionally add title and status updates:
        if let Some(title) = title {
            query.push("title = ");
            query.push_bind(title.to_string());
            query.push(",");
        }
        if let Some(status) = status {
            query.push("status = ");
            query.push_bind(status);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);
        query.push(" RETURNING *");

        let entities = query
            .build_query_as::<Task>()
            .fetch_one(self.mm.db())
            .await?;

        Ok(entities)
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        let query = " DELETE FROM tasks
    WHERE id = $1";

        sqlx::query_as::<_, Task>(query)
            .bind(id)
            .fetch_one(self.mm.db())
            .await?;

        Ok(())
    }
}
