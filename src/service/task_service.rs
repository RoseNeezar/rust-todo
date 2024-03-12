use std::sync::Arc;

use crate::repository::task_repository::{Task, TaskRepository, TaskStatus};
#[derive(Clone, Debug)]
pub struct TaskService {
    task_repository: Arc<TaskRepository>,
}

impl TaskService {
    pub fn new(task_repository: Arc<TaskRepository>) -> Self {
        Self { task_repository }
    }

    pub async fn create_todo(&self, title: String, status: TaskStatus) -> eyre::Result<Task> {
        let new_task = self.task_repository.create(title, status).await;
        new_task
    }

    pub async fn get_todo(&self, id: i32) -> eyre::Result<Task> {
        let task = self.task_repository.get_task(id).await;
        task
    }

    pub async fn get_all_todos(&self) -> eyre::Result<Vec<Task>> {
        let tasks = self.task_repository.get_all_task().await;
        tasks
    }

    pub async fn get_todos(&self, page_size: i32, page_number: i32) -> eyre::Result<Vec<Task>> {
        let tasks = self.task_repository.get_tasks(page_size, page_number).await;
        tasks
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: Option<&str>,
        status: Option<TaskStatus>,
    ) -> eyre::Result<Task> {
        let tasks = self.task_repository.update_task(id, title, status).await;
        tasks
    }

    pub async fn delete_todo(&self, id: i32) -> eyre::Result<bool> {
        let _ = self.task_repository.delete(id).await;
        eyre::Ok(true)
    }
}
