use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::model::model_manager::ModelManager;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TaskEntity {
    id: i32,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
}

impl From<TaskEntity> for Task {
    fn from(value: TaskEntity) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}

pub struct TaskRepository {
    mm: ModelManager,
}

impl TaskRepository {
    pub fn new(mm: ModelManager) -> Self {
        Self { mm }
    }

    pub async fn create(&self, title: String) -> Result<Task> {
        let query =
            "INSERT INTO tasks (title, created_at, updated_at) values ($1, $2, $3) returning *";

        let entity = sqlx::query_as::<_, TaskEntity>(query)
            .bind(title)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(self.mm.db())
            .await?;

        Ok(Task::from(entity))
    }

    pub async fn get(&self, id: String) -> Result<Task> {
        let query = "select * from tasks where id = $1";

        let entity = sqlx::query_as::<_, TaskEntity>(query)
            .bind(id.parse::<i32>().unwrap())
            .fetch_one(self.mm.db())
            .await?;

        Ok(Task::from(entity))
    }
}
