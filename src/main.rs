use crate::{
    model::ModelManager, repository::task_repository::TaskRepository,
    service::task_service::TaskService,
};
use config::config::Config;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod model;
mod repository;
mod routes;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();

    let mm = ModelManager::new(&config).await.unwrap();

    let task_repository = Arc::new(TaskRepository::new(mm.clone()));

    let task_service = Arc::new(TaskService::new(task_repository));

    let routes = routes::new(task_service.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
