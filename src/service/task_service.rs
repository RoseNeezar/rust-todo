use std::sync::Arc;

use crate::repository::task_repository::{Task, TaskRepository};

pub struct TaskService {
    task_repository: Arc<TaskRepository>,
}

impl TaskService {
    pub fn new(task_repository: Arc<TaskRepository>) -> Self {
        Self { task_repository }
    }

    pub async fn create_todo(&self, title: String) -> eyre::Result<Task> {
        let new_task = self.task_repository.create(title).await;
        new_task
    }

    pub async fn get_todo(&self, id: String) -> eyre::Result<Task> {
        let task = self.task_repository.get(id).await;
        task
    }
}
